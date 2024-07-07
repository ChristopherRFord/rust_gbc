pub struct MMU
{
    pub memory : [u8; 0xFFFF]
}

impl MMU
{
    pub fn new() -> Self
    {
        MMU
        {
            memory : [0x0; 0xFFFF]
        }
    }

    pub fn read8(&self, address : u16) -> u8          { self.memory[address as usize] }
    pub fn write8(&mut self, address: u16, value: u8) { self.memory[address as usize] = value; }

    pub fn read16(&self, address : u16) -> u16
    {
        let low  = self.memory[address as usize] as u16;
        let high = self.memory[(address+1) as usize] as u16;
        (high << 8) | low
    }
    pub fn write16(&mut self, address: u16, value: u16)
    {
        let low  = (value & 0xFF) as u8;
        let high = (value >> 8) as u8;
        self.memory[address as usize] = low;
        self.memory[(address + 1) as usize] = high;
    }
}