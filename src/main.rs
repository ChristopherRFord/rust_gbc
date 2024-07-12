pub mod bus;
pub mod cpu;
pub mod cpu_enums;
pub mod cart;
pub mod console;
pub mod instructions;
pub mod mem;
pub mod regs;

pub use console::Console;

fn main()
{
    let mut console = Console::new();
    console.start("/workspace/Rust/rust_gbc/roms/dmg-acid2.gb");
}