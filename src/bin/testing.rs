extern crate taglib2_sys;

use taglib2_sys::AudioProperties_ReadStyle;
use taglib2_sys::read_file;
use taglib2_sys::Tag_properties;

use std::ffi::CStr;
use std::env;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let file = read_file(&filename, false, AudioProperties_ReadStyle::Average);
    let tag = unsafe { file.tag() };
    let properties = unsafe { Tag_properties(tag as *mut std::os::raw::c_void) };
    let properties_string = unsafe { CStr::from_ptr(properties.toString().toCString(false)) };

    println!("{}", properties_string.to_str().unwrap());
}