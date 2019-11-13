use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;

mod emulator;

enum Register {
    EAX = 0, ECX = 1, EDX = 2, EBX =3,
    ESP = 4, EBP = 5, ESI = 6, EDI = 7
}

struct ModRM {
    mode: u32,
    reg: u32,
    rm: u32
}

static REGISTER_NAME: [&str; 8] =
 ["EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI"];

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: {} FILE", args[0])
    }

    let mut emu = emulator::Emulator::new();

    let path = Path::new(&args[1]);
    let mut f: std::fs::File = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(f) => f,
    };

    emu.load_memory(&mut f);

    loop {
        let code = code8(&emu, 0);
        emu.epi_inc();
        println!("opcode: {:2X}", code);

        if (0xb8 <= code) && (code <= (0xb8 + 7)) {
            let reg = (code - 0xb8) as usize;
            let reg_name = REGISTER_NAME[reg];
            println!("reg: {}", REGISTER_NAME[reg]);
            println!("mov {},?", reg_name);
            let value = code32(&emu, 0);
            println!("mov {},{:#X}",reg_name,  value);
            emu.register[reg] = value;
            emu.epi_add4();
        } else if code == 0x89 {
            let modrm_code = code8(&emu, 0);
            emu.epi_inc();

            let modrm = read_modrm(modrm_code);

            let reg_name1 = REGISTER_NAME[modrm.reg as usize];

            if modrm.mode == 0b11 {
                let reg_name2 = REGISTER_NAME[modrm.rm as usize];
                println!("mov {},{}", reg_name2, reg_name1);
                emu.register[modrm.rm as usize] = emu.register[modrm.reg as usize];
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

fn code8(emu: &emulator::Emulator, index: usize) -> u32 {
    return emu.memory[emu.eip + index].into();
}

fn code32(emu: &emulator::Emulator, index: usize) -> u32 {
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

fn read_modrm(code: u32) -> ModRM {
    println!("ModR/M: {:X} {:#8b}", code, code);

    let mut modrm = ModRM {
        mode: 0,
        reg: 0,
        rm: 0
    };

    let mod_mask = 0b11000000;
    modrm.mode = (code & mod_mask) >> 6;

    let reg_mask = 0b00111000;
    modrm.reg = (code & reg_mask) >> 3;

    let rm_mask = 0b00000111;
    modrm.rm = code & rm_mask;

    println!("Mod: {:02b}, REG: {:03b}, R/M: {:03b}",
             modrm.mode, modrm.reg, modrm.rm);

    return modrm;
}

fn dump_register(emu: &emulator::Emulator) {
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

