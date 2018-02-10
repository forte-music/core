extern crate taglib2_sys;

use std::env;

fn main() {
    let filename = env::args().nth(1).unwrap_or("<unknown-file>".to_owned());
    let song_properties = taglib2_sys::read_song_properties(&filename);

    println!("{:?}", song_properties);
}