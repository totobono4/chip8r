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

    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: Vec<u8>, clipping: bool) -> bool {
        let mut overlapped = false;
        for i in 0..sprite.len() {
            let byte = sprite[i];
            for j in 0..8 {
                let mut x_coord = x+j;
                let mut y_coord = y+i;
                if !clipping {
                    x_coord %= consts::DISPLAY_WIDTH;
                    y_coord %= consts::DISPLAY_HEIGHT;
                }
                if x_coord >= consts::DISPLAY_WIDTH { continue; }
                if y_coord >= consts::DISPLAY_HEIGHT { continue; }
                overlapped |= self.draw_pixel(x_coord, y_coord, (byte>>(7-j))%2==1);
            }
        }
        overlapped
    }

    fn draw_pixel(&mut self, x: usize, y: usize, value: bool) -> bool {
        let overlapped = self.matrix[x][y] & value;
        self.matrix[x][y] ^= value;
        overlapped
    }

    pub fn clear(&mut self) {
        self.matrix = [[false; consts::DISPLAY_HEIGHT]; consts::DISPLAY_WIDTH];
    }
}
