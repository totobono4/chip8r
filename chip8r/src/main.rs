use std::fs;

use clap::Parser;

mod app;
mod chip8;
mod consts;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    rom: String,

    #[arg(short, long, default_value_t = false)]
    vf_reset: bool,
    #[arg(short, long, default_value_t = false)]
    legacy_memory: bool
}

fn main() {
    let args = Args::parse();
    let rom = fs::read(args.rom).unwrap();

    let vf_reset = args.vf_reset;
    let legacy_memory = args.legacy_memory;

    let mut chip8 = chip8::Chip8::new(vf_reset, legacy_memory);
    chip8._write_arbitrary_byte(0x1FF, 0x1);

    chip8.load_rom(rom);
    app::run(chip8);
}
