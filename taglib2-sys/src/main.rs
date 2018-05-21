extern crate taglib2_sys;

use std::env;
use taglib2_sys::SongProperties;

fn main() {
    env::args().skip(1).for_each(|arg| {
        let props = SongProperties::read(&arg).unwrap();

        println!("{:#?}", props);
    });
}
