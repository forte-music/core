mod targets;
mod transcode;
mod errors {
    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error("{0}")]
        Io(String),

        #[error("Item not found in disk cache after cache was populated. It was either removed, or failed to add. Try increasing the cache size.")]
        NoDiskCacheEntryError(lru_disk_cache::Error),
    }
}

pub use self::targets::TranscodeTarget;
pub use self::transcode::{TranscodeRequest, Transcoder};
