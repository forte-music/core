extern crate dotenv;
extern crate forte_core;
extern crate indicatif;
extern crate r2d2;
extern crate taglib2_sys;
extern crate walkdir;

#[macro_use]
extern crate error_chain;

use std::env;
use std::ops::Deref;

use dotenv::dotenv;
use indicatif::ProgressBar;
use walkdir::WalkDir;

use forte_core::context;
use forte_core::import;

use indicatif::ProgressStyle;
use taglib2_sys::SongProperties;
use walkdir::DirEntry;

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
}

fn main() {
    start().unwrap();
}

const FORMAT_EXTENSIONS: [&str; 3] = ["flac", "mp3", "m4a"];

fn start() -> Result<()> {
    dotenv().ok();

    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        return Err("invalid number of arguments there should only be one".into());
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
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta})")
            .progress_chars("#>-"),
    );

    for entry in entries {
        let path = entry.path();
        let props = SongProperties::read(path)?;
        let props = if let Some(props) = props {
            props
        } else {
            continue;
        };

        import::add_song(props, conn)?;

        bar.inc(1);
    }

    bar.finish();

    Ok(())
}
