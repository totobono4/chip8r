use crate::consts;
use simply_colored::*;

pub struct Memory {
    ram: [u8; consts::RAM_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        let mut memory = Self {
            ram: [0x00; consts::RAM_SIZE],
        };

        memory.load_font();
        memory
    }

    pub fn get_opcode(&mut self, program_counter : u16, opcode_size: usize) -> u16 {
        let opcode_datas = self.get_data(program_counter as usize, opcode_size).clone();
        u16::from_be_bytes([opcode_datas[0], opcode_datas[1]])
    }

    pub fn get_data(&mut self, adress: usize, size: usize) -> Vec<u8> {
        let start = adress;
        let end = start + size;
        let data = self.ram[start..end].to_vec();
        data
    }

    fn load_font(&mut self) {
        self.load_data(0x00, consts::FONT.to_vec());
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.load_data(consts::PROGRAM_START_ADDRESS as usize, rom);
    }

    pub fn load_data(&mut self, start: usize, data: Vec<u8>) {
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
        print!("{BLUE}{BOLD}mem: 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F{RESET}");
        for index in byte_start..byte_end {
            if index < byte_start { continue; }
            if index >= byte_end { continue; }
            if index % 0x010 == 0x000 { print!("{GREEN}{BOLD}\n{:03X}{RESET}:", index); }
            if index < start || index >= end { print!("   "); continue; }
            print!("{YELLOW} {:02X}{RESET}", self.ram[index]);
        }
        println!();
    }
}
