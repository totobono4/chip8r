use std::ptr::replace;

const FONT: [u8; 80] = [
    0x60,0x90,0x90,0x90,0x60, // 0
    0x20,0x20,0x20,0x20,0x20, // 1
    0x60,0x90,0x20,0x40,0xF0, // 2
    0xE0,0x10,0x60,0x10,0xE0, // 3
    0x20,0x60,0xF0,0x20,0x20, // 4
    0xF0,0x80,0xE0,0x10,0xE0, // 5
    0x60,0x80,0xE0,0x90,0x60, // 6
    0xF0,0x10,0x20,0x40,0x80, // 7
    0x60,0x90,0x60,0x90,0x60, // 8
    0x60,0x90,0x70,0x10,0x60, // 9
    0x60,0x90,0xF0,0x90,0x90, // A
    0xE0,0x90,0xE0,0x90,0xE0, // B
    0x70,0x80,0x80,0x80,0x70, // C
    0xE0,0x90,0x90,0x90,0xE0, // D
    0xF0,0x80,0xE0,0x80,0xF0, // E
    0xF0,0x80,0xE0,0x80,0x80, // F
];

pub struct Memory {
    ram: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        let mut memory = Self {
            ram: [0x00; 4096],
        };

        memory.load_font();
        return memory;
    }

    fn load_font(&mut self) {
        self.load_data(0x00, FONT.to_vec());
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.load_data(0x200, rom);
    }

    fn load_data(&mut self, start: usize, data: Vec<u8>) {
        let size = data.len();
        let end = start + size;
        for index in 0x00..size {
            self.ram[start + index] = data[index];
        }

        println!("Added some data to the rom between 0x{:02X} and 0x{:02X} adresses:", start, end);
        self._debug(start, end);
    }

    fn _debug(&self, start: usize, end: usize) {
        if start >= self.ram.len() || end >= self.ram.len() { return; }
        let byte_start = start & 0xFF0;
        let byte_end = end + 0x00F & 0xFF0;
        print!("mem: 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F");
        for index in byte_start..byte_end {
            if index < byte_start { continue; }
            if index >= byte_end { continue; }
            if index % 0x010 == 0x000 { print!("\n{:03X}:", index); }
            if index < start || index >= end { print!("   "); continue; }
            print!(" {:02X}", self.ram[index]);
        }
        println!();
    }
}
