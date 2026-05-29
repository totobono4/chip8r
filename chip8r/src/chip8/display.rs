pub struct Display {
    matrix: [[bool; 32]; 64],
}

impl Display {
    pub fn new() -> Self {
        Self {
            matrix: [[false; 32]; 64],
        }
    }
}

struct Sprite {
    width: u8,
    height: u8,
}
