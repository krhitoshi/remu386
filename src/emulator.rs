mod modrm;
use modrm::ModRM;
use Register::*;

pub const DEBUG: bool = true;

// 1MB 0x00000 - 0xfffff
pub const MEMORY_SIZE: u32 = 1024 * 1024;

enum Register {
    EAX = 0, ECX = 1, EDX = 2, EBX = 3,
    ESP = 4, EBP = 5, ESI = 6, EDI = 7
}

pub struct Emulator {
    pub memory: Vec<u8>,
    eip: u32,
    register: [u32; 8],
    eflags: u32
}

struct SIB {
    scale: u32,
    index: u32,
    base: u32
}

const REGISTER_NAME: [&str; 8] =
 ["EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI"];

fn register_name(index: u32) -> String {
    return REGISTER_NAME[index as usize].to_string();
}

impl Emulator {
    pub fn new(mem_size: u32) -> Self {
        let mut emu = Self {
            memory: Vec::with_capacity(mem_size as usize),
            eip: 0,
            register: [0; 8],
            eflags: 0
        };

        // initialize memory
        for _ in 0..mem_size {
            emu.memory.push(0);
        }
        emu.register[ESP as usize] = mem_size - 4;

        return emu;
    }

    fn esp(&self) -> u32 {
        return self.register[ESP as usize];
    }

    fn esp_add4(&mut self) {
        self.register[ESP as usize] += 4;
    }

    fn esp_sub4(&mut self) {
        self.register[ESP as usize] -= 4;
    }

    fn epi_add4(&mut self) {
        self.eip += 4;
    }

    fn epi_inc(&mut self) {
        self.eip += 1;
    }

    fn register(&self, index: u32) -> u32 {
        return self.register[index as usize];
    }

    fn memory(&self, address: u32) -> u8 {
        return self.memory[(address) as usize];
    }

    fn memory_i8(&self, address: u32) -> i8 {
        return self.memory(address) as i8;
    }

    fn push32(&mut self, value: u32) {
        self.esp_sub4();
        self.memory_set32(self.esp(), value);
    }

    fn pop32(&mut self) -> u32 {
        let value = self.memory_u32(self.esp());
        self.esp_add4();
        return value;
    }

    fn memory_set8(&mut self, address: u32, value: u8) {
        self.memory[address as usize] = value;
    }

    fn memory_set32(&mut self, address: u32, value: u32) {
        // println!("address: {:08X}", address);
        // println!("value: {:08X}", value);
        for i in 0..4 {
            let mask = 0xff << 8*i;
            let data = (value & mask) >> 8*i;
            self.memory_set8(address + i, data as u8);
            // println!("hex: {:02X}", temp);
        }
    }

    fn memory_u32(&self, address: u32) -> u32 {
        // println!("address: {:08X}", address);
        let mut value: u32 = 0;

        for i in 0..4 {
            let temp = self.memory(address + i) as u32;
            // println!("hex: {:02X}", temp);
            value += temp << 8 * i;
        }
        // println!("value: {:08X}", value);

        return value;
    }

    fn code8(&self, index: u32) -> u32 {
        return self.memory(self.eip + index).into();
    }

    fn sign_code8(&self, index: u32) -> i32 {
        return self.memory_i8(self.eip + index).into();
    }

    fn sign_code32(&self, index: u32) -> i32 {
        let value = self.code32(index);
        return value as i32;
    }

    fn code32(&self, index: u32) -> u32 {
        let mut value: u32 = 0;
        let mut data: String = String::new();

        for i in 0..4 {
            let mut temp = self.code8(index + i);
            let str = format!("{:02X} ", temp);
            data.push_str(&str);
            // println!("hex: {:2X}", temp);
            // println!("bin: {:032b}", temp);
            temp <<= 8 * i;
            // println!("bin: {:032b}", temp);
            value += temp;
        }
        if DEBUG {
            println!("data: {}", data);
        }

        return value;
    }

    fn read_modrm(&mut self) -> ModRM {
        let code = self.code8(0);
        self.epi_inc();
        return ModRM::new(code);
    }

    fn read_sib(&mut self) -> SIB {
        let code = self.code8(0);
        self.epi_inc();
        if DEBUG {
            println!("SIB: {:02X} {:#8b}", code, code);
        }

        let mut sib = SIB {
            scale: 0,
            index: 0,
            base: 0
        };

        let mod_mask = 0b11000000;
        sib.scale = (code & mod_mask) >> 6;

        let reg_mask = 0b00111000;
        sib.index = (code & reg_mask) >> 3;

        let rm_mask = 0b00000111;
        sib.base = code & rm_mask;

        if DEBUG {
            println!("scale: {:02b}, index: {:03b}, base {:03b}",
                     sib.scale, sib.index, sib.base);
        }
        return sib;
    }

    fn read_effective_address(&mut self) -> (u32, u32) {
        let modrm = self.read_modrm();
        return self.read_effective_address_from_modrm(&modrm);
    }

    fn read_effective_address_from_modrm(&mut self, modrm: &ModRM) -> (u32, u32) {
        if modrm.mode == 0b01 {
            if modrm.rm == 0b100 {
                let sib = self.read_sib();
                let disp = self.sign_code8(0);
                self.epi_inc();
                if sib.scale == 0 && sib.index == 0b100 {
                    if DEBUG {
                        let reg_name = register_name(sib.base);
                        println!("address: [{} {}]", reg_name, disp);
                    }
                    let temp = self.register(sib.base) as i32;
                    let address = (temp + disp) as u32;
                    return (modrm.reg, address);
                } else {
                    unimplemented!("not implemented ModR/M rm: 100");
                }
            } else {
                let disp = self.sign_code8(0);
                self.epi_inc();
                if DEBUG {
                    let reg_name = register_name(modrm.rm);
                    println!("address: [{} {}]", reg_name, disp);
                }
                let temp = self.register(modrm.rm) as i32;
                let address = (temp + disp) as u32;
                return (modrm.reg, address);
            }
        } else if modrm.mode == 0b10 {
            if modrm.rm == 0b100 {
                let sib = self.read_sib();
                let disp = self.sign_code32(0);
                self.epi_add4();
                if sib.scale == 0 && sib.index == 0b100 {
                    if DEBUG {
                        let reg_name2 = register_name(sib.base);
                        println!("address: [{} {}]", reg_name2, disp);
                    }
                    let temp = self.register(sib.base) as i32;
                    let address = (temp + disp) as u32;
                    return (modrm.reg, address);
                } else {
                    unimplemented!("");
                }
            } else {
                unimplemented!("");
            }
        } else {
            unimplemented!("unknown Mod: {:02b}", modrm.mode);
        }
    }

    fn leave(&mut self) {
        if DEBUG {
            println!("leave");
        }
        self.register[ESP as usize] = self.register[EBP as usize];
        self.register[EBP as usize] = self.pop32();
    }

    fn jump(&mut self, value: i32) {
        let mut address = self.eip as i32;
        address += value;
        if DEBUG {
            println!("jump => {:08X}", address);
        }
        self.eip = address as u32;
    }

    fn jump_short(&mut self) {
        let value = self.sign_code8(0);
        if DEBUG {
            println!("jmp short, {:08X}, {}", value, value);
        }
        self.jump(value + 1);
    }

    fn shr_rm32(&mut self, modrm: ModRM) {
        if modrm.mode == 0b11 {
            if DEBUG {
                let reg_name = register_name(modrm.reg);
                println!("shr: {},{}", reg_name, 1);
            }
            self.register[modrm.reg as usize] = self.register[modrm.reg as usize] >> 1;
        } else {
            unimplemented!();
        }
    }

    fn push_rm32(&mut self, modrm: ModRM) {
        if modrm.mode == 0b01 {
            let (_reg, address) = self.read_effective_address_from_modrm(&modrm);
            if DEBUG {
                println!("push: [{:08X}]", address);
            }
            let value = self.memory_u32(address);
            if DEBUG {
                println!("push: {:08X}", value);
            }
            self.push32(value);
        } else {
            unimplemented!("unknown Mod");
        }
    }

    fn push_r32(&mut self, code: u32) {
        let reg = code - 0x50;
        let reg_name = register_name(reg);
        if DEBUG {
            println!("push {}", reg_name);
        }
        self.esp_sub4();
        self.memory_set32(self.esp(), self.register(reg));
    }

    fn push_imm8(&mut self) {
        let value = self.code8(0);
        self.epi_inc();
        if DEBUG {
            println!("push {:#04X} {}", value, value);
        }
        self.push32(value as u32);
    }

    fn pop_r32(&mut self, code: u32) {
        let reg = code - 0x58;
        let reg_name = register_name(reg);
        if DEBUG {
            println!("pop {}", reg_name);
        }
        let value = self.pop32();
        if DEBUG {
            println!("value: {:X}", value);
        }
        self.register[reg as usize] = value;
    }

    fn add_rm32_imm32(&mut self, modrm: ModRM) {
        let reg_name = register_name(modrm.rm);
        let value = self.code32(0);
        if DEBUG {
            println!("add {},{}", reg_name, value);
        }
        self.epi_add4();
        self.register[modrm.rm as usize] += value;
    }

    fn add_rm32_imm8(&mut self, modrm: ModRM) {
        if modrm.mode == 0b01 {
            let (_reg, address) = self.read_effective_address_from_modrm(&modrm);
            let value = self.sign_code8(0);
            if DEBUG {
                println!("add [{:08X}],{}", address, value);
            }
            self.epi_inc();
            let temp = self.memory_u32(address) as i32;
            self.memory_set32(address, (temp + value) as u32);
        } else if modrm.mode == 0b11 {
            let reg_name = register_name(modrm.rm);
            let value = self.sign_code8(0);
            if DEBUG {
                println!("add {},{}", reg_name, value);
            }
            self.epi_inc();
            let temp = self.register(modrm.rm) as i32;
            self.register[modrm.rm as usize] = (temp + value) as u32;
        } else {
            unimplemented!();
        }
    }

    fn sub_rm32_imm32(&mut self, modrm: ModRM) {
        let reg_name = register_name(modrm.rm);
        let value = self.code32(0);
        if DEBUG {
            println!("sub {},{}", reg_name, value);
        }
        self.epi_add4();
        self.register[modrm.rm as usize] -= value;
    }

    fn sub_rm32_imm8(&mut self, modrm: ModRM) {
        if modrm.mode == 0b01 {
            let (_reg, address) = self.read_effective_address_from_modrm(&modrm);
            let value = self.sign_code8(0);
            if DEBUG {
                println!("sub [{:08X}],{}", address, value);
            }
            self.epi_inc();
            let temp = self.memory_u32(address) as i32;
            self.memory_set32(address, (temp - value) as u32);
        } else if modrm.mode == 0b11 {
            let reg_name = register_name(modrm.rm);
            let value = self.sign_code8(0);
            if DEBUG {
                println!("sub {},{}", reg_name, value);
            }
            self.epi_inc();
            let temp = self.register(modrm.rm) as i32;
            self.register[modrm.rm as usize] = (temp - value) as u32;
        }
    }

    fn opcode0f(&mut self) {
        let code = self.code8(0);
        self.epi_inc();
        if DEBUG {
            println!("opcode: {:02X}", code);
        }
        if code == 0x84 {
            self.jz_rel32();
        } else if code == 0x85 {
            self.jnz_rel32();
        } else {
            unimplemented!();
        }
    }

    fn opcode81(&mut self) {
        let modrm = self.read_modrm();
        if modrm.opcode == 0 {
            self.add_rm32_imm32(modrm);
        } else if modrm.opcode == 5 {
            self.sub_rm32_imm32(modrm);
        } else {
            unimplemented!("unknown opcode: {}", modrm.opcode);
        }
    }

    fn opcode83(&mut self) {
        let modrm = self.read_modrm();
        if modrm.opcode == 0 {
            self.add_rm32_imm8(modrm);
        } else if modrm.opcode == 4 {
            self.and_rm32_imm8(modrm)
        } else if modrm.opcode == 5 {
            self.sub_rm32_imm8(modrm);
        } else if modrm.opcode == 7 {
            self.cmp_rm32_imm8(modrm);
        } else {
            unimplemented!("unknown sub opcode: {}", modrm.opcode);
        }
    }

    fn opcodeff(&mut self) {
        let modrm = self.read_modrm();
        if modrm.opcode == 6 {
            self.push_rm32(modrm);
        } else {
            unimplemented!("unknown sub opcode: {}", modrm.opcode);
        }
    }

    fn opcoded1(&mut self) {
        let modrm = self.read_modrm();
        if modrm.opcode == 5 {
            self.shr_rm32(modrm);
        } else {
            unimplemented!("unknown sub opcode: {}", modrm.opcode);
        }
    }


    fn is_carry(&self) -> bool {
        return (self.eflags & 1) == 1;
    }

    fn is_zero(&self) -> bool {
        return (self.eflags & 1 << 6) == 1 << 6;
    }

    fn is_sign_flag(&self) -> bool {
        return (self.eflags & 1 << 7) == 1 << 7;
    }

    fn is_overflow(&self) -> bool {
        return (self.eflags & 1 << 11) == 1 << 11;
    }

    fn jz_rel8(&mut self) {
        let value = self.sign_code8(0);
        self.epi_inc();
        if DEBUG {
            println!("jz {:08X}", value);
            println!("eflags = {:032b}", self.eflags);
        }
        if self.is_zero() {
            self.jump(value);
        };
    }

    fn jz_rel32(&mut self) {
        let value = self.sign_code32(0);
        self.epi_add4();
        if DEBUG {
            println!("jz {:08X}", value);
            println!("eflags = {:032b}", self.eflags);
        }
        if self.is_zero() {
            self.jump(value);
        };
    }

    fn jnz_rel8(&mut self) {
        let value = self.sign_code8(0);
        self.epi_inc();
        if DEBUG {
            println!("jnz {:08X}", value);
            println!("eflags = {:032b}", self.eflags);
        }
        if !self.is_zero() {
            self.jump(value);
        };
    }

    fn jnz_rel32(&mut self) {
        let value = self.sign_code32(0);
        self.epi_add4();
        if DEBUG {
            println!("jnz {:08X}", value);
            println!("eflags = {:032b}", self.eflags);
        }
        if !self.is_zero() {
            self.jump(value);
        };
    }

    fn jg_rel8(&mut self) {
        let value = self.sign_code8(0);
        self.epi_inc();
        if DEBUG {
            println!("jg {:08X}", value);
            println!("eflags = {:032b}", self.eflags);
        }
        if !self.is_zero() && (self.is_sign_flag() == self.is_overflow()) {
            self.jump(value);
        };
    }

    fn jng_rel8(&mut self) {
        let value = self.sign_code8(0);
        self.epi_inc();
        if DEBUG {
            println!("jng {:08X}", value);
            println!("eflags = {:032b}", self.eflags);
        }
        if self.is_zero() || (self.is_sign_flag() != self.is_overflow()) {
            self.jump(value);
        };
    }

    fn cmp_base(&mut self, target: u32, value: u32, sign_value: i32) {
        let sign_target = target as i32;
        let (result, carry_flag) = target.overflowing_sub(value);
        if DEBUG {
            println!("result {}, {:08X}", result, result);
        }
        // CF: Carry Flag
        if carry_flag {
            if DEBUG {
                println!("carry flag");
            }
            self.eflags |= 1;
        } else {
            self.eflags &= !1;
        }
        // ZF: Zero Flag
        if result == 0 {
            if DEBUG {
                println!("zero flag");
            }
            self.eflags |= 1 << 6;
        } else {
            self.eflags &= !(1 << 6);
        }
        // SF: Sign Flag
        if (result >> 31) == 1 {
            if DEBUG {
                println!("sign flag");
            }
            self.eflags |= 1 << 7;
        } else {
            self.eflags &= !(1 << 7);
        }
        // OF: Overflow Flag
        if sign_target.checked_sub(sign_value) == None {
            if DEBUG {
                println!("overflow flag");
            }
            self.eflags |= 1 << 11;
        } else {
            self.eflags &= !(1 << 11);
        }
        if DEBUG {
            println!("eflags = {:032b}", self.eflags);
        }
    }

    fn cmp_u32_u32(&mut self, target: u32, value: u32) {
        self.cmp_base(target, value, value as i32);
    }

    fn cmp_u32_i32(&mut self, target: u32, sign_value: i32) {
        self.cmp_base(target, sign_value as u32, sign_value);
    }

    fn cmp_r32_rm32(&mut self) {
        let modrm = self.read_modrm();
        if modrm.mode == 0b01 {
            let (reg, address) = self.read_effective_address_from_modrm(&modrm);
            let reg_name = register_name(reg);
            if DEBUG {
                println!("cmp {},[{:08X}]", reg_name, address);
            }
            let target = self.register[reg as usize];
            let value = self.memory_u32(address);
            if DEBUG {
                println!("cmp {},{}", reg_name, value);
                println!("value: {}", value);
            }
            self.cmp_u32_u32(target, value);
        } else {
            unimplemented!("unknown Mod: {:02b}", modrm.mode);
        }
    }

    fn cmp_rm32_imm8(&mut self, modrm: ModRM) {
        let target: u32;
        if modrm.mode == 0b01 || modrm.mode == 0b10 {
            let (_reg, address) = self.read_effective_address_from_modrm(&modrm);
            if DEBUG {
                print!("cmp [{:08X}],", address);
            }
            target = self.memory_u32(address) as u32;
        } else if modrm.mode == 0b11 {
            let reg_name = register_name(modrm.rm);
            if DEBUG {
                print!("cmp {},", reg_name);
            }
            target = self.register[modrm.rm as usize] as u32;
        } else {
            unimplemented!("unknown Mod");
        }
        let sign_value = self.sign_code8(0) as i32;
        self.epi_inc();
        if DEBUG {
            println!("value: {}", sign_value);
        }
        self.cmp_u32_i32(target, sign_value);
    }

    fn nop(&mut self) {
        if DEBUG {
            println!("nop");
        }
    }

    fn lea(&mut self) {
        let modrm = self.read_modrm();
        if modrm.mode == 0b01 ||  modrm.mode == 0b10 {
            let (reg, address) = self.read_effective_address_from_modrm(&modrm);
            let reg_name = register_name(reg);
            if DEBUG {
                println!("lea {},[{:08X}]", reg_name, address);
            }
            self.register[reg as usize] = address;
        } else {
            unimplemented!("unknown Mod");
        }
    }

    fn xor_rm32_r32(&mut self) {
        let modrm = self.read_modrm();
        if modrm.mode == 0b11 {
            if DEBUG {
                let reg_name1 = register_name(modrm.rm);
                let reg_name2 = register_name(modrm.reg);
                println!("xor {},{}", reg_name1, reg_name2);
            }
            self.register[modrm.rm as usize] ^= self.register[modrm.reg as usize];
        } else {
            unimplemented!("unknown Mod");
        }
    }

    fn and_rm32_imm8(&mut self, modrm: ModRM) {
        if modrm.mode == 0b11 {
            let reg_name = register_name(modrm.rm);
            let value = self.sign_code8(0);
            if DEBUG {
                println!("and {},{}", reg_name, value);
            }
            self.epi_inc();
            let temp = self.register(modrm.rm) as i32;
            self.register[modrm.rm as usize] = (temp & value) as u32;
        } else {
            unimplemented!("unknown Mod");
        }
    }

    fn add_eax_imm32(&mut self) {
        let value = self.code32(0);
        if DEBUG {
            println!("add EAX,{:08X}", value);
        }
        self.register[EAX as usize] += value;
        self.epi_add4();
    }

    fn add_r32_rm32(&mut self) {
        let (reg, address) = self.read_effective_address();
        self.register[reg as usize] += self.memory_u32(address);
    }

    fn add_rm32_r32(&mut self) {
        let modrm = self.read_modrm();
        if modrm.mode == 0b01 {
            let (reg, address) = self.read_effective_address_from_modrm(&modrm);
            if DEBUG {
                let reg_name = register_name(modrm.reg);
                println!("add {:08X},{}", address, reg_name);
            }
            let result = self.memory_u32(address) + self.register[modrm.reg as usize];
            self.memory_set32(address, result);
        } else if modrm.mode == 0b11 {
            if DEBUG {
                let reg_name1 = register_name(modrm.rm);
                let reg_name2 = register_name(modrm.reg);
                println!("add {},{}", reg_name1, reg_name2);
            }
            self.register[modrm.rm as usize] += self.register[modrm.reg as usize]
        } else {
            unimplemented!("unknown Mod");
        }
    }

    fn sub_eax_imm32(&mut self) {
        let value = self.code32(0);
        if DEBUG {
            println!("sub EAX,{:08X}", value);
        }
        self.register[EAX as usize] -= value;
        self.epi_add4();
    }

    fn sub_rm32_r32(&mut self) {
        let modrm = self.read_modrm();
        if modrm.mode == 0b11 {
            if DEBUG {
                let reg_name1 = register_name(modrm.rm);
                let reg_name2 = register_name(modrm.reg);
                println!("sub {},{}", reg_name1, reg_name2);
            }
            self.register[modrm.rm as usize] -= self.register[modrm.reg as usize];
        } else {
            unimplemented!("unknown Mod");
        }
    }

    fn sub_r32_rm32(&mut self) {
        let modrm = self.read_modrm();
        if modrm.mode == 0b01 {
            let (reg, address) = self.read_effective_address_from_modrm(&modrm);
            self.register[reg as usize] -= self.memory_u32(address);
        } else {
            unimplemented!("unknown Mod");
        }
    }

    fn mov_r32_rm32(&mut self) {
        let modrm = self.read_modrm();
        if modrm.mode == 0b01 {
            let (reg, address) = self.read_effective_address_from_modrm(&modrm);
            let reg_name = register_name(reg);
            if DEBUG {
                println!("mov {},[{:#X}]", reg_name, address);
            }
            let value = self.memory_u32(address);
            if DEBUG {
                println!("value: {}", value);
            }
            self.register[reg as usize] = value;
        } else {
            unimplemented!("unknown Mod");
        }
    }

    fn mov_r32_imm32(&mut self, code: u32) {
        let reg = code - 0xb8;
        let reg_name = register_name(reg);
        let value = self.code32(0);
        if DEBUG {
            println!("mov {},{:#X}", reg_name, value);
        }
        self.register[reg as usize] = value;
        self.epi_add4();
    }

    fn mov_rm32_imm32(&mut self) {
        let (opcode, address) = self.read_effective_address();
        if opcode == 0 {
            let value = self.code32(0);
            if DEBUG {
                println!("mov [{:08X}],{:08X}", address, value);
            }
            self.epi_add4();
            self.memory_set32(address, value);
        } else {
            unimplemented!("unknown opcode: {}", opcode);
        }
    }

    fn mov_rm32_r32(&mut self) {
        let modrm = self.read_modrm();

        if modrm.mode == 0b01 {
            let value = self.register(modrm.reg);
            let (_reg, address) = self.read_effective_address_from_modrm(&modrm);
            if DEBUG {
                print!("mov [{:08X}]", address);
            }
            self.memory_set32(address, value);
        } else if modrm.mode == 0b11 {
            let reg_name1 = register_name(modrm.reg);
            let reg_name2 = register_name(modrm.rm);
            if DEBUG {
                println!("mov {},{}", reg_name2, reg_name1);
            }
            self.register[modrm.rm as usize] = self.register(modrm.reg);
        } else {
            unimplemented!("unknown Mod");
        }
    }

    fn call_rel32(&mut self) {
        let value = self.sign_code32(0);
        if DEBUG {
            println!("call {:08X}", value);
        }
        self.push32(self.eip as u32 + 4);
        self.jump(4 + value);
    }

    pub fn launch(&mut self) -> Result<(), ()> {
        println!("--- START ---");
        loop {
            if DEBUG {
                println!("EIP: {:08X}", self.eip);
            }

            let code = self.code8(0);
            self.epi_inc();

            if DEBUG {
                println!("opcode: {:02X}", code);
            }

            if code == 0x01 {
                self.add_rm32_r32();
            } else if code == 0x03 {
                self.add_r32_rm32();
            } else if code == 0x05 {
                self.add_eax_imm32();
            } else if code == 0x0f {
                self.opcode0f();
            } else if code == 0x29 {
                self.sub_rm32_r32();
            } else if code == 0x2b {
                self.sub_r32_rm32();
            } else if code == 0x2d {
                self.sub_eax_imm32();
            } else if code == 0x31 {
                self.xor_rm32_r32();
            } else if code == 0x3b {
                self.cmp_r32_rm32();
            } else if (0x50 <= code) && (code <= (0x50 + 7)) {
                self.push_r32(code);
            } else if (0x58 <= code) && (code <= (0x58 + 7)) {
                self.pop_r32(code);
            } else if code == 0x6a {
                self.push_imm8();
            } else if code == 0x74 {
                self.jz_rel8();
            } else if code == 0x75 {
                self.jnz_rel8();
            } else if code == 0x7e {
                self.jng_rel8();
            } else if code == 0x7f {
                self.jg_rel8();
            } else if code == 0x81 {
                self.opcode81();
            } else if code == 0x83 {
                self.opcode83();
            } else if code == 0x89 {
                self.mov_rm32_r32();
            } else if code == 0x8b {
                self.mov_r32_rm32();
            } else if code == 0x8d {
                self.lea();
            } else if code == 0x90 {
                self.nop();
            } else if code == 0xd1 {
                self.opcoded1();
            } else if code == 0xff {
                self.opcodeff();
            } else if code == 0xc9 {
                self.leave();
            } else if code == 0xc7 {
                self.mov_rm32_imm32();
            } else if code == 0xeb {
                self.jump_short();
            } else if code == 0xe8 {
                self.call_rel32();
            } else if (0xb8 <= code) && (code <= (0xb8 + 7)) {
                self.mov_r32_imm32(code);
            } else if code == 0xc3 {
                if DEBUG {
                    println!("ret");
                }
                let address = self.pop32();
                if DEBUG {
                    println!("ret => address: {:08X}", address);
                }
                if address == 0 {
                    println!("--- EXIT ---");
                    return Ok(());
                } else {
                    self.eip = address;
                }
            } else {
                unimplemented!("unknown code: {:02X}", code);
            }
            // self.dump_register();
            if DEBUG {
                println!("---");
            }
        }
        panic!("main loop ends without ret");
    }

    pub fn dump_memory(&self) {
        for i in 0..20 {
            let index = (4 * i) as usize;
            let mut data: String = String::new();
            for j in 0..4 {
                let str1 = format!("{:02X}", self.memory[index+j]);
                data.push_str(&str1);
            }
            println!("{:08X} : {}", index, data);
        }
        println!("---");
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
            let reg_name = register_name(i as u32);
            let value = self.register[i];
            println!("{} = {:#010X} {}", reg_name, value, value);
        }
    }
}

#[cfg(test)]
mod tests {
    const TEST_MEMSIZE: u32 = 1024;
    use super::Emulator;

    #[test]
    fn emulator_new() {
        let mut emu = Emulator::new(TEST_MEMSIZE);
        assert_eq!(emu.eip, 0);
        assert_eq!(emu.eflags, 0);
    }

    #[test]
    fn cmp_u32_u32() {
        let mut emu = Emulator::new(TEST_MEMSIZE);
        emu.cmp_u32_u32(0xffff, 0xffff);
        assert_eq!(emu.is_zero(), true);
        assert_eq!(emu.is_sign_flag(), false);
        assert_eq!(emu.is_carry(), false);
        assert_eq!(emu.is_overflow(), false);
    }

    #[test]
    fn cmp_u32_i32() {
        let mut emu = Emulator::new(TEST_MEMSIZE);
        emu.cmp_u32_i32(0xffffffff, -1);
        assert_eq!(emu.is_zero(), true);
        assert_eq!(emu.is_sign_flag(), false);
        assert_eq!(emu.is_carry(), false);
        assert_eq!(emu.is_overflow(), false);
    }

    #[test]
    fn cmp_zero() {
        let mut emu = Emulator::new(TEST_MEMSIZE);
        emu.cmp_u32_u32(0xffff, 0xffff);
        assert_eq!(emu.is_zero(), true);
        assert_eq!(emu.is_sign_flag(), false);
        assert_eq!(emu.is_carry(), false);
        assert_eq!(emu.is_overflow(), false);
    }

    #[test]
    fn cmp_carry() {
        let mut emu = Emulator::new(TEST_MEMSIZE);
        emu.cmp_u32_u32(0, 1);
        assert_eq!(emu.is_zero(), false);
        assert_eq!(emu.is_sign_flag(), true);
        assert_eq!(emu.is_carry(), true);
        assert_eq!(emu.is_overflow(), false);
    }

    #[test]
    fn cmp_overflow() {
        let mut emu = Emulator::new(TEST_MEMSIZE);
        emu.cmp_u32_u32(0x80000000, 1);
        assert_eq!(emu.is_zero(), false);
        assert_eq!(emu.is_sign_flag(), false);
        assert_eq!(emu.is_carry(), false);
        assert_eq!(emu.is_overflow(), true);
    }

    #[test]
    fn eflags_carry() {
        let mut emu = Emulator::new(TEST_MEMSIZE);
        emu.eflags = 1;
        assert_eq!(emu.is_zero(), false);
        assert_eq!(emu.is_sign_flag(), false);
        assert_eq!(emu.is_carry(), true);
        assert_eq!(emu.is_overflow(), false);
    }

    #[test]
    fn eflags_zero() {
        let mut emu = Emulator::new(TEST_MEMSIZE);
        emu.eflags = 1 << 6;
        assert_eq!(emu.is_zero(), true);
        assert_eq!(emu.is_sign_flag(), false);
        assert_eq!(emu.is_carry(), false);
        assert_eq!(emu.is_overflow(), false);
    }

    #[test]
    fn eflags_sign() {
        let mut emu = Emulator::new(TEST_MEMSIZE);
        emu.eflags = 1 << 7;
        assert_eq!(emu.is_zero(), false);
        assert_eq!(emu.is_sign_flag(), true);
        assert_eq!(emu.is_carry(), false);
        assert_eq!(emu.is_overflow(), false);
    }

    #[test]
    fn eflags_overflow() {
        let mut emu = Emulator::new(TEST_MEMSIZE);
        emu.eflags = 1 << 11;
        assert_eq!(emu.is_zero(), false);
        assert_eq!(emu.is_sign_flag(), false);
        assert_eq!(emu.is_carry(), false);
        assert_eq!(emu.is_overflow(), true);
    }
}