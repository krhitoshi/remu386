use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("test1");
    let mut f = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(f) => f,
    };

    let mut eip = 0;

    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer);

    println!("opcode: {:2X}", buffer[eip]);
    if buffer[eip] == 0xb8 {
        println!("mov eax,?");
    }

    eip += 1;

    let mut value: u32 = 0;

    let mut temp: u32 = 0;
    temp = buffer[eip] as u32;
    println!("hex: {:2X}", buffer[eip]);
    println!("bin: {:032b}", buffer[eip]);

    value = temp;
    println!("value: {}", value);
    eip += 1;

    temp = buffer[eip] as u32;
    value += (temp << 8);
    println!("hex: {:2X}", buffer[eip]);
    println!("bin: {:032b}", buffer[eip]);
    println!("bin: {:032b}", temp << 8);
    println!("value: {}", value);
    eip += 1;

    temp = buffer[eip] as u32;
    value += (temp << 16);
    println!("hex: {:2X}", buffer[eip]);
    println!("bin: {:032b}", buffer[eip]);
    println!("bin: {:032b}", temp << 16);
    println!("value: {}", value);
    eip += 1;

    temp = buffer[eip] as u32;
    value += (temp << 24);
    println!("hex: {:2X}", buffer[eip]);
    println!("bin: {:032b}", buffer[eip]);
    println!("bin: {:032b}", temp << 25);
    println!("value: {}", value);
    eip += 1;

    // for byte in f.bytes() {
    //     println!("{:#04x}", byte.unwrap());
    // }
}
