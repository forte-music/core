#[macro_use]
extern crate error_chain;

use std::ffi::{CStr, CString};
use std::fmt::Debug;
use std::fmt::Formatter;
use std::os::raw::c_char;

error_chain! {
    foreign_links {
        NulError(::std::ffi::NulError);
    }

    errors {
        NoTagError(name: String) {
            description("the file doesn't contain a tag")
            display("the file '{}' doesn't contain a tag", name)
        }
    }
}

unsafe fn from_cstr(cstr: *const std::os::raw::c_char) -> Option<String> {
    if cstr.is_null() {
        return None;
    }

    Some(CStr::from_ptr(cstr).to_string_lossy().into_owned())
}

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
    fn fmt(&self, _f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
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

impl SongProperties {
    pub fn read(file_name: &str) -> Result<SongProperties> {
        let file_name_c = CString::new(file_name)?;
        let props_c = unsafe { song_properties(file_name_c.as_ptr()).as_ref() }
            .ok_or(ErrorKind::NoTagError(file_name.to_string()))?;

        let props = unsafe { SongProperties::from(props_c) };

        unsafe { destroy_properties(props_c) };

        Ok(props)
    }

    unsafe fn from(song_properties_c: &SongPropertiesC) -> Self {
        SongProperties {
            title: from_cstr((*song_properties_c).title),
            album: from_cstr((*song_properties_c).album),
            artist: from_cstr((*song_properties_c).artist),
            album_artist: from_cstr((*song_properties_c).album_artist),
            year: (*song_properties_c).year,
            track_number: (*song_properties_c).track_number,
            duration: (*song_properties_c).duration,
            cover: Picture::from_raw(
                song_properties_c.picture_data,
                song_properties_c.picture_data_len,
                song_properties_c.picture_mime,
            ),
        }
    }
}
