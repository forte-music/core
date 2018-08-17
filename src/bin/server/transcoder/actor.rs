use futures::Future;

use lru_disk_cache::LruDiskCache;
use lru_disk_cache::ReadSeek;

use server::temp::TemporaryFiles;
use server::transcoder::TranscodeTarget;

use std::collections::HashMap;
use std::ffi::OsString;
use std::io;
use std::path::Path;
use std::process::Command;

use actix::prelude::*;

use futures;
use futures::future::Shared;

use std::cell::RefCell;
use std::rc::Rc;
use tokio_process::CommandExt;

pub struct Transcoder {
    // Rc<RefCell<T>> combination is used to have multiple owning pointers to the same data with
    // runtime borrow checking. Needed because satisfying the borrow checker is difficult (and less
    // elegant) when working with futures.
    /// Cache of all pre-transcoded items. Persists across application restarts.
    cache: Rc<RefCell<LruDiskCache>>,

    /// Cache of message keys to futures of things which are being converted now.
    future_cache: Rc<RefCell<HashMap<OsString, Shared<ResponseFuture<(), io::Error>>>>>,
    temp: TemporaryFiles,
}

impl Transcoder {
    pub fn new(cache: LruDiskCache, temp: TemporaryFiles) -> Transcoder {
        Transcoder {
            cache: Rc::new(RefCell::new(cache)),
            future_cache: Rc::new(RefCell::new(HashMap::new())),
            temp,
        }
    }
}

impl Actor for Transcoder {
    type Context = Context<Self>;
}

impl<P> Handler<Transcode<P>> for Transcoder
where
    P: AsRef<Path>,
{
    type Result = ResponseFuture<Box<ReadSeek>, io::Error>;

    fn handle(&mut self, msg: Transcode<P>, _ctx: &mut Self::Context) -> Self::Result {
        // TODO: borrow -> try_borrow
        let key = msg.to_key();

        // Resolves when the cache has been populated.
        let populate_cache_future: ResponseFuture<(), io::Error> =
            if !self.cache.borrow().contains_key(&key) {
                if !self.future_cache.borrow().contains_key(&key) {
                    let temporary_file_path = self.temp.get_file();
                    let mut future_cache = self.future_cache.clone();
                    let mut cache = self.cache.clone();
                    let local_key = key.clone();

                    // TODO: Configurable FFMpeg Instance
                    // TODO: Async Command
                    let transcoded_future: ResponseFuture<(), io::Error> = Box::new(
                        Command::new("ffmpeg")
                    .args(
                        msg.target
                            .get_ffmpeg_args(msg.path.as_ref(), &temporary_file_path),
                    )
                    // TODO: Remove Expect
                    // TODO: Log Standard Output
                    .output_async()
                    .map(move |_| {
                        cache.borrow_mut()
                            .insert_file(&local_key, temporary_file_path)
                            .expect("didn't fail");

                        future_cache.borrow_mut()
                            .remove(&local_key);

                        ()
                    }),
                    );

                    self.future_cache
                        .borrow_mut()
                        .insert(key.clone(), transcoded_future.shared());
                }

                Box::new(
                    self.future_cache
                        .borrow()
                        .get(&key)
                        .expect("// TODO didn't fail")
                        .clone()
                        .map_err(|_| io::ErrorKind::AlreadyExists.into())
                        .map(|_| ()),
                )
            } else {
                Box::new(futures::future::ok(()))
            };

        let cache = self.cache.clone();
        Box::new(populate_cache_future.map(move |_| {
            let mut lru_disk_cache = cache.borrow_mut();
            let file = lru_disk_cache.get(key).expect("didn't fail");

            file
        }))
    }
}

// TODO: Rename
/// Message sent to the transcoder actor to request a file be transcoded.
pub struct Transcode<P>
where
    P: AsRef<Path>,
{
    /// Path of the input to the transcoding process.
    path: P,

    /// A string which shares the uniqueness of the path. Should maintain:
    /// a.path == b.path iff a.partial_key == b.partial_key.
    /// Used to compute cache key.
    partial_key: String,

    /// Desired quality of the transcoding.
    target: TranscodeTarget,
}

impl<P> Message for Transcode<P>
where
    P: AsRef<Path>,
{
    type Result = Result<Box<ReadSeek>, io::Error>;
}

impl<P> Transcode<P>
where
    P: AsRef<Path>,
{
    pub fn new(path: P, partial_key: String, target: TranscodeTarget) -> Transcode<P> {
        Transcode {
            path,
            partial_key,
            target,
        }
    }

    pub fn to_key(&self) -> OsString {
        (self.partial_key.clone() + &self.target.to_string().to_lowercase()).into()
    }
}
