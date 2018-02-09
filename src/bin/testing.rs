extern crate taglib2_sys;

use std::ffi::{CStr, CString};
use std::env;

fn main() {
    let filename = env::args().nth(1).unwrap_or("<unknown-file>".to_owned());

    let filename_c = CString::new(filename).unwrap();
    let hello_string = unsafe { CStr::from_ptr(taglib2_sys::hello(filename_c.as_ptr())) };

    println!("{}", hello_string.to_str().unwrap());
}