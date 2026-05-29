use std::{thread, time};

use winit::keyboard::NamedKey::End;

mod memory;
mod registers;
mod cpu;
mod keyboard;
mod display;

pub struct Chip8 {
    memory: memory::Memory,
    registers: registers::Registers,
    cpu: cpu::Cpu,
    keyboard: keyboard::Keyboard,
    display: display::Display,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: memory::Memory::new(),
            registers: registers::Registers::new(),
            cpu: cpu::Cpu::new(),
            keyboard: keyboard::Keyboard::new(),
            display: display::Display::new(),
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.memory.load_rom(rom);
    }

    pub fn launch(&mut self, frequency: f64) {
        loop {
            let start_time = time::Instant::now();

            // Insert Chip8 Code.

            let end_time = time::Instant::now();
            let elapsed = end_time - start_time;
            thread::sleep(time::Duration::from_secs_f64(frequency - elapsed.as_secs_f64()));
        }
    }
}
