extern crate dotenv;
extern crate forte_core;
extern crate r2d2;
extern crate taglib2_sys;
extern crate walkdir;

#[macro_use]
extern crate error_chain;

use std::env;
use std::ops::Deref;

use dotenv::dotenv;
use walkdir::WalkDir;

use forte_core::context;
use forte_core::import;

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
    let iter = WalkDir::new(path).follow_links(true);

    for entry in iter {
        let entry = entry?;
        let entry_path = entry.path();
        let extension = if let Some(extension) = entry_path.extension().and_then(|e| e.to_str()) {
            extension
        } else {
            continue;
        };

        if !FORMAT_EXTENSIONS.contains(&extension) {
            continue;
        }

        let props = SongProperties::read(entry_path)?;
        let props = if let Some(props) = props {
            props
        } else {
            continue;
        };

        import::add_song(props, conn)?;
    }

    Ok(())
}
