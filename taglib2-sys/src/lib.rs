use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt;

extern "C" {
    fn song_properties(file_name: *const std::os::raw::c_char) -> *const SongPropertiesC;
    fn destroy_properties(song_properties: *const SongPropertiesC);
}

#[repr(C)]
struct SongPropertiesC {
    title: *const c_char,
    album: *const c_char,
    artist: *const c_char,
    album_artist: *const c_char,
    year: u32,
    track_number: u32,
    duration: i32,
    picture_data: *const u8,
    picture_data_len: u32,
    picture_mime: *const c_char,
}

pub struct Picture {
    data: Vec<u8>,
    mime: Option<String>,
}

impl Picture {
    fn from_raw(data: *const u8, len: u32, mime: *const c_char) -> Option<Picture> {
        if data.is_null() {
            return None;
        }

        unsafe {
            let bytes = std::slice::from_raw_parts(data, len as usize);
            Some(Picture {
                data: bytes.to_vec(),
                mime: from_cstr(mime),
            })
        }
    }
}

impl Debug for Picture {
    fn fmt(&self, _f: &mut Formatter) -> Result<(), fmt::Error> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct SongProperties {
    title: Option<String>,
    album: Option<String>,
    artist: Option<String>,
    album_artist: Option<String>,
    year: u32,
    track_number: u32,
    duration: i32,
    cover: Option<Picture>,
}

pub fn read_song_properties(file_name: &str) -> Option<SongProperties> {
    let file_name_c = CString::new(file_name).unwrap();
    let song_properties_c = unsafe {
        match song_properties(file_name_c.as_ptr()).as_ref() {
            Some(p) => p,
            None => return None,
        }
    };

    let song_properties = unsafe {
        SongProperties {
            title: from_cstr((*song_properties_c).title),
            album: from_cstr((*song_properties_c).album),
            artist: from_cstr((*song_properties_c).artist),
            album_artist: from_cstr((*song_properties_c).album_artist),
            year: (*song_properties_c).year,
            track_number: (*song_properties_c).track_number,
            duration: (*song_properties_c).duration,
            cover: Picture::from_raw(song_properties_c.picture_data, song_properties_c.picture_data_len, song_properties_c.picture_mime),
        }
    };

    unsafe { destroy_properties(song_properties_c) };

    Some(song_properties)
}

unsafe fn from_cstr(cstr: *const std::os::raw::c_char) -> Option<String> {
    if cstr.is_null() {
        return None;
    }

    Some(CStr::from_ptr(cstr).to_string_lossy().into_owned())
}
