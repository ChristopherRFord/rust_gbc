pub mod emulation;
pub mod cpu;
pub mod mmu;
pub mod cart;

use cart::Cart;
use emulation::Emulation;

fn main()
{
    let mut cart = Cart::new();
    let _ = cart.load_cart("/workspace/Rust/rust_gbc/roms/dmg-acid2.gb");
    _ = cart.load_cart("/workspace/Rust/rust_gbc/roms/zelda.gbc");


    let gameboycolor = Emulation::new();

    gameboycolor.start();
}
