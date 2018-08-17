use futures::Future;

use lru_disk_cache::LruDiskCache;
use lru_disk_cache::ReadSeek;

use server::temp::TemporaryFiles;

use std::ffi::OsString;
use std::path::Path;
use std::process::Command;

use actix::prelude::*;

use futures;

use server::transcoder::TranscodeTarget;
use std::io;

pub struct Transcoder {
    cache: LruDiskCache,
    temp: TemporaryFiles,
}

impl Transcoder {
    pub fn new(cache: LruDiskCache, temp: TemporaryFiles) -> Transcoder {
        Transcoder { cache, temp }
    }
}

impl Actor for Transcoder {
    type Context = Context<Self>;
}

impl<P> Handler<Transcode<P>> for Transcoder
where
    P: AsRef<Path>,
{
    type Result = io::Result<Box<ReadSeek>>;

    fn handle(&mut self, msg: Transcode<P>, _ctx: &mut Context<Self>) -> Self::Result {
        let key = msg.to_key();
        if !self.cache.contains_key(&key) {
            let temporary_file_path = self.temp.get_file();

            // TODO: Configurable FFMpeg Instance
            // TODO: Async Command
            Command::new("ffmpeg")
                .args(
                    msg.target
                        .get_ffmpeg_args(msg.path.as_ref(), &temporary_file_path),
                )
                // TODO: Remove Expect
                // TODO: Log Standard Output
                .output().expect("didn't fail");

            // TODO: Remove Expect
            self.cache
                .insert_file(&key, temporary_file_path)
                .expect("didn't fail");
        }

        // TODO: Remove Expect
        Ok(self.cache.get(&key).expect("didn't fail"))
    }
}

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
    type Result = io::Result<Box<ReadSeek>>;
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
