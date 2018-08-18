use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

use rand;
use rand::distributions;
use rand::Rng;

/// Creates files with randomly generated file names in a temporary directory.
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

    pub fn get_file(&self) -> PathBuf {
        let mut file_path = self.root.to_path_buf();

        let file_name = rand::thread_rng()
            .sample_iter(&distributions::Alphanumeric)
            .take(10)
            .collect::<String>();

        file_path.push(file_name);

        file_path
    }
}
