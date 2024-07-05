pub mod rust_gbc;
pub mod cartridge;
pub mod cpu;
pub mod mmu;
pub mod memory_bus;

pub use rust_gbc::RustGBC;

fn main()
{
    let gbc = RustGBC::new();
    gbc.start();
}
