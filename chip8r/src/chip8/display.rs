use simply_colored::*;
use crate::consts;

pub struct Display {
    matrix: [[bool; consts::DISPLAY_HEIGHT]; consts::DISPLAY_WIDTH],
}

impl Display {
    pub fn new() -> Self {
        Self {
            matrix: [[false; consts::DISPLAY_HEIGHT]; consts::DISPLAY_WIDTH],
        }
    }

    pub fn get_display_buffer(&mut self) -> [[u8; 4]; consts::DISPLAY_HEIGHT * consts::DISPLAY_WIDTH] {
        let mut display_buffer = [[0; 4]; consts::DISPLAY_HEIGHT * consts::DISPLAY_WIDTH];

        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[i].len() {
                if self.matrix[i][j] { display_buffer[j*consts::DISPLAY_WIDTH+i] = consts::DISPLAY_COLOR_1; }
                else { display_buffer[j*consts::DISPLAY_WIDTH+i] = consts::DISPLAY_COLOR_0; }
            }
        }

        display_buffer
    }

    pub fn set_sprite(&mut self, x: usize, y: usize, sprite: Vec<u8>) {
        for index in 0..sprite.len() {
            let byte = sprite[index];

            self.set_pixel((x+0)%consts::DISPLAY_WIDTH, (y+index)%consts::DISPLAY_HEIGHT, (byte & 0x80) == 0x80);
            self.set_pixel((x+1)%consts::DISPLAY_WIDTH, (y+index)%consts::DISPLAY_HEIGHT, (byte & 0x40) == 0x40);
            self.set_pixel((x+2)%consts::DISPLAY_WIDTH, (y+index)%consts::DISPLAY_HEIGHT, (byte & 0x20) == 0x20);
            self.set_pixel((x+3)%consts::DISPLAY_WIDTH, (y+index)%consts::DISPLAY_HEIGHT, (byte & 0x10) == 0x10);
            self.set_pixel((x+4)%consts::DISPLAY_WIDTH, (y+index)%consts::DISPLAY_HEIGHT, (byte & 0x08) == 0x08);
            self.set_pixel((x+5)%consts::DISPLAY_WIDTH, (y+index)%consts::DISPLAY_HEIGHT, (byte & 0x04) == 0x04);
            self.set_pixel((x+6)%consts::DISPLAY_WIDTH, (y+index)%consts::DISPLAY_HEIGHT, (byte & 0x02) == 0x02);
            self.set_pixel((x+7)%consts::DISPLAY_WIDTH, (y+index)%consts::DISPLAY_HEIGHT, (byte & 0x01) == 0x01);
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
        self.matrix[x][y] |= value;
    }

    pub fn clear(&mut self) {
        self.matrix = [[false; consts::DISPLAY_HEIGHT]; consts::DISPLAY_WIDTH];
    }
}
