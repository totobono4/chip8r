pub struct Registers {
    v: [u8; 16],
    i: u16,

    delay_timer: u8,
    sound_timer: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            v: [0; 16],
            i: 0,

            delay_timer: 0,
            sound_timer: 0,
        }
    }
}