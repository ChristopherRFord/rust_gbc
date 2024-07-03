pub mod emulation;
pub mod cpu;
pub mod mmu;

use emulation::Emulation;

fn main()
{
    let gameboycolor = Emulation::new();

    gameboycolor.start();
}
