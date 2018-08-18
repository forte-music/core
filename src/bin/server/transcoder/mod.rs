mod actor;
mod targets;
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

pub use self::actor::{TranscodeMessage, Transcoder};
pub use self::targets::TranscodeTarget;
