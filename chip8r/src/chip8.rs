mod memory;
mod cpu;
mod keyboard;
mod display;
mod audio;

use crate::consts;

pub struct Chip8 {
    memory: memory::Memory,
    cpu: cpu::Cpu,
    keyboard: keyboard::Keyboard,
    display: display::Display,
    audio: audio::Audio,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: memory::Memory::new(),
            cpu: cpu::Cpu::new(),
            keyboard: keyboard::Keyboard::new(),
            display: display::Display::new(),
            audio: audio::Audio::new(),
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.memory.load_rom(rom);
    }

    pub fn tick(&mut self) {
        self.cpu.update(&mut self.memory, &mut self.audio, &mut self.display);
    }

    pub fn get_display_buffer(&mut self) -> [[u8; 4]; consts::DISPLAY_HEIGHT * consts::DISPLAY_WIDTH] {
        self.display.get_display_buffer()
    }
}
