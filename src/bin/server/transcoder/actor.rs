use futures::Future;

use lru_disk_cache::LruDiskCache;
use lru_disk_cache::ReadSeek;

use server::temp::TemporaryFiles;
use server::transcoder::TranscodeTarget;

use std::io;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;
use std::collections::HashMap;

use actix::prelude::*;

use futures::future;
use futures::future::Shared;

use std::cell::RefCell;
use std::rc::Rc;
use tokio_process::CommandExt;

pub struct Transcoder {
    // Rc<RefCell<T>> combination is used to have multiple owning pointers to the same data with
    // runtime borrow checking. Needed because satisfying the borrow checker is difficult (and less
    // elegant) when working with futures.
    /// Cache of all pre-transcoded items. Stored on disk. Persists across application restarts.
    disk_cache: Rc<RefCell<LruDiskCache>>,

    /// Cache of message keys to futures of things which are being converted now.
    future_cache: Rc<RefCell<HashMap<OsString, Shared<ResponseFuture<(), io::Error>>>>>,
    temp: TemporaryFiles,
}

impl Transcoder {
    pub fn new(cache: LruDiskCache, temp: TemporaryFiles) -> Transcoder {
        Transcoder {
            disk_cache: Rc::new(RefCell::new(cache)),
            future_cache: Rc::new(RefCell::new(HashMap::new())),
            temp,
        }
    }

    /// Transcodes the file requested by the message into a temporary path.
    fn transcode<P: AsRef<Path>>(
        &self,
        msg: &Transcode<P>,
    ) -> Box<Future<Item = (Output, PathBuf), Error = io::Error>> {
        let temporary_file_path = self.temp.get_file();

        // TODO: Configurable FFMpeg Instance
        // TODO: Handle Non Zero Exit Code
        Box::new(
            Command::new("ffmpeg")
                .args(msg.get_ffmpeg_args(&temporary_file_path))
                .output_async()
                .map(move |output| (output, temporary_file_path.to_path_buf())),
        )
    }

    /// Transcodes the file requested by the message, updating the relevant caches.
    fn transcode_and_cache<P: AsRef<Path>>(
        &self,
        msg: &Transcode<P>,
    ) -> Shared<ResponseFuture<(), io::Error>> {
        let key = msg.to_key();
        let future_map_key = key.clone();

        let cache = self.disk_cache.clone();
        let future_cache = self.future_cache.clone();

        let transcode_future = self
            .transcode(&msg)
            .map(move |(_output, temporary_file_path)| {
                cache
                    .try_borrow_mut()
                    .expect("// TODO: Remove Me")
                    .insert_file(&future_map_key, temporary_file_path)
                    .expect("didn't fail");

                future_cache
                    .try_borrow_mut()
                    .expect("// TODO: Remove Me")
                    .remove(&future_map_key);

                ()
            });

        let boxed_transcode_future: Box<Future<Item = (), Error = io::Error>> =
            Box::new(transcode_future);

        let shared_future = boxed_transcode_future.shared();

        self.future_cache
            .try_borrow_mut()
            .expect("// TODO: Remove Me")
            .insert(key.clone(), shared_future.clone());

        shared_future
    }

    fn disk_cache_has(&self, key: &OsStr) -> bool {
        self.disk_cache
            .try_borrow()
            .expect("// TODO: No Error")
            .contains_key(key)
    }

    fn future_cache_has(&self, key: &OsStr) -> bool {
        self.future_cache
            .try_borrow()
            .expect("// TODO: No Error")
            .contains_key(key)
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
        let key = msg.to_key();

        if self.disk_cache_has(&key) {
            let mut lru_disk_cache = self
                .disk_cache
                .try_borrow_mut()
                .expect("// TODO: Remove Me");
            let file = lru_disk_cache.get(key).expect("didn't fail");

            return Box::new(future::ok(file));
        }

        let disk_cache = self.disk_cache.clone();
        let local_key = key.clone();
        let get_from_disk_cache = move || {
            let mut lru_disk_cache = disk_cache.try_borrow_mut().expect("// TODO: Complete");
            let file = lru_disk_cache.get(local_key).expect("// TODO: Complete");

            file
        };

        if self.future_cache_has(&key) {
            return Box::new(
                self.future_cache
                    .try_borrow()
                    .expect("// TODO: Remove Me")
                    .get(&key)
                    .expect("// TODO didn't fail")
                    .clone()
                    .map_err(|_| io::ErrorKind::AlreadyExists.into())
                    .map(|_| get_from_disk_cache()),
            );
        }

        Box::new(
            self.transcode_and_cache(&msg)
                .map_err(|_| io::ErrorKind::AlreadyExists.into())
                .map(|_| get_from_disk_cache()),
        )
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

    fn get_ffmpeg_args<'a>(&'a self, output_path: &'a Path) -> Vec<&'a OsStr> {
        self.target.get_ffmpeg_args(self.path.as_ref(), output_path)
    }
}
