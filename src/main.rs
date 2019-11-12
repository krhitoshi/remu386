use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

struct Emulator {
    memory: Vec<u8>,
    eip: usize,
}

fn main() {
    let mut emu = Emulator {
        memory: Vec::new(),
        eip: 0
    };

    let path = Path::new("test1");
    let mut f = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(f) => f,
    };

    f.read_to_end(&mut emu.memory);

    let code = code8(&mut emu, 0);
    println!("opcode: {:2X}", code);
    if code == 0xb8 {
        println!("mov eax,?");
    }
    emu.eip += 1;

    let value = code32(&mut emu, 0);
    println!("value: {}", value);
    emu.eip += 4;

    let code = code8(&mut emu, 0);
    println!("opcode: {:2X}", code);
    if code == 0xc3 {
        println!("ret");
    }
    emu.eip += 1;
}

fn code8(emu: &Emulator, index: usize) -> u32 {
    return emu.memory[emu.eip + index].into();
}

fn code32(emu: &Emulator, index: usize) -> u32 {
    let mut value: u32 = 0;

    let mut temp: u32 = 0;
    temp = code8(emu, 0);
    println!("hex: {:2X}", temp);
    println!("bin: {:032b}", temp);

    value = temp;
    println!("value: {}", value);

    temp = code8(emu, 1);
    value += temp << 8;
    println!("hex: {:2X}", temp);
    println!("bin: {:032b}", temp);
    println!("bin: {:032b}", temp << 8);
    println!("value: {}", value);

    temp = code8(emu, 2);
    value += temp << 16;
    println!("hex: {:2X}", temp);
    println!("bin: {:032b}", temp);
    println!("bin: {:032b}", temp << 16);
    println!("value: {}", value);

    temp = code8(emu, 3);
    value += temp << 24;
    println!("hex: {:2X}", temp);
    println!("bin: {:032b}", temp);
    println!("bin: {:032b}", temp << 25);
    println!("value: {}", value);
    return value;
}

