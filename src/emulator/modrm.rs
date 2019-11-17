#[derive(Debug)]
pub struct ModRM {
    pub mode: u32,
    pub reg: u32,
    pub rm: u32,
    pub opcode: u32
}

impl ModRM {
    pub fn new(code: u32) -> Self {
        let mut modrm = Self {
            mode: 0,
            reg: 0,
            rm: 0,
            opcode: 0
        };

        println!("ModR/M: {:02X} {:#8b}", code, code);
        let mod_mask = 0b11000000;
        modrm.mode = (code & mod_mask) >> 6;

        let reg_mask = 0b00111000;
        modrm.reg = (code & reg_mask) >> 3;
        modrm.opcode = modrm.reg;

        let rm_mask = 0b00000111;
        modrm.rm = code & rm_mask;

        // let reg_name1 = self.register_name(modrm.reg);
        // let reg_name2 = self.register_name(modrm.rm);

        // println!("Mod: {:02b}, REG: {:03b} (opcode: {}, {}), R/M: {:03b} ({})",
        //          modrm.mode, modrm.reg, modrm.opcode, reg_name1,
        //          modrm.rm, reg_name2);
        println!("ModR/M: {:?}", modrm);
        println!("Mod: {:02b}, REG: {:03b} (opcode: {}), R/M: {:03b}",
                 modrm.mode, modrm.reg, modrm.opcode, modrm.rm);

        return modrm;
    }
}

#[cfg(test)]
mod tests {
    use super::ModRM;
    #[test]
    fn modrm_new() {
        let modrm = ModRM::new(0x61);
        assert_eq!(modrm.mode, 0b01);
        assert_eq!(modrm.reg, 0b100);
        assert_eq!(modrm.rm, 0b001);
    }
}