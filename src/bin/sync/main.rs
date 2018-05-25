extern crate dotenv;
extern crate forte_core;
extern crate indicatif;
extern crate r2d2;
extern crate taglib2_sys;
extern crate walkdir;

#[macro_use]
extern crate error_chain;
extern crate diesel;

use std::env;
use std::ops::Deref;

use dotenv::dotenv;
use walkdir::DirEntry;
use walkdir::WalkDir;

use forte_core::context;
use forte_core::import;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;

use diesel::sqlite::SqliteConnection;
use std::path::Path;
use taglib2_sys::SongProperties;

error_chain! {
    links {
        Taglib(::taglib2_sys::Error, ::taglib2_sys::ErrorKind);
        Import(::import::errors::Error, ::import::errors::ErrorKind);
    }

    foreign_links {
        R2d2(::r2d2::Error);
        VarError(::std::env::VarError);
        WalkdirError(::walkdir::Error);
    }

    errors {
            MissingSongProperties {
                description("this audio file doesn't have a tag")
            }
    }
}

fn main() {
    start().unwrap();
}

const FORMAT_EXTENSIONS: [&str; 3] = ["flac", "mp3", "m4a"];

fn start() -> Result<()> {
    dotenv().ok();

    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        return Err("invalid number of arguments; there should only be one".into());
    }

    let database_url = env::var("DATABASE_URL")?;
    let pool = context::init_pool(&database_url)?;
    let connection = pool.get()?;
    let conn = connection.deref();

    let path = args.get(0).unwrap();
    let entries: Vec<DirEntry> = WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|d| d.ok())
        .filter(|entry| {
            let path = entry.path();
            let extension = path.extension().and_then(|e| e.to_str());
            match extension {
                Some(extension) if FORMAT_EXTENSIONS.contains(&extension) => true,
                _ => false,
            }
        })
        .collect();

    let bar = ProgressBar::new(entries.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{prefix}[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta})\n {msg}",
            )
            .progress_chars("#>-"),
    );

    let mut prefix = String::new();

    bar.wrap_iter(entries.iter()).for_each(|dir_entry| {
        let path = dir_entry.path();
        let path_string = path.display().to_string();

        let message = format!("Importing {}", path_string);
        bar.set_message(message.as_str());

        if let Err(e) = handle_entry(path, conn) {
            prefix = prefix.clone() + &format!("Error importing '{}': {}\n", path_string, e);
            bar.set_prefix(prefix.as_str())
        }
    });

    bar.finish();

    Ok(())
}

fn handle_entry(path: &Path, conn: &SqliteConnection) -> Result<()> {
    let props = SongProperties::read(path)?;
    let props = props.ok_or(ErrorKind::MissingSongProperties)?;

    import::add_song(path, props, conn)?;

    Ok(())
}
