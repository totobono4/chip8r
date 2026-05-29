use std::fs;

use clap::Parser;

mod window;
mod chip8;

const FREQUENCY: f64 = 1./60.;

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
    chip8.launch(FREQUENCY);
}
