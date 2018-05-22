extern crate taglib2_sys;

use std::env;
use std::path::Path;
use taglib2_sys::SongProperties;

fn main() {
    env::args().skip(1).for_each(|arg| {
        let path = Path::new(&arg);
        let props = SongProperties::read(&path).unwrap();

        println!("{:#?}", props);
    });
}
