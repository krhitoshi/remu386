use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

enum Register {
    EAX = 0, ECX = 1, EDX = 2, EBX =3,
    ESP = 4, EBP = 5, ESI = 6, EDI = 7
}

// 1MB 0x00000 - 0xfffff
const MEMORY_SIZE: u32 = 1024 * 1024;

struct Emulator {
    memory: [u8; MEMORY_SIZE as usize],
    eip: usize,
    register: [u32; 8]
}

static REGISTER_NAME: [&str; 8] =
 ["EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI"];

fn main() {
    let mut emu = Emulator {
        memory: [0; MEMORY_SIZE as usize],
        eip: 0,
        register: [0; 8]
    };

    let path = Path::new("test2");
    let mut f = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(f) => f,
    };

    f.read(&mut emu.memory);

    loop {
        let code = code8(&emu, 0);
        emu.eip += 1;
        println!("opcode: {:2X}", code);

        if (0xb8 <= code) && (code <= (0xb8 + 7)) {
            let reg = (code - 0xb8) as usize;
            let reg_name = REGISTER_NAME[reg];
            println!("reg: {}", REGISTER_NAME[reg]);
            println!("mov {},?", reg_name);
            let value = code32(&emu, 0);
            println!("mov {},{:#X}",reg_name,  value);
            emu.register[reg] = value;
            emu.eip += 4;
        } else if code == 0x89 {
            let mod_rm = code8(&emu, 0);
            emu.eip += 1;
            println!("{:#8b}",mod_rm);
            let mod_mask = 0b11000000;
            let _mod = (mod_rm & mod_mask) >> 6;
            println!("Mod: {:#04b}", _mod);
            let reg_mask = 0b00111000;
            let reg = ((mod_rm & reg_mask) >> 3) as usize;
            println!("REG: {:#05b}", reg);
            let rm_mask = 0b00000111;
            let rm = (mod_rm & rm_mask) as usize;
            println!("R/M: {:#05b}", rm);

            let reg_name1 = REGISTER_NAME[reg];

            if _mod == 0b11 {
                let reg_name2 = REGISTER_NAME[rm];
                println!("mov {},{}", reg_name2, reg_name1);
                emu.register[rm] = emu.register[reg];
            } else {
                println!("unknown Mod");
                println!("break");
                break;
            }
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
    dump_register(&emu);
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

fn dump_register(emu: &Emulator) {
    let mut count = 0;
    loop {
        if count == emu.register.len() {
            break;
        }
        let reg_name = REGISTER_NAME[count];
        println!("{} = {:#010X}", reg_name, emu.register[count]);
        count += 1;
    }
}

