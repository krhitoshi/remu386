use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;

mod emulator;

enum Register {
    EAX = 0, ECX = 1, EDX = 2, EBX =3,
    ESP = 4, EBP = 5, ESI = 6, EDI = 7
}

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
    emu.launch();
    emu.dump_register();
}
