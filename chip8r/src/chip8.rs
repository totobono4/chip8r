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
    pub fn new(vf_reset: bool, memory: bool, clipping: bool, shifting: bool, jumping: bool) -> Self {
        Self {
            memory: memory::Memory::new(),
            cpu: cpu::Cpu::new(vf_reset, memory, clipping, shifting, jumping),
            keyboard: keyboard::Keyboard::new(),
            display: display::Display::new(),
            audio: audio::Audio::new(),
        }
    }

    pub fn _write_arbitrary_byte(&mut self, address: usize, byte: u8) {
        self.memory.write_byte(address, byte);
        self.memory._debug(address, address+1);
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.memory.load_rom(rom);
    }

    pub fn tick(&mut self) {
        self.cpu.update(&mut self.memory, &mut self.display, &mut self.keyboard, &mut self.audio);
    }

    pub fn has_drawn(&mut self) -> bool {
        self.cpu.has_drawn
    }

    pub fn get_display_buffer(&mut self) -> [[u8; 4]; consts::DISPLAY_HEIGHT * consts::DISPLAY_WIDTH] {
        self.display.get_display_buffer()
    }

    pub fn set_key(&mut self, key: usize, is_pressed: bool) {
        self.keyboard.set_key(key, is_pressed);
    }
}
