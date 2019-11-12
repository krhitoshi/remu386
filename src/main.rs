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

    loop {
        let code = code8(&mut emu, 0);
        emu.eip += 1;
        println!("opcode: {:2X}", code);

        if code == 0xb8 {
            println!("mov eax,?");
            let value = code32(&mut emu, 0);
            println!("mov eax,{:#X}", value);
            emu.eip += 4;
        } else if code == 0xc3 {
            println!("ret");
            println!("break");
            break;
        }
    }
}

fn code8(emu: &Emulator, index: usize) -> u32 {
    return emu.memory[emu.eip + index].into();
}

fn code32(emu: &Emulator, index: usize) -> u32 {
    let mut value: u32 = 0;

    let mut count = 0;
    loop {
        if count > 3 {
            break;
        }

        let mut temp = code8(emu, index + count);
        println!("hex: {:2X}", temp);
        println!("bin: {:032b}", temp);
        temp <<= 8 * count;
        println!("bin: {:032b}", temp);
        value += temp;

        count += 1;
    }

    return value;
}

