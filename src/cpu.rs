use crate::registers::Registers;
use crate::registers::ByteRegisterLabel;


pub struct CPU
{
    pub registers    : Registers,
    pub program_cntr : u16
}

pub enum Instruction
{
    ADD(ByteRegisterLabel),
}

impl CPU
{
    pub fn new() -> Self
    {
        CPU
        {
            registers    : Registers::new(),
            program_cntr : 0
        }
    }

    pub fn execute(&mut self, instruction : Instruction)
    {
        match instruction
        {
            Instruction::ADD(target) =>
            {
                let value = self.registers.get_byte(target);
                let new_value = self.add(value);
                self.registers.set_byte(ByteRegisterLabel::A, new_value);
            }
        }
    }

    fn add(&mut self, value: u8) -> u8
    {
        let a_value = self.registers.get_byte(ByteRegisterLabel::A);
        let (new_value, did_overflow) = a_value.overflowing_add(value);
        new_value
      }
}