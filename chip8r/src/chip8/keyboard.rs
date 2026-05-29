pub struct Keyboard {
    keys: [u8; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keys: [0; 16],
        }
    }
}
