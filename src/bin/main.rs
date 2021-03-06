#[macro_use]
extern crate diesel_migrations;

#[cfg(feature = "embed_web")]
#[macro_use]
extern crate rust_embed;

pub mod server;
pub mod sync;

use crate::server::temp::TemporaryFiles;
use app_dirs::app_root;
use app_dirs::AppDataType;
use app_dirs::AppInfo;
use forte_core::context;
use lru_disk_cache::LruDiskCache;
use std::ops::Deref;
use std::path::PathBuf;
use std::{fs, io};
use structopt::StructOpt;

embed_migrations!("./migrations");

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error(transparent)]
    R2d2(#[from] r2d2::Error),

    #[error(transparent)]
    AppDirs(#[from] app_dirs::AppDirsError),

    #[error(transparent)]
    DieselMigration(#[from] diesel_migrations::RunMigrationsError),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    LruDiskCache(#[from] lru_disk_cache::Error),

    #[error(transparent)]
    Sync(#[from] sync::Error),
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(flatten)]
    common: CommonFlags,

    #[structopt(flatten)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "serve")]
    Serve {
        /// The address at which to serve the backend.
        #[structopt(long = "host", default_value = "0.0.0.0:8080")]
        host: String,
    },

    #[structopt(name = "sync")]
    Sync {
        /// The directory to sync.
        #[structopt(name = "sync-dir", parse(from_os_str))]
        directory: PathBuf,
    },
}

#[derive(StructOpt, Debug)]
struct CommonFlags {
    /// The path which holds the application data (extracted album artwork, database). By default,
    /// this is an OS specific application directory.
    #[structopt(long = "app-dir", parse(from_os_str))]
    app_dir: Option<PathBuf>,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);

        // Go down the chain of errors
        let mut error: &dyn std::error::Error = &err;
        while let Some(source) = error.source() {
            eprintln!("Caused by: {}", source);
            error = source;
        }

        std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let opt: Opt = Opt::from_args();
    let app_dir: PathBuf = opt.common.app_dir.map_or_else(
        || {
            app_root(
                AppDataType::UserData,
                &AppInfo {
                    name: "forte",
                    author: "forte-music",
                },
            )
        },
        Ok,
    )?;

    // Get Connection Pool
    let mut database_path = app_dir.clone();
    database_path.push("db.sqlite");

    let database_url = database_path.to_string_lossy();
    let pool = context::init_pool(&database_url)?;

    // Initialize Database
    embedded_migrations::run(pool.get()?.deref())?;

    match opt.command {
        Command::Serve { host } => {
            let transcode_cache = make_transcode_cache(app_dir)?;
            let temporary_files = TemporaryFiles::new("forte")?;

            server::serve(pool, &host, transcode_cache, temporary_files)?;
        }
        Command::Sync { directory } => {
            let mut artwork_directory = app_dir;
            artwork_directory.push("artwork");
            fs::create_dir_all(&artwork_directory)?;

            sync::sync(pool, &directory, &artwork_directory)?;
        }
    }

    Ok(())
}

fn make_transcode_cache(app_dir: PathBuf) -> Result<LruDiskCache, Error> {
    let mut transcode_cache_path = app_dir;
    transcode_cache_path.push("transcode-cache");

    let transcode_cache_size = 100_000_000_u64; // 100 MB

    Ok(LruDiskCache::new(
        transcode_cache_path,
        transcode_cache_size,
    )?)
}
