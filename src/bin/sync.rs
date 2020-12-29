use diesel::sqlite::SqliteConnection;
use forte_core::context;
use forte_core::import;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::path::Path;
use taglib2_sys::SongProperties;
use walkdir::DirEntry;
use walkdir::WalkDir;

error_chain! {
    links {
        Import(import::errors::Error, import::errors::ErrorKind);
    }

    foreign_links {
        R2d2(::r2d2::Error);
        WalkdirError(::walkdir::Error);
        Taglib(::taglib2_sys::Error);
    }

    errors {
            MissingSongProperties {
                description("this audio file doesn't have a tag")
            }
    }
}

const FORMAT_EXTENSIONS: [&str; 3] = ["flac", "mp3", "m4a"];

pub fn sync(pool: context::Pool, path: &Path, artwork_directory: &Path) -> Result<()> {
    let conn = pool.get()?;

    let entries: Vec<DirEntry> = WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|d| d.ok())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .and_then(|e| e.to_str())
                .map_or(false, |extension| {
                    FORMAT_EXTENSIONS.contains(&extension.to_lowercase().as_ref())
                })
        })
        .collect();

    let bar = ProgressBar::new(entries.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta})\n {msg}")
            .progress_chars("#>-"),
    );

    bar.wrap_iter(entries.iter()).for_each(|dir_entry| {
        let path = dir_entry.path();
        let path_string = path.display().to_string();

        let message = format!("Importing {}", path_string);
        bar.set_message(message.as_str());

        if let Err(e) = handle_entry(path, artwork_directory, &conn) {
            bar.println(format!("Error importing '{}': {}", path_string, e));
        }
    });

    bar.finish();

    Ok(())
}

/// The non-error outcomes of handling a path.
enum EntryResult {
    /// Importing the path was skipped because it is already in the database.
    Skipped,

    /// A new item was imported.
    Imported,
}

fn handle_entry(
    path: &Path,
    artwork_directory: &Path,
    conn: &SqliteConnection,
) -> Result<EntryResult> {
    if import::is_imported(path, conn)? {
        return Ok(EntryResult::Skipped);
    }

    let props = SongProperties::read(path)?;
    let props = props.ok_or(ErrorKind::MissingSongProperties)?;

    import::add_song(path, artwork_directory, props, conn)?;

    Ok(EntryResult::Imported)
}
