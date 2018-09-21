#[macro_use]
extern crate structopt;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate diesel_migrations;

extern crate app_dirs;
extern crate diesel;
extern crate forte_core;
extern crate r2d2;
extern crate taglib2_sys;
extern crate walkdir;

extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate lru_disk_cache;
extern crate mime_guess;
extern crate rand;
#[macro_use]
extern crate futures;
extern crate core;
extern crate futures_cpupool;
extern crate http_range;
extern crate juniper;
extern crate serde;
extern crate serde_json;
extern crate tokio_process;
extern crate uuid;

#[cfg(feature = "embed_web")]
#[macro_use]
extern crate rust_embed;

pub mod server;
pub mod sync;

use std::ops::Deref;
use std::path::PathBuf;

use server::temp::TemporaryFiles;

use structopt::StructOpt;

use app_dirs::app_root;
use app_dirs::AppDataType;
use app_dirs::AppInfo;

use error_chain::ChainedError;
use forte_core::context;
use lru_disk_cache::LruDiskCache;
use std::fs;

embed_migrations!("./migrations");

error_chain! {
    foreign_links {
        R2d2(::r2d2::Error);
        AppDirs(::app_dirs::AppDirsError);
        DieselMigration(::diesel_migrations::RunMigrationsError);
        Io(::std::io::Error);
        LruDiskCache(::lru_disk_cache::Error);
    }

    links {
        Sync(::sync::Error, ::sync::ErrorKind);
    }
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
    if let Err(ref err) = run() {
        println!("{}", err.display_chain());
    }
}

fn run() -> Result<()> {
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

            Ok(server::serve(pool, &host, transcode_cache, temporary_files))
        }
        Command::Sync { directory } => {
            let mut artwork_directory = app_dir;
            artwork_directory.push("artwork");
            fs::create_dir_all(&artwork_directory)?;

            sync::sync(pool, &directory, &artwork_directory)
        }
    }?;

    Ok(())
}

fn make_transcode_cache(app_dir: PathBuf) -> Result<LruDiskCache> {
    let mut transcode_cache_path = app_dir;
    transcode_cache_path.push("transcode-cache");

    let transcode_cache_size = 100_000_000_u64; // 100 MB

    Ok(LruDiskCache::new(
        transcode_cache_path,
        transcode_cache_size,
    )?)
}
