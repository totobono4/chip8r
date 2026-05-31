use std::fs;

use clap::Parser;

mod app;
mod chip8;
mod consts;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    rom: String
}

fn main() {
    let args = Args::parse();
    let rom = fs::read(args.rom).unwrap();

    let mut chip8 = chip8::Chip8::new();
    chip8.load_rom(rom);
    app::run(chip8);
}
