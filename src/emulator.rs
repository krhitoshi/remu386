use std::fs::File;
use std::io::Read;

// 1MB 0x00000 - 0xfffff
pub const MEMORY_SIZE: u32 = 1024 * 1024;

pub struct Emulator {
    pub memory: [u8; MEMORY_SIZE as usize],
    pub eip: usize,
    pub register: [u32; 8]
}

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

    pub fn epi_add4(&mut self) {
        self.eip += 4;
    }

    pub fn epi_inc(&mut self) {
        self.eip += 1;
    }
}