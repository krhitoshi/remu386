// 1MB 0x00000 - 0xfffff
pub const MEMORY_SIZE: u32 = 1024 * 1024;

pub struct Emulator {
    pub memory: [u8; MEMORY_SIZE as usize],
    pub eip: usize,
    pub register: [u32; 8]
}