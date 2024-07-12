use crate::cart::Cart;
use crate::cpu::CPU;
use crate::mem::Mem;

pub struct Console
{
    cart : Cart,
    cpu  : CPU,
    mem  : Mem
}

impl Console
{
    pub fn new() -> Self
    {
        Console
        {
            cart : Cart::new(),
            cpu  : CPU::new(),
            mem  : Mem::new()
        }
    }

    pub fn start(&mut self, rom_path : &str)
    {
        let loaded = self.cart.load(rom_path);
        if loaded
        {
            self.cart.print_info();

            self.cpu.start
            (
                &mut self.cart,
                &mut self.mem
            );
        }
    }
}