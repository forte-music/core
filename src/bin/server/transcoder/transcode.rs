use crate::server::temp::TemporaryFiles;
use crate::server::transcoder::errors;
use crate::server::transcoder::errors::ResultExt;
use crate::server::transcoder::TranscodeTarget;
use futures::future::{BoxFuture, Shared};
use futures::{future, TryFutureExt};
use futures::{Future, FutureExt};
use lru_disk_cache::LruDiskCache;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::process::Output;
use std::sync::{Arc, Mutex};
use tokio::process::Command;

type TranscodeFuture = BoxFuture<'static, Result<(), String>>;

#[derive(Clone)]
pub struct Transcoder {
    /// Cache of all pre-transcoded items. Stored on disk. Persists across application restarts.
    disk_cache: Arc<Mutex<LruDiskCache>>,

    /// Cache of message keys to futures of things which are being converted now.
    future_cache: Arc<Mutex<HashMap<OsString, Shared<TranscodeFuture>>>>,
    temp: TemporaryFiles,
}

impl Transcoder {
    pub fn new(cache: LruDiskCache, temp: TemporaryFiles) -> Transcoder {
        Transcoder {
            disk_cache: Arc::new(Mutex::new(cache)),
            future_cache: Arc::new(Mutex::new(HashMap::new())),
            temp,
        }
    }

    /// Transcode a file or get the result from cache
    pub async fn get_transcoded_file(&self, msg: &TranscodeRequest) -> Result<File, errors::Error> {
        let key = msg.compute_key();

        self.get_cache_populated_future(&msg).await?;

        let file = self
            .disk_cache
            .lock()
            .unwrap()
            .get_file(&key)
            .chain_err(|| errors::ErrorKind::NoDiskCacheEntryError)?;

        Ok(file)
    }

    /// Transcodes the file requested by the message into a temporary path.
    fn transcode(
        &self,
        msg: &TranscodeRequest,
    ) -> impl Future<Output = Result<(Output, PathBuf), String>> {
        let temporary_file_path = self.temp.get_file_path();

        // TODO: Configurable FFMpeg Instance
        // TODO: Handle Non Zero Exit Code
        Command::new("ffmpeg")
            .args(msg.get_ffmpeg_args(&temporary_file_path))
            .output()
            .map_ok(|output| (output, temporary_file_path))
            .map_err(|e| e.to_string())
    }

    /// Transcodes the file requested by the message, updating the relevant caches.
    fn transcode_and_cache(&self, msg: &TranscodeRequest) -> TranscodeFuture {
        let key = msg.compute_key();
        let future_map_key = key.clone();

        let cache = self.disk_cache.clone();
        let future_cache = self.future_cache.clone();

        let transcode_future = self
            .transcode(&msg)
            .map_ok(move |(_output, temporary_file_path)| {
                // TODO: Try Using Failure Errors
                // Result ignored here because if inserting a file fails later,
                // getting it out of the cache will fail also. It's better to
                // start the error here, but forwarding errors from here is
                // difficult because Shared<Future<...>> turns the error into
                // an error reference. error-chain errors can't be cloned.
                let _ = cache
                    .lock()
                    .unwrap()
                    .insert_file(&future_map_key, temporary_file_path);

                future_cache.lock().unwrap().remove(&future_map_key);
            })
            .boxed();

        let shared_future = transcode_future.shared();

        self.future_cache
            .lock()
            .unwrap()
            .insert(key, shared_future.clone());

        shared_future.boxed()
    }

    fn disk_cache_has(&self, key: &OsStr) -> bool {
        self.disk_cache.lock().unwrap().contains_key(key)
    }

    fn future_cache_has(&self, key: &OsStr) -> bool {
        self.future_cache.lock().unwrap().contains_key(key)
    }

    /// Returns a future which resolves when the transcode job associated with the message is
    /// complete and the transcoded file is in the disk_cache.
    fn get_cache_populated_future(&self, msg: &TranscodeRequest) -> TranscodeFuture {
        let key = msg.compute_key();

        if self.disk_cache_has(&key) {
            return future::ok(()).boxed();
        }

        if self.future_cache_has(&key) {
            return self
                .future_cache
                .lock()
                .unwrap()
                .get(&key)
                .unwrap()
                .clone()
                .boxed();
        }

        self.transcode_and_cache(&msg)
    }
}

/// Message sent to the transcoder to request a file be transcoded.
pub struct TranscodeRequest {
    /// Path of the input to the transcoding process.
    path: PathBuf,

    /// A string which shares the uniqueness of the path. Should maintain:
    /// a.path == b.path iff a.partial_key == b.partial_key.
    /// Used to compute cache key.
    partial_key: String,

    /// Desired quality of the transcoding.
    target: TranscodeTarget,
}

impl TranscodeRequest {
    pub fn new(path: PathBuf, partial_key: String, target: TranscodeTarget) -> TranscodeRequest {
        TranscodeRequest {
            path,
            partial_key,
            target,
        }
    }

    pub fn compute_key(&self) -> OsString {
        (self.partial_key.clone() + &self.target.to_string().to_lowercase()).into()
    }

    fn get_ffmpeg_args<'a>(&'a self, output_path: &'a Path) -> Vec<&'a OsStr> {
        self.target.get_ffmpeg_args(&self.path, output_path)
    }
}
