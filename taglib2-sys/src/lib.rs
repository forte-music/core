//#![allow(non_upper_case_globals)]
//#![allow(non_camel_case_types)]
//#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub use root::TagLib::*;

pub fn read_file(file_name: &str, read_audio_properties: bool, audio_properties_style: AudioProperties_ReadStyle) -> FileRef {
    let file_name_c = std::ffi::CString::new(file_name).unwrap();
    unsafe {
        FileRef::new1(file_name_c.as_ptr() as FileName, read_audio_properties, audio_properties_style)
    }
}
