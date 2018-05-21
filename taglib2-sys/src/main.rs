extern crate taglib2_sys;

use taglib2_sys::read_song_properties;
use std::env;

fn main() {
    env::args().skip(1).for_each(|arg| {
        let props = read_song_properties(&arg);

        println!("{:#?}", props);
    });
}
