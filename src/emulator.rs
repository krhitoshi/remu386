use std::fs::File;
use std::io::Read;

// 1MB 0x00000 - 0xfffff
pub const MEMORY_SIZE: u32 = 1024 * 1024;

// enum Register {
//     EAX = 0, ECX = 1, EDX = 2, EBX =3,
//     ESP = 4, EBP = 5, ESI = 6, EDI = 7
// }

pub struct Emulator {
    memory: [u8; MEMORY_SIZE as usize],
    eip: usize,
    register: [u32; 8]
}

struct ModRM {
    mode: u32,
    reg: u32,
    rm: u32
}

static REGISTER_NAME: [&str; 8] =
 ["EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI"];

impl Emulator {
    pub fn new() -> Emulator {
        return Emulator {
            memory: [0; MEMORY_SIZE as usize],
            eip: 0,
            register: [0; 8]
        };
    }

    pub fn load_memory(&mut self, mut file: &File) {
        file.read(&mut self.memory);
    }

    fn epi_add4(&mut self) {
        self.eip += 4;
    }

    fn epi_inc(&mut self) {
        self.eip += 1;
    }

    fn code8(&mut self, index: usize) -> u32 {
        return self.memory[self.eip + index].into();
    }

    fn code32(&mut self, index: usize) -> u32 {
        let mut value: u32 = 0;
        let mut data: String = String::new();

        let mut count = 0;
        loop {
            if count > 3 {
                break;
            }

            let mut temp = self.code8(index + count);
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

    fn read_modrm(&mut self, code: u32) -> ModRM {
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

    pub fn launch(&mut self) {
        loop {
            let code = self.code8(0);
            self.epi_inc();
            println!("opcode: {:2X}", code);

            if (0xb8 <= code) && (code <= (0xb8 + 7)) {
                let reg = (code - 0xb8) as usize;
                let reg_name = REGISTER_NAME[reg];
                println!("reg: {}", REGISTER_NAME[reg]);
                println!("mov {},?", reg_name);
                let value = self.code32(0);
                println!("mov {},{:#X}",reg_name,  value);
                self.register[reg] = value;
                self.epi_add4();
            } else if code == 0x89 {
                let modrm_code = self.code8(0);
                self.epi_inc();

                let modrm = self.read_modrm(modrm_code);

                let reg_name1 = REGISTER_NAME[modrm.reg as usize];

                if modrm.mode == 0b11 {
                    let reg_name2 = REGISTER_NAME[modrm.rm as usize];
                    println!("mov {},{}", reg_name2, reg_name1);
                    self.register[modrm.rm as usize] = self.register[modrm.reg as usize];
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
    }

    pub fn dump_register(&mut self) {
        let mut count = 0;
        loop {
            if count == self.register.len() {
                break;
            }
            let reg_name = REGISTER_NAME[count];
            println!("{} = {:#010X}", reg_name, self.register[count]);
            count += 1;
        }
    }
}