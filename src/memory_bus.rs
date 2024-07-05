use crate::cartridge::Cartridge;
use crate::mmu::MMU;


pub struct MemoryBus<'a>
{
    cart : &'a mut Cartridge,
    mmu : &'a mut MMU
}

impl<'a> MemoryBus<'a>
{
    pub fn new
    (
        cart : &'a mut Cartridge,
        mmu : &'a mut MMU
    ) -> Self
    {
        MemoryBus
        {
            cart,
            mmu
        }
    }

    pub fn read(&self, address : u16) -> u8
    {
        if address < 0x8000
        {
            self.cart.read(address)
        }
        else
        {
            self.mmu.read(address)
        }
    }

    pub fn read16(&self, address : u16) -> u16
    {
        self.mmu.read16(address)
    }

    pub fn write(&mut self, address : u16, value : u8)
    {
        self.mmu.write(address, value);
    }

    pub fn write16(&mut self, address : u16, value : u16)
    {

        self.mmu.write16(address, value);
    }
}