use std::fs::File;
use std::io::Read;

// 1MB 0x00000 - 0xfffff
pub const MEMORY_SIZE: u32 = 1024 * 1024;

enum Register {
    EAX = 0, ECX = 1, EDX = 2, EBX = 3,
    ESP = 4, EBP = 5, ESI = 6, EDI = 7
}

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
        let mut emu = Emulator {
            memory: [0; MEMORY_SIZE as usize],
            eip: 0,
            register: [0; 8]
        };
        emu.register[Register::ESP as usize] = MEMORY_SIZE - 4;

        return emu;
    }

    pub fn load_memory(&mut self, mut file: &File) {
        file.read(&mut self.memory);
    }

    fn esp(&mut self) -> u32 {
        return self.register[Register::ESP as usize];
    }

    fn esp_sub4(&mut self) {
        self.register[Register::ESP as usize] -= 4;
    }

    fn epi_add4(&mut self) {
        self.eip += 4;
    }

    fn epi_inc(&mut self) {
        self.eip += 1;
    }

    fn mem_set32(&mut self, index: u32, value: u32) {
        println!("index: {:08X}", index);
        println!("value: {:08X}", value);

        let mask1 = 0xff000000;
        let temp1 = (value & mask1) >> 8*3;
        let offset1 = (index + 3) as usize;
        self.memory[offset1] = temp1 as u8;
        println!("hex: {:02X}", temp1);

        let mask2 = 0x00ff0000;
        let temp2 = (value & mask2) >> 8*2;
        let offset2 = (index + 2) as usize;
        self.memory[offset2] = temp2 as u8;
        println!("hex: {:02X}", temp2);

        let mask3 = 0x0000ff00;
        let temp3 = (value & mask3) >> 8*1;
        let offset3 = (index + 1) as usize;
        self.memory[offset3] = temp3 as u8;
        println!("hex: {:02X}", temp3);

        let mask4 = 0x000000ff;
        let temp4 = (value & mask4) >> 8*0;
        println!("hex: {:02X}", temp4);
        let offset4 = (index + 0) as usize;
        self.memory[offset4] = temp4 as u8;
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
            if (0x50 <= code) && (code <= (0x50 + 7)) {
                let reg = (code - 0x50) as usize;
                let reg_name = REGISTER_NAME[reg];
                println!("reg: {}", REGISTER_NAME[reg]);
                println!("push {},?", reg_name);
                self.esp_sub4();
                self.mem_set32(self.register[Register::ESP as usize],
                               self.register[reg]);
                // self.memory[self.esp()] = self.register[reg];
            } else if (0xb8 <= code) && (code <= (0xb8 + 7)) {
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

    pub fn dump_memory(&mut self) {
        let mut index = (MEMORY_SIZE - 1) as usize;
        loop {
            if index < 0xfff90 {
                break;
            }
            let mut data: String = String::new();
            let str1 = format!("{:02X}", self.memory[index-3]);
            data.push_str(&str1);
            let str2 = format!("{:02X}", self.memory[index-2]);
            data.push_str(&str2);
            let str3 = format!("{:02X}", self.memory[index-1]);
            data.push_str(&str3);
            let str4 = format!("{:02X}", self.memory[index]);
            data.push_str(&str4);
            println!("{:08X} : {}", index-3, data);
            index -= 4;
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