pub mod cart;
pub mod cpu;
pub mod bus;
pub mod mmu;
pub mod rust_gbc;

pub use rust_gbc::RustGBC;

fn main()
{
    let gbc = RustGBC::new();
    gbc.start();
}