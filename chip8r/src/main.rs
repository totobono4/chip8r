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
    memory: bool,
    #[arg(short, long, default_value_t = false)]
    clipping: bool,
    #[arg(short, long, default_value_t = false)]
    shifting: bool,
    #[arg(short, long, default_value_t = false)]
    jumping: bool,
}

fn main() {
    let args = Args::parse();
    let rom = fs::read(args.rom).unwrap();

    let vf_reset = args.vf_reset;
    let memory = args.memory;
    let clipping = args.clipping;
    let shifting = args.shifting;
    let jumping = args.jumping;

    let mut chip8 = chip8::Chip8::new(vf_reset, memory, clipping, shifting, jumping);
    chip8._write_arbitrary_byte(0x1FF, 0x1);

    chip8.load_rom(rom);
    app::run(chip8);
}
