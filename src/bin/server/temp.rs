// TODO: replace with tempfile

use rand::distributions;
use rand::Rng;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

/// Creates files with randomly generated file names in a temporary directory.
#[derive(Clone)]
pub struct TemporaryFiles {
    /// Path where files are stored.
    root: PathBuf,
}

impl TemporaryFiles {
    pub fn new(suffix: &str) -> io::Result<TemporaryFiles> {
        let mut temp_dir = env::temp_dir();
        temp_dir.push(suffix);

        fs::create_dir_all(temp_dir.as_path())?;

        Ok(TemporaryFiles { root: temp_dir })
    }

    /// Gets path inside the root directory of a randomly named file.
    pub fn get_file_path(&self) -> PathBuf {
        let mut file_path = self.root.to_path_buf();

        let file_name = rand::thread_rng()
            .sample_iter(&distributions::Alphanumeric)
            .map(char::from)
            .take(10)
            .collect::<String>();

        file_path.push(file_name);

        file_path
    }
}

#[cfg(test)]
mod test {
    use super::TemporaryFiles;

    #[test]
    fn folder_exists() {
        let temp = TemporaryFiles::new("test").unwrap();
        let file = temp.get_file_path();

        assert!(file.parent().unwrap().exists());
    }

    #[test]
    fn file_not_exists() {
        let temp = TemporaryFiles::new("test").unwrap();
        let file = temp.get_file_path();

        assert!(!file.exists())
    }
}
