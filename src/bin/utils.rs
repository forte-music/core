use std::fs;
use std::fs::File;
use std::io;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
pub struct FileWrapper {
    inner: File,
}

impl Deref for FileWrapper {
    type Target = File;

    fn deref(&self) -> &File {
        &self.inner
    }
}

impl AsRef<File> for FileWrapper {
    fn as_ref(&self) -> &File {
        self.deref()
    }
}

impl FromStr for FileWrapper {
    type Err = io::Error;

    fn from_str(path: &str) -> Result<Self, Self::Err> {
        let file = File::open(path)?;

        Ok(FileWrapper { inner: file })
    }
}

pub fn get_and_make_artwork_dir<P: AsRef<Path>>(app_dir: P) -> Result<PathBuf, io::Error> {
    let mut artwork_directory = app_dir.as_ref().to_owned();
    artwork_directory.push("artwork");
    fs::create_dir_all(&artwork_directory)?;

    Ok(artwork_directory)
}
