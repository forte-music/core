mod targets;
mod transcode;
mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
        }

        errors {
            NoDiskCacheEntryError {
                description("Item not found in disk cache after cache was populated. It was either removed, or failed to add. Try increasing the cache size.")
            }
        }
    }
}

pub use self::targets::TranscodeTarget;
pub use self::transcode::{TranscodeRequest, Transcoder};
