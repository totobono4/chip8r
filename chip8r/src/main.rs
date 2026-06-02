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

    #[arg(long, default_value_t = false, help = "start with original chip8 quirks (vf-reset+memory+clipping)")]
    chip8: bool,

    #[arg(short, long, default_value_t = false, help = "quirk: jumping resets the VF flag in instructions 8XY1, 8XY2, 8XY3.")]
    vf_reset: bool,
    #[arg(short, long, default_value_t = false, help = "quirk: I gets incremented by X in instructions FX55, FX65.")]
    memory: bool,
    #[arg(short, long, default_value_t = false, help = "quirk: sprites are clipping instead of wrapping at screen bottom.")]
    clipping: bool,
    #[arg(short, long, default_value_t = false, help = "quirk: VX shifts itself instead of shifting from VY in instructions 8XY6 and 8XYE.")]
    shifting: bool,
    #[arg(short, long, default_value_t = false, help = "quirk: jumps to NNN + VX instead of NNN + V0 in instruction BNNN.")]
    jumping: bool,
}

fn main() {
    let args = Args::parse();
    let rom = fs::read(args.rom).unwrap();

    let mut vf_reset = args.vf_reset;
    let mut memory = args.memory;
    let mut clipping = args.clipping;
    let shifting = args.shifting;
    let jumping = args.jumping;

    if args.chip8 {
        vf_reset = true;
        memory = true;
        clipping = true;
    }

    let mut chip8 = chip8::Chip8::new(vf_reset, memory, clipping, shifting, jumping);
    chip8._write_arbitrary_byte(0x1FF, 0x1);

    chip8.load_rom(rom);
    app::run(chip8);
}
