use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

enum Register {
    EAX = 0, ECX = 1, EDX = 2, EBX =3,
    ESP = 4, EBP = 5, ESI = 6, EDI = 7
}

struct Emulator {
    memory: Vec<u8>,
    eip: usize,
    register: [u32; 8]
}

static REGISTER_NAME: [&str; 8] =
 ["EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI"];

fn main() {
    let mut emu = Emulator {
        memory: Vec::new(),
        eip: 0,
        register: [0; 8]
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

        if (0xb8 <= code) && (code <= (0xb8 + 7)) {
            let reg = (code - 0xb8) as usize;
            let reg_name = REGISTER_NAME[reg];
            println!("reg: {}", REGISTER_NAME[reg]);
            println!("mov {},?", reg_name);
            let value = code32(&mut emu, 0);
            println!("mov {},{:#X}",reg_name,  value);
            emu.register[reg] = value;
            emu.eip += 4;
        } else if code == 0xc3 {
            println!("ret");
            println!("break");
            break;
        } else {
            println!("unknown code");
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
    let mut data: String = String::new();

    let mut count = 0;
    loop {
        if count > 3 {
            break;
        }

        let mut temp = code8(emu, index + count);
        let str = format!("{:2X}", temp);
        data.push_str(&str);
        println!("hex: {:2X}", temp);
        println!("bin: {:032b}", temp);
        temp <<= 8 * count;
        println!("bin: {:032b}", temp);
        value += temp;

        count += 1;
    }
    println!("data: {}", data);

    return value;
}

