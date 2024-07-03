use crate::cpu::cpu::CPU;
use crate::mmu::mmu::MMU;

pub struct Emulation
{
    pub cpu : CPU,
    pub mmu : MMU
}

impl Emulation
{
    pub fn new() -> Self
    {
        Emulation
        {
            cpu : CPU::new(),
            mmu : MMU::new()
        }
    }

    pub fn start(&self)
    {

    }
}