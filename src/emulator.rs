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

    fn esp(&self) -> u32 {
        return self.register[Register::ESP as usize];
    }

    fn esp_add4(&mut self) {
        self.register[Register::ESP as usize] += 4;
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

    fn register_name(&self, index: u32) -> &str {
        return REGISTER_NAME[index as usize];
    }

    fn mem_set32(&mut self, address: u32, value: u32) {
        println!("address: {:08X}", address);
        println!("value: {:08X}", value);

        for i in 0..4 {
            let mask = 0xff << 8*i;
            let temp = (value & mask) >> 8*i;
            let offset = (address + i) as usize;
            self.memory[offset] = temp as u8;
            // println!("hex: {:02X}", temp);
        }
    }

    fn mem_get32(&self, address: u32) -> u32 {
        println!("address: {:08X}", address);
        let mut value: u32 = 0;

        for i in 0..4 {
            let temp = self.memory[(address + i) as usize] as u32;
            // println!("hex: {:02X}", temp);
            value += (temp << 8 * i);
        }
        println!("value: {:08X}", value);

        return value;
    }

    fn code8(&self, index: usize) -> u32 {
        return self.memory[self.eip + index].into();
    }

    fn code32(&self, index: usize) -> u32 {
        let mut value: u32 = 0;
        let mut data: String = String::new();

        for i in 0..4 {
            let mut temp = self.code8(index + i);
            let str = format!("{:2X}", temp);
            data.push_str(&str);
            // println!("hex: {:2X}", temp);
            // println!("bin: {:032b}", temp);
            temp <<= 8 * i;
            // println!("bin: {:032b}", temp);
            value += temp;
        }
        println!("data: {}", data);

        return value;
    }

    fn read_modrm(&self, code: u32) -> ModRM {
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
                let reg = code - 0x50;
                let reg_name = self.register_name(reg);
                println!("reg: {}", reg_name);
                println!("push {},?", reg_name);
                self.esp_sub4();
                self.mem_set32(self.esp(), self.register[reg as usize]);
            } else if (0x58 <= code) && (code <= (0x58 + 7)) {
                let reg = code - 0x58;
                let reg_name = self.register_name(reg);
                println!("reg: {}", reg_name);
                println!("pop {},?", reg_name);
                self.register[reg as usize] = self.mem_get32(self.esp());
                self.esp_add4();
            } else if (0xb8 <= code) && (code <= (0xb8 + 7)) {
                let reg = code - 0xb8;
                let reg_name = self.register_name(reg);
                println!("reg: {}", reg_name);
                println!("mov {},?", reg_name);
                let value = self.code32(0);
                println!("mov {},{:#X}",reg_name,  value);
                self.register[reg as usize] = value;
                self.epi_add4();
            } else if code == 0x89 {
                let modrm_code = self.code8(0);
                self.epi_inc();

                let modrm = self.read_modrm(modrm_code);

                let reg_name1 = self.register_name(modrm.reg);

                if modrm.mode == 0b11 {
                    let reg_name2 = self.register_name(modrm.rm);
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

    pub fn dump_memory(&self) {
        for i in 1..10 {
            let index = (MEMORY_SIZE - 4 * i) as usize;
            let mut data: String = String::new();
            for j in 0..4 {
                let str1 = format!("{:02X}", self.memory[index+j]);
                data.push_str(&str1);
            }
            println!("{:08X} : {}", index, data);
        }
    }

    pub fn dump_register(&self) {
        for i in 0..self.register.len() {
            let reg_name = self.register_name(i as u32);
            println!("{} = {:#010X}", reg_name, self.register[i]);
        }
    }
}