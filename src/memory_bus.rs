use crate::cartridge::Cartridge;
use crate::cpu::CPURegisters;

pub struct MemoryBus<'a>
{
    cart       : &'a mut Cartridge
}

impl<'a> MemoryBus<'a>
{
    pub fn new(cart : &'a mut Cartridge) -> Self
    {
        MemoryBus
        {
            cart
        }
    }

    pub fn read(&self, address : u16) -> u8
    {
        self.cart.read(address)
    }
}