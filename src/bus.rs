use crate::cart::Cart;
use crate::mmu::MMU;

pub struct Bus<'a>
{
    cart : &'a mut Cart,
    mmu  : &'a mut MMU
}

impl<'a> Bus<'a>
{
    pub fn new
    (
        cart      : &'a mut Cart, 
        mmu       : &'a mut MMU
    ) -> Self
    {
        Bus { cart, mmu }
    }

    pub fn read8(&self, address : u16) -> u8
    {
        if address < 0x8000
        {
            self.cart.read8(address)
        }
        else
        {
            self.mmu.read8(address)
        }
    }
    pub fn write8(&mut self, address : u16, value : u8)
    {
        self.mmu.write8(address, value);
    }

    pub fn read16(&self, address : u16) -> u16
    {
        if address < 0x8000
        {
            self.cart.read16(address)
        }
        else
        {
            self.mmu.read16(address)
        }
    }
    pub fn write16(&mut self, address: u16, value: u16)
    {
        self.mmu.write16(address, value);
    }
}