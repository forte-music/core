use mime_guess;
use mime_guess::Mime;

use taglib2_sys::{Picture, SongProperties};

use image;
use image::GenericImage;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::result;

error_chain! {
    foreign_links {
        Image(::image::ImageError);
        Io(::std::io::Error);
    }

    errors {
        UnknownExtension(mime: Mime) {
            description("couldn't determine the file extension for this mime type")
            display("couldn't determine file extension for mime type '{}'", mime)
        }

        NoExtensions(mime: Mime) {
            description("no extensions were returned for the mime type")
            display("no extensions were returned for the mime type '{}'", mime)
        }
    }
}

/// Gets the path of the best artwork for the file at `path`. It looks in two places for possible
/// artwork.
///
/// 1. The artwork embedded in the file's tags.
///
/// 2. PNG and JPEG files in the same directory as the song.
///
/// Artwork must be a square image. The artwork with the highest resolution is returned.
///
/// # Arguments
/// * `path` - The path of the song.
/// * `artwork_dir` - The directory artwork extracted from the audio file's tags are stored.
/// * `new_artwork_name` - The name of the file in which to store extracted artwork inside the `artwork_dir` directory.
/// * `props` - The information extracted from the song's tags.
pub fn get_best_artwork_path(
    path: &Path,
    artwork_dir: &Path,
    new_artwork_name: &str,
    props: &SongProperties,
) -> Result<Option<PathBuf>> {
    find_best_artwork(path, props)?.map_or(Ok(None), |info| {
        Ok(Some(info.make_and_get_path(artwork_dir, new_artwork_name)?))
    })
}

/// Holds information about the location of an image.
enum ImageType<'a> {
    /// Artwork embedded in the tag of a song.
    Embedded(&'a Picture),

    /// Artwork found near the song's audio file.
    Linked(PathBuf),
}

/// Holds the size and location of an image.
struct ImageInfo<'a> {
    /// One of the dimensions of an artwork. Since it all artwork must be a square, it could be
    /// either the width or height.
    size: u32,

    image_type: ImageType<'a>,
}

impl<'a> ImageInfo<'a> {
    /// Gets information about an embedded image. If the picture isn't a square returns `None`.
    fn from_embedded(picture: &Picture) -> result::Result<Option<ImageInfo>, image::ImageError> {
        let img = image::load_from_memory(&picture.data)?;
        let (width, height) = img.dimensions();

        if width != height {
            return Ok(None);
        }

        Ok(Some(ImageInfo {
            size: width,
            image_type: ImageType::Embedded(picture),
        }))
    }

    /// Gets information about an image on the disk. If the picture isn't a square, returns `None`.
    fn from_path(path: PathBuf) -> result::Result<Option<ImageInfo<'a>>, image::ImageError> {
        let img = image::open(&path)?;
        let (width, height) = img.dimensions();

        if width != height {
            return Ok(None);
        }

        Ok(Some(ImageInfo {
            size: width,
            image_type: ImageType::Linked(path),
        }))
    }

    /// Gets path to image, creating a file in `artwork_dir` if the image is embedded.
    fn make_and_get_path(self, artwork_dir: &Path, new_artwork_name: &str) -> Result<PathBuf> {
        let artwork_path = match self.image_type {
            ImageType::Embedded(picture) => {
                let extensions = mime_guess::get_mime_extensions(&picture.mime).ok_or(
                    Error::from(ErrorKind::UnknownExtension(picture.mime.clone())),
                )?;

                let extension = extensions
                    .get(0)
                    .ok_or(Error::from(ErrorKind::NoExtensions(picture.mime.clone())))?;

                let mut artwork_path = artwork_dir.to_owned();
                artwork_path.push(format!("{}.{}", new_artwork_name, extension));

                // Write Picture
                let mut artwork_file = File::create(&artwork_path)?;
                artwork_file.write_all(&picture.data)?;

                artwork_path
            }
            ImageType::Linked(path) => path,
        };

        Ok(artwork_path)
    }
}

/// Finds PNGs and JPEGs which are squares in the directory at `path`.
fn find_covers_in_path(path: &Path) -> Result<Vec<ImageInfo<'static>>> {
    let images = path.read_dir()?
        .filter_map(|e| e.ok())
        .filter_map(|file| {
            let path = file.path();
            if !path.is_file() {
                return None;
            };

            let extension = path.extension()?.to_string_lossy().to_lowercase();

            if ["jpg", "jpe", "jpeg", "png"].contains(&extension.as_str()) {
                return Some(path);
            }

            None
        })
        .map(ImageInfo::from_path)
        .collect::<result::Result<Vec<Option<ImageInfo>>, image::ImageError>>()?
        .into_iter()
        .filter_map(|option| option)
        .collect();

    Ok(images)
}

/// Searches the tag and `path` (see `find_covers_in_path`) for images which are squares returning
/// the one with the highest resolution.
fn find_best_artwork<'a>(path: &Path, props: &'a SongProperties) -> Result<Option<ImageInfo<'a>>> {
    let embedded_artwork = props
        .cover_artwork
        .as_ref()
        .map_or(Ok(None), ImageInfo::from_embedded)?;

    let linked_artwork = find_covers_in_path(path.parent().unwrap())?;

    let mut all_artwork = linked_artwork;
    if let Some(embedded_artwork) = embedded_artwork {
        all_artwork.push(embedded_artwork);
    }

    let best_artwork = all_artwork.into_iter().max_by_key(|info| info.size);

    Ok(best_artwork)
}
