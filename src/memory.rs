pub struct Memory
{
    pub memory : [u8; 0xFFFF]
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

    pub fn read_byte(&self, address : u16) -> u8          { self.memory[address as usize] }
    pub fn write_byte(&mut self, address: u16, value: u8) { self.memory[address as usize] = value; }

    pub fn read_word(&self, address : u16) -> u16
    {
        let low  = self.memory[address as usize] as u16;
        let high = self.memory[(address+1) as usize] as u16;
        (high << 8) | low
    }
    pub fn write_word(&mut self, address: u16, value: u16)
    {
        let low = (value & 0xFF) as u8;
        let high = (value >> 8) as u8;
        self.memory[address as usize] = low;
        self.memory[(address + 1) as usize] = high;
    }
}