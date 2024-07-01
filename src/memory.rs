pub struct Memory
{
    memory : [u8; 0xFFFF]
}

impl Memory
{
    pub fn new() -> Self
    {
        Memory
        {
            memory : [0; 0xFFFF]
        }
    }

    pub fn read_byte(&self, address : u16) -> u8 { self.memory[address as usize] }
}