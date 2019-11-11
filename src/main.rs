use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("helloworld.bin");
    let mut f = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(f) => f,
    };

    let mut buffer = [0; 1];
    f.read(&mut buffer);
    println!("{:#04x}", buffer[0]);
    // println!("{:#04x}", buffer[1]);
    // println!("{:#04x}", buffer[2]);
    // println!("{:#04x}", buffer[3]);

    // for byte in f.bytes() {
    //     println!("{:#04x}", byte.unwrap());
    // }
}
