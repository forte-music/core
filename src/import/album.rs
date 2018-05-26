use mime_guess;

use taglib2_sys::{Picture, SongProperties};

use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use chrono::prelude::*;
use diesel::prelude::*;
use models::*;

use super::errors;
use database::album;

pub fn add_or_get_album(
    path: &Path,
    artwork_dir: &Path,
    props: &SongProperties,
    name: &str,
    artist_id: UUID,
    release_year: Option<u32>,
    conn: &SqliteConnection,
) -> errors::Result<Album> {
    let album: Option<Album> = album::table
        .filter(album::name.eq(name))
        .filter(album::artist_id.eq(artist_id))
        .first(conn)
        .optional()?;

    if let Some(album) = album {
        return Ok(album);
    }

    let id = UUID::new();

    let artwork_path: Option<PathBuf> = props
        .cover_artwork
        .as_ref()
        .map_or(Ok(None), |picture| {
            create_file_for_picture(id, artwork_dir, picture)
        })?
        .map_or_else(
            || find_artwork_in_dir(path.parent().unwrap()),
            |path| Ok(Some(path)),
        )?;

    let album = Album {
        id,
        artwork_path: artwork_path.map(|p| p.into()),
        name: name.to_string(),
        artist_id,
        release_year: release_year.map(|year| year as i32),
        time_added: Utc::now().naive_utc(),
        last_played: None,
    };

    album.clone().insert_into(album::table).execute(conn)?;

    Ok(album)
}

fn create_file_for_picture(
    album_id: UUID,
    artwork_dir: &Path,
    picture: &Picture,
) -> io::Result<Option<PathBuf>> {
    mime_guess::get_mime_extensions(&picture.mime)
        .and_then(|extensions| extensions.get(0))
        .map_or(Ok(None), |extension| {
            let mut artwork_path = artwork_dir.to_owned();
            artwork_path.push(format!("{}.{}", album_id.to_string(), extension));

            // Write Picture
            let mut artwork_file = File::create(&artwork_path)?;
            artwork_file.write_all(&picture.data)?;

            Ok(Some(artwork_path))
        })
}

fn find_artwork_in_dir(directory: &Path) -> io::Result<Option<PathBuf>> {
    let images_iter = directory
        .read_dir()?
        .filter_map(|e| e.ok())
        .filter_map(|file| {
            let path = file.path();
            let extension = path.extension()?.to_string_lossy().to_lowercase();

            if path.is_file() && ["jpg", "jpeg", "png"].contains(&extension.as_str()) {
                return Some(path);
            }

            None
        });

    // If the file contains one of {folder,cover,front,thumb}.{jpg,png}, use it, otherwise use any
    // jpg/png.
    let mut first: Option<PathBuf> = None;

    for image_path_buf in images_iter {
        first = Some(image_path_buf.clone());

        let image_path: &Path = &image_path_buf.clone();
        let file_name = image_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_lowercase();

        if ["folder", "cover", "front", "thumb"].contains(&file_name.as_str()) {
            return Ok(Some(image_path_buf));
        }
    }

    return Ok(first);
}
