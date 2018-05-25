extern crate indicatif;

use std::ops::Deref;
use std::path::Path;

use forte_core::context;
use forte_core::import;

use diesel::sqlite::SqliteConnection;

use walkdir::DirEntry;
use walkdir::WalkDir;

use taglib2_sys::SongProperties;

use self::indicatif::ProgressBar;
use self::indicatif::ProgressStyle;

error_chain! {
    links {
        Taglib(::taglib2_sys::Error, ::taglib2_sys::ErrorKind);
        Import(import::errors::Error, import::errors::ErrorKind);
    }

    foreign_links {
        R2d2(::r2d2::Error);
        WalkdirError(::walkdir::Error);
    }

    errors {
            MissingSongProperties {
                description("this audio file doesn't have a tag")
            }
    }
}

const FORMAT_EXTENSIONS: [&str; 3] = ["flac", "mp3", "m4a"];

pub fn sync(pool: context::Pool, path: &Path, artwork_directory: &Path) -> Result<()> {
    let connection = pool.get()?;
    let conn = connection.deref();

    let entries: Vec<DirEntry> = WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|d| d.ok())
        .filter(|entry| {
            let path = entry.path();
            let extension = path.extension().and_then(|e| e.to_str());
            match extension {
                Some(extension)
                    if FORMAT_EXTENSIONS.contains(&extension.to_lowercase().as_ref()) =>
                {
                    true
                }
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

        if let Err(e) = handle_entry(path, artwork_directory, conn) {
            prefix = prefix.clone() + &format!("Error importing '{}': {}\n", path_string, e);
            bar.set_prefix(prefix.as_str())
        }
    });

    bar.finish();

    Ok(())
}

fn handle_entry(path: &Path, artwork_directory: &Path, conn: &SqliteConnection) -> Result<()> {
    let props = SongProperties::read(path)?;
    let props = props.ok_or(ErrorKind::MissingSongProperties)?;

    import::add_song(path, artwork_directory, props, conn)?;

    Ok(())
}