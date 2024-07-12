use crate::cpu_enums::RamType;

pub struct Mem
{
    wram : [u8; 0x2000],
    hram : [u8; 0x80]
}

impl Mem
{
    pub fn new() -> Self
    {
        Mem
        {
            wram : [0x0; 0x2000],
            hram : [0x0; 0x80]
        }
    }

    pub fn read8(&self, ram_type : &RamType, address : u16) -> u8
    {
        match ram_type
        {
            RamType::WRAM =>
            {
                let wram_address = address.wrapping_sub(0xC000);
                self.wram[wram_address as usize]
            },
            RamType::HRAM =>
            {
                let hram_address = address.wrapping_sub(0xFF80);
                self.hram[hram_address as usize]
            }
        }
    }
    pub fn read16(&self, ram_type : &RamType, address : u16) -> u16
    {
        let low_byte  = self.read8(ram_type, address) as u16;
        let high_byte = self.read8(ram_type, address.wrapping_add(1)) as u16;
        (high_byte << 8) | low_byte
    }

    pub fn write8(&mut self, ram_type: &RamType, address: u16, value: u8)
    {
        match ram_type
        {
            RamType::WRAM =>
            {
                let wram_address = address - 0xC000;
                self.wram[wram_address as usize] = value;
            },
            RamType::HRAM =>
            {
                let hram_address = address - 0xFF80;
                self.hram[hram_address as usize] = value;
            },
        }
    }
    pub fn write16(&mut self, ram_type : &RamType, address: u16, value: u16)
    {
        let low_byte  = (value & 0xFF) as u8;
        let high_byte = (value >> 8) as u8;
        self.write8(ram_type, address, low_byte);
        self.write8(ram_type, address.wrapping_add(1), high_byte);
    }
}