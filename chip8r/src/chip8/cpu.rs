use simply_colored::*;
use rand::prelude::*;

use crate::consts;
use crate::chip8::memory;
use crate::chip8::audio;
use crate::chip8::display;

pub struct Cpu {
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; 16],

    v: [u8; 0x10],
    i: u16,
    
    dt: u8,
    st: u8,

    jumped: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            program_counter: consts::PROGRAM_START_ADDRESS,
            stack_pointer: 0,
            stack: [0; 16],

            v: [0; 0x10],
            i: 0,

            dt: 0, // delay_timer
            st: 0, // sound_timer

            jumped: false,
        }
    }

    pub fn update(&mut self, memory: &mut memory::Memory, audio: &mut audio::Audio, display: &mut display::Display) {
        self.reg_update();
        self.handle_audio(audio);

        if self.program_counter%2 != 0 { self.program_counter += 1; }
        let opcode = memory.get_opcode(self.program_counter, consts::OPCODE_SIZE);

        println!("pc:[{:02X?}] v:[{:02X?}] i:[{:03X?}] opcode:[{:04X?}]", self.program_counter, self.v, self.i, opcode);
        self.process_opcode(opcode, memory, display);
        if !self.jumped { self.program_counter += 2; }
        self.jumped = false;
    }

    fn process_opcode(&mut self, opcode: u16, memory: &mut memory::Memory, display: &mut display::Display) {
        let nnn = opcode & 0x0FFF;
        let n = (opcode & 0x000F) as u8;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let kk = (opcode & 0x00FF) as u8;

        match ((opcode & 0xF000) >> 12) as u8 {
            0x0 => {
                match (opcode & 0x00FF) as u8 {
                    0xE0 => {
                        display.clear();
                    }
                    0xEE => {
                        self.stack_pointer -= 1;
                        self.program_counter = self.stack[self.stack_pointer as usize];
                        self.jumped = true;
                    }
                    _ => { Self::not_implemented_opcode(opcode); }
                }
            }
            0x1 => {
                self.program_counter = nnn;
                self.jumped = true;
            }
            0x2 => {
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.stack_pointer += 1;
                self.program_counter = nnn;
                self.jumped = true;
            }
            0x3 => {
                if self.v[x] != kk { return; }
                self.program_counter += 2;
            }
            0x4 => {
                if self.v[x] == kk { return; }
                self.program_counter += 2;
            }
            0x5 => {
                if self.v[x] != self.v[y] { return; }
                self.program_counter += 2;
            }
            0x6 => {
                self.v[x] = kk;
            }
            0x7 => {
                self.v[x] = (self.v[x] as u16 + kk as u16) as u8;
            }
            0x8 => {
                match (opcode & 0x000F) as u8 {
                    0x0 => {
                        self.v[x] = self.v[y];
                    }
                    0x1 => {
                        self.v[x] |= self.v[y];
                    }
                    0x2 => {
                        self.v[x] &= self.v[y];
                    }
                    0x3 => {
                        self.v[x] ^= self.v[y];
                    }
                    0x4 => {
                        let res: u16 = self.v[x] as u16 + self.v[y] as u16;
                        self.v[x] = res as u8;
                        self.v[0xF] = (res >> 8) as u8;
                    }
                    0x5 => {
                        let cond = self.v[x] > self.v[y];
                        if cond { self.v[x] -= self.v[y]; }
                        else { self.v[y] -= self.v[x]; }
                        self.v[0xF] = cond as u8;
                    }
                    0x6 => {
                        self.v[0xF] = self.v[x] & 0x01;
                        self.v[x] >>= 1;
                    }
                    0x7 => {
                        let cond = self.v[y] > self.v[x];
                        if cond { self.v[y] -= self.v[x]; }
                        else { self.v[x] -= self.v[y]; }
                        self.v[0xF] = cond as u8;
                    }
                    0xE => {
                        self.v[0xF] = self.v[x] >> 7;
                        self.v[x] <<= 1;
                    }
                    _ => { Self::not_implemented_opcode(opcode); }
                }
            }
            0x9 => {
                if self.v[x] == self.v[y] { return; }
                self.program_counter += 2;
            }
            0xA => {
                self.i = nnn;
            }
            0xB => {
                self.program_counter = nnn + self.v[0x0] as u16;
                self.jumped = true;
            }
            0xC => {
                self.v[x] = rand::random_range(0x0..=0xFF) & kk;
            }
            0xD => {
                let sprite = memory.get_data(self.i as usize, n as usize);
                display.set_sprite(self.v[x] as usize, self.v[y] as usize, sprite);
            }
            0xE => {
                match (opcode & 0x0F) as u8 {
                    _ => { Self::not_implemented_opcode(opcode); }
                }
            }
            0xF => {
                match (opcode & 0xFF) as u8 {
                    0x07 => {
                        self.v[x] = self.dt;
                    }
                    0x0A => {
                        Self::not_implemented_opcode(opcode);
                    }
                    0x15 => {
                        self.dt = self.v[x];
                    }
                    0x18 => {
                        self.st = self.v[x];
                    }
                    0x1E => {
                        self.i += self.v[x] as u16;
                    }
                    0x29 => {
                        self.i = self.v[x] as u16;
                    }
                    0x33 => {
                        let dec = self.v[x];
                        memory.load_data(self.i as usize, [dec/100%10, dec/10%10, dec%10].to_vec());
                    }
                    0x55 => {
                        memory.load_data(self.i as usize, self.v[0..=x].to_vec());
                    }
                    0x65 => {
                        let vec_data = memory.get_data(self.i as usize, x);
                        for index in 0..x {
                            self.v[index] = vec_data[index];
                        }
                    }
                    _ => { Self::not_implemented_opcode(opcode); }
                }
            }
            _ => { Self::not_implemented_opcode(opcode); }
        }
    }

    fn not_implemented_opcode(opcode: u16) {
        println!("{RED}Instruction {:04X?} not implemented.{RESET}", opcode)
    }

    fn reg_update(&mut self) {
        if self.dt > 0 { self.dt -= 1; }
        if self.st > 0 { self.st -= 1; }
    }

    fn handle_audio(&mut self, audio: &mut audio::Audio) {
        if self.st > 0 { audio.play_sound(); }
        else { audio.stop_sound(); }
    }
}
