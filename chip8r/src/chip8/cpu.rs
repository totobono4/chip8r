pub struct Cpu {
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; 16],

}

impl Cpu {
    pub fn new() -> Self {
        Self {
            program_counter: 0x200,
            stack_pointer: 0,
            stack: [0; 16],
        }
    }
}
