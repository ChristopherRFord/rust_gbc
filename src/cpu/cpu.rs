use super::registers::Registers;

pub struct CPU
{
    registers : Registers
}

impl CPU
{
    
    pub fn new() -> Self
    {
        CPU
        {
            registers : Registers::new()
        }
    }

    pub fn run(&mut self)
    {

    }

    pub fn step(&mut self)
    {

    }
}