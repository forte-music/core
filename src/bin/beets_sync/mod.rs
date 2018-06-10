use forte_core::context;
use std::fs::File;
use std::path::Path;

error_chain!{}

pub fn sync<P: AsRef<Path>, F: AsRef<File>>(
    pool: context::Pool,
    exported_file: F,
    artwork_directory: P,
) -> Result<()> {
    let exported_file = exported_file.as_ref();
    let artwork_directory = artwork_directory.as_ref();

    unimplemented!()
}
