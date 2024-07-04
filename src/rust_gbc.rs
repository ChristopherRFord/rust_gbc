use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::memory_bus::MemoryBus;

pub struct RustGBC;

impl RustGBC
{
    pub fn new() -> Self
    {
        RustGBC{}
    }

    pub fn start(&self)
    {
        let mut cart = Cartridge::new();
        cart.load_cart("/workspace/Rust/rust_gbc/roms/dmg-acid2.gb");
        //cart.load_cart("/workspace/Rust/rust_gbc/roms/zelda.gbc");

        let bus      = MemoryBus::new(&mut cart);
        let mut cpu  = CPU::new(bus);

        cpu.cpu_step();
        cpu.cpu_step();
        cpu.cpu_step();
        cpu.cpu_step();
        cpu.cpu_step();

    }
}