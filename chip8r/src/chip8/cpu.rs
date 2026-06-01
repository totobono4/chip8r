use simply_colored::*;

use crate::consts;
use crate::chip8::memory;
use crate::chip8::keyboard;
use crate::chip8::display;
use crate::chip8::audio;

pub struct Cpu {
    pc: u16,
    sp: u8,
    stack: [u16; 16],

    v: [u8; 0x10],
    i: u16,
    
    dt: u8,
    st: u8,

    pc_counting: bool,
    vf_reset: bool,
    memory: bool,
    clipping: bool,
    shifting: bool,
    jumping: bool,
}

impl Cpu {
    pub fn new(vf_reset: bool, memory: bool, clipping: bool, shifting: bool, jumping: bool) -> Self {
        Self {
            pc: consts::PROGRAM_START_ADDRESS, // program_counter
            sp: 0, // stack_pointer
            stack: [0; 16],

            v: [0; 0x10],
            i: 0,

            dt: 0, // delay_timer
            st: 0, // sound_timer

            pc_counting: true,

            vf_reset,
            memory,
            clipping,
            shifting,
            jumping,
        }
    }

    pub fn update(&mut self, memory: &mut memory::Memory, display: &mut display::Display, keyboard: &mut keyboard::Keyboard, audio: &mut audio::Audio,) {
        self.reg_update();
        self.handle_audio(audio);

        if self.pc%2 != 0 { self.pc += 1; }
        let opcode = memory.get_opcode(self.pc, consts::OPCODE_SIZE);

        // println!("pc:[{:02X?}] v:[{:02X?}] i:[{:03X?}] opcode:[{:04X?}]", self.pc, self.v, self.i, opcode);
        self.process_opcode(opcode, memory, keyboard, display);
        if self.pc_counting { self.pc += 2; }
        self.pc_counting = true;
    }

    fn process_opcode(&mut self, opcode: u16, memory: &mut memory::Memory, keyboard: &mut keyboard::Keyboard, display: &mut display::Display) {
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
                        self.sp -= 1;
                        self.pc = self.stack[self.sp as usize];
                    }
                    _ => { Self::not_implemented_opcode(opcode); }
                }
            }
            0x1 => {
                self.pc = nnn;
                self.pc_counting = false;
            }
            0x2 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
                self.pc_counting = false;
            }
            0x3 => {
                if self.v[x] != kk { return; }
                self.pc += 2;
            }
            0x4 => {
                if self.v[x] == kk { return; }
                self.pc += 2;
            }
            0x5 => {
                if self.v[x] != self.v[y] { return; }
                self.pc += 2;
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
                        if self.vf_reset { self.v[0xF] = 0x0; }
                    }
                    0x2 => {
                        self.v[x] &= self.v[y];
                        if self.vf_reset { self.v[0xF] = 0x0; }
                    }
                    0x3 => {
                        self.v[x] ^= self.v[y];
                        if self.vf_reset { self.v[0xF] = 0x0; }
                    }
                    0x4 => {
                        let res: u16 = self.v[x] as u16 + self.v[y] as u16;
                        self.v[x] = res as u8;
                        self.v[0xF] = (res >> 8) as u8;
                    }
                    0x5 => {
                        let res = self.v[x] as i16 - self.v[y] as i16;
                        self.v[x] = res as u8;
                        if res < 0 { self.v[0xF] = 0; }
                        else { self.v[0xF] = 1; }
                    }
                    0x6 => {
                        if !self.shifting { self.v[x] = self.v[y]; }
                        let flag = self.v[x] & 0x01;
                        self.v[x] >>= 1;
                        self.v[0xF] = flag;
                    }
                    0x7 => {
                        let res = self.v[y] as i16 - self.v[x] as i16;
                        self.v[x] = res as u8;
                        if res < 0 { self.v[0xF] = 0; }
                        else { self.v[0xF] = 1; }
                    }
                    0xE => {
                        if !self.shifting { self.v[x] = self.v[y]; }
                        let flag = self.v[x] >> 7;
                        self.v[x] <<= 1;
                        self.v[0xF] = flag;
                    }
                    _ => { Self::not_implemented_opcode(opcode); }
                }
            }
            0x9 => {
                if self.v[x] == self.v[y] { return; }
                self.pc += 2;
            }
            0xA => {
                self.i = nnn;
            }
            0xB => {
                if !self.jumping { self.pc = nnn + self.v[0x0] as u16; }
                else { self.pc = nnn + self.v[x] as u16; }
                self.pc_counting = false;
            }
            0xC => {
                self.v[x] = rand::random_range(0x0..=0xFF) & kk;
            }
            0xD => {
                let sprite = memory.get_data(self.i as usize, n as usize);
                let x_coord = self.v[x] as usize % consts::DISPLAY_WIDTH;
                let y_coord = self.v[y] as usize % consts::DISPLAY_HEIGHT;
                if display.draw_sprite(x_coord, y_coord, sprite, self.clipping) { self.v[0xF] = 1; }
                else { self.v[0xF] = 0; }
            }
            0xE => {
                match (opcode & 0xFF) as u8 {
                    0x9E => {
                        if !keyboard.is_key_pressed(self.v[x] as usize) { return; }
                        self.pc += 2;
                    }
                    0xA1 => {
                        if keyboard.is_key_pressed(self.v[x] as usize) { return; }
                        self.pc += 2;
                    }
                    _ => { Self::not_implemented_opcode(opcode); }
                }
            }
            0xF => {
                match (opcode & 0xFF) as u8 {
                    0x07 => {
                        self.v[x] = self.dt;
                    }
                    0x0A => {
                        let mut key_pressed = false;
                        for i in 0x0..0xF {
                            if !keyboard.is_key_pressed(i) { continue; }
                            key_pressed = true;
                            self.v[x] = i as u8;
                        }
                        if !key_pressed { self.pc_counting = false; }
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
                        memory.write_data(self.i as usize, [(dec/100)%10, (dec/10)%10, dec%10].to_vec());
                    }
                    0x55 => {
                        memory.write_data(self.i as usize, self.v[0..=x].to_vec());
                        if self.memory { self.i += x as u16 +1; }
                    }
                    0x65 => {
                        let vec_data = memory.get_data(self.i as usize, x+1);
                        for index in 0..=x {
                            self.v[index] = vec_data[index];
                        }
                        if self.memory { self.i += x as u16 +1; }
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
