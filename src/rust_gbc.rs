use crate::cart::Cart;
use crate::cpu::cpu::CPU;
use crate::mmu::MMU;


pub struct RustGBC;

impl RustGBC
{
    pub fn new() -> Self
    {
        RustGBC{}
    }

    pub fn start(&self)
    {
        //cart.load_cart("/workspace/Rust/rust_gbc/roms/dmg-acid2.gb");
        //cart.print_info();

        let mut cart = Cart::new();
        let mut mmu  = MMU::new();
        let cpu  = CPU::new(&mut cart, &mut mmu);
    }
}