use lru_disk_cache::LruDiskCache;
use lru_disk_cache::ReadSeek;

use server::temp::TemporaryFiles;
use server::transcoder::errors;
use server::transcoder::errors::ResultExt;
use server::transcoder::TranscodeTarget;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;

use actix::prelude::*;

use futures::future;
use futures::future::Shared;
use futures::Future;

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
    fn transcode(
        &self,
        msg: &TranscodeMessage,
    ) -> impl Future<Item = (Output, PathBuf), Error = io::Error> {
        let temporary_file_path = self.temp.get_file_path();

        // TODO: Configurable FFMpeg Instance
        // TODO: Handle Non Zero Exit Code
        Command::new("ffmpeg")
            .args(msg.get_ffmpeg_args(&temporary_file_path))
            .output_async()
            .map(move |output| (output, temporary_file_path.to_path_buf()))
    }

    /// Transcodes the file requested by the message, updating the relevant caches.
    fn transcode_and_cache(&self, msg: &TranscodeMessage) -> ResponseFuture<(), io::Error> {
        let key = msg.compute_key();
        let future_map_key = key.clone();

        let cache = self.disk_cache.clone();
        let future_cache = self.future_cache.clone();

        let transcode_future = self
            .transcode(&msg)
            .map(move |(_output, temporary_file_path)| {
                // TODO: Try Using Failure Errors
                // Result ignored here because if inserting a file fails later,
                // getting it out of the cache will fail also. It's better to
                // start the error here, but forwarding errors from here is
                // difficult because Shared<Future<...>> turns the error into
                // an error reference. error-chain errors can't be cloned.
                let _ = cache
                    .borrow_mut()
                    .insert_file(&future_map_key, temporary_file_path);

                future_cache.borrow_mut().remove(&future_map_key);

                ()
            });

        let boxed_transcode_future: Box<Future<Item = (), Error = io::Error>> =
            Box::new(transcode_future);

        let shared_future = boxed_transcode_future.shared();

        self.future_cache
            .borrow_mut()
            .insert(key.clone(), shared_future.clone());

        Box::new(shared_future.map(|_| ()).map_err(|err| err.kind().into()))
    }

    fn disk_cache_has(&self, key: &OsStr) -> bool {
        self.disk_cache.borrow().contains_key(key)
    }

    fn future_cache_has(&self, key: &OsStr) -> bool {
        self.future_cache.borrow().contains_key(key)
    }

    /// Returns a future which resolves when the transcode job associated with the message is
    /// complete and the transcoded file is in the disk_cache.
    fn get_cache_populated_future(&self, msg: &TranscodeMessage) -> ResponseFuture<(), io::Error> {
        let key = msg.compute_key();

        if self.disk_cache_has(&key) {
            return Box::new(future::ok(()));
        }

        if self.future_cache_has(&key) {
            return Box::new(
                self.future_cache
                    .borrow()
                    .get(&key)
                    .unwrap()
                    .clone()
                    .map(|_| ())
                    .map_err(|err| err.kind().into()),
            );
        }

        Box::new(self.transcode_and_cache(&msg))
    }
}

impl Actor for Transcoder {
    type Context = Context<Self>;
}

impl Handler<TranscodeMessage> for Transcoder {
    type Result = ResponseFuture<Box<ReadSeek>, errors::Error>;

    fn handle(&mut self, msg: TranscodeMessage, _ctx: &mut Self::Context) -> Self::Result {
        let disk_cache_ref = self.disk_cache.clone();
        let key = msg.compute_key();

        Box::new(
            self.get_cache_populated_future(&msg)
                .map_err(errors::Error::from)
                .and_then(move |_| {
                    let file = disk_cache_ref
                        .borrow_mut()
                        .get(&key)
                        .chain_err(|| errors::ErrorKind::NoDiskCacheEntryError)?;

                    Ok(file)
                }),
        )
    }
}

/// Message sent to the transcoder actor to request a file be transcoded.
pub struct TranscodeMessage {
    /// Path of the input to the transcoding process.
    path: PathBuf,

    /// A string which shares the uniqueness of the path. Should maintain:
    /// a.path == b.path iff a.partial_key == b.partial_key.
    /// Used to compute cache key.
    partial_key: String,

    /// Desired quality of the transcoding.
    target: TranscodeTarget,
}

impl Message for TranscodeMessage {
    type Result = errors::Result<Box<ReadSeek>>;
}

impl TranscodeMessage {
    pub fn new(path: PathBuf, partial_key: String, target: TranscodeTarget) -> TranscodeMessage {
        TranscodeMessage {
            path,
            partial_key,
            target,
        }
    }

    pub fn compute_key(&self) -> OsString {
        (self.partial_key.clone() + &self.target.to_string().to_lowercase()).into()
    }

    fn get_ffmpeg_args<'a>(&'a self, output_path: &'a Path) -> Vec<&'a OsStr> {
        self.target.get_ffmpeg_args(self.path.as_ref(), output_path)
    }
}
