use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod emulator;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: {} FILE", args[0])
    }

    let mut emu = emulator::Emulator::new(emulator::MEMORY_SIZE);

    let path = Path::new(&args[1]);
    let mut f: std::fs::File = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(f) => f,
    };

    let size = match f.read(&mut emu.memory) {
        Err(why) => panic!("couldn't read binary file: {}", why.description()),
        Ok(size) => size,
    };
    println!("loaded memory size: {} B", size);

    // emu.load_memory(&mut f);
    emu.launch();
    emu.dump_register();
    emu.dump_memory();
}
