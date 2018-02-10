use std::ffi::{CString, CStr};

extern "C" {
    fn song_properties(file_name: *const std::os::raw::c_char) -> *const SongPropertiesC;
    fn destroy_properties(song_properties: *const SongPropertiesC);
}

#[repr(C)]
struct SongPropertiesC {
    title: *const std::os::raw::c_char,
    album: *const std::os::raw::c_char,
    artist: *const std::os::raw::c_char,
    album_artist: *const std::os::raw::c_char,
    year: u32,
    track_number: u32,
    duration: i32,
    picture_data: *const std::os::raw::c_char,
    picture_data_len: u32,
    picture_mime: *const std::os::raw::c_char
}

#[derive(Debug)]
pub struct SongProperties {
    title: String,
    album: String,
    artist: String,
    album_artist: String,
    year: u32,
    track_number: u32,
    duration: i32,
    picture_data: Vec<u8>,
    picture_mime: String
}

pub fn read_song_properties(file_name: &str) -> Option<SongProperties> {
    let file_name_c = CString::new(file_name).unwrap();
    let song_properties_c = unsafe {
        match song_properties(file_name_c.as_ptr()).as_ref() {
            Some(p) => p,
            None => return None
        }
    };

    let song_properties = unsafe {
        SongProperties {
            title: CStr::from_ptr((*song_properties_c).title).to_string_lossy().into_owned(),
            album: CStr::from_ptr((*song_properties_c).album).to_string_lossy().into_owned(),
            artist: CStr::from_ptr((*song_properties_c).artist).to_string_lossy().into_owned(),
            album_artist: CStr::from_ptr((*song_properties_c).album_artist).to_string_lossy().into_owned(),
            year: (*song_properties_c).year,
            track_number: (*song_properties_c).track_number,
            duration: (*song_properties_c).duration,
            picture_data: std::slice::from_raw_parts(
                (*song_properties_c).picture_data as *const u8,
                (*song_properties_c).picture_data_len as usize
            ).to_vec(),
            picture_mime: CStr::from_ptr((*song_properties_c).picture_mime).to_string_lossy().into_owned()
        }
    };

    unsafe { destroy_properties(song_properties_c) };

    Some(song_properties)
}
