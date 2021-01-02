use mime::Mime;
use std::ffi::{CStr, CString};
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::os::raw::c_char;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("the path '{}' points to a directory instead of a file", .0.display())]
    InvalidPathError(PathBuf),

    #[error(transparent)]
    NulError(#[from] std::ffi::NulError),
}

unsafe fn from_cstr(cstr: *const c_char) -> Option<String> {
    if cstr.is_null() {
        return None;
    }

    Some(CStr::from_ptr(cstr).to_string_lossy().into_owned())
}

extern "C" {
    fn song_properties(file_name: *const c_char) -> *const SongPropertiesC;
    fn destroy_properties(song_properties: *const SongPropertiesC);
}

#[repr(C)]
struct SongPropertiesC {
    title: *const c_char,
    album: *const c_char,
    artist: *const c_char,
    album_artist: *const c_char,
    disk_number: *const c_char,
    year: u32,
    track_number: u32,
    duration: i32,
    picture_data: *const u8,
    picture_data_len: u32,
    picture_mime: *const c_char,
}

pub struct Picture {
    pub data: Vec<u8>,
    pub mime: Mime,
}

impl Picture {
    fn from_raw(data: *const u8, len: u32, raw_mime: *const c_char) -> Option<Picture> {
        if data.is_null() {
            return None;
        }

        unsafe {
            let bytes = std::slice::from_raw_parts(data, len as usize);
            let mime_string = from_cstr(raw_mime)?;
            let mime = Mime::from_str(&mime_string).ok()?;

            Some(Picture {
                data: bytes.to_vec(),
                mime,
            })
        }
    }
}

impl Debug for Picture {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "MIME: {:?}", self.mime)
    }
}

#[derive(Debug)]
pub struct SongProperties {
    pub title: Option<String>,
    pub album: Option<String>,
    pub artist: Option<String>,
    pub album_artist: Option<String>,
    pub year: Option<u32>,
    pub disk_number: Option<u32>,
    pub track_number: u32,
    pub duration: i32,
    pub cover_artwork: Option<Picture>,
}

impl SongProperties {
    #[cfg(unix)]
    pub fn read(path: &Path) -> Result<Option<SongProperties>, Error> {
        use std::os::unix::ffi::OsStrExt;

        if path.is_dir() {
            return Err(Error::InvalidPathError(path.to_path_buf()));
        }

        let file_name = path.as_os_str().as_bytes();
        let file_name_c = CString::new(file_name)?;
        let props_c = match unsafe { song_properties(file_name_c.as_ptr()).as_ref() } {
            Some(props_c) => props_c,
            None => return Ok(None),
        };

        let props = unsafe { SongProperties::from(props_c) };

        unsafe { destroy_properties(props_c) };

        Ok(Some(props))
    }

    unsafe fn from(song_properties_c: &SongPropertiesC) -> Self {
        let year = (*song_properties_c).year;
        let year = if year == 0 { None } else { Some(year) };

        let disk_number: Option<u32> =
            from_cstr((*song_properties_c).disk_number).and_then(|s| s.parse().ok());

        SongProperties {
            title: from_cstr((*song_properties_c).title),
            album: from_cstr((*song_properties_c).album),
            artist: from_cstr((*song_properties_c).artist),
            album_artist: from_cstr((*song_properties_c).album_artist),
            year,
            disk_number,
            track_number: (*song_properties_c).track_number,
            duration: (*song_properties_c).duration,
            cover_artwork: Picture::from_raw(
                song_properties_c.picture_data,
                song_properties_c.picture_data_len,
                song_properties_c.picture_mime,
            ),
        }
    }
}
