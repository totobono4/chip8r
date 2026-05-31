pub struct Keyboard {
    keys: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keys: [false; 16],
        }
    }

    pub fn set_key(&mut self, key: usize, is_pressed: bool) {
        self.keys[key] = is_pressed;
    }

    pub fn is_key_pressed(&mut self, key: usize) -> bool {
        self.keys[key]
    }
}
