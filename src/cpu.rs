use crate::registers::Registers;
use crate::registers::ByteRegisterLabel;

use crate::memory::Memory;

use crate::cpu_instructions::add;

pub struct CPU
{
    pub registers    : Registers,
    pub program_cntr : u16,
    pub memory       : Memory
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
            program_cntr : 0,
            memory       : Memory::new()
        }
    }

    pub fn step(&mut self)
    {
        /*
        let mut instruction_byte = self.memory.read_byte(self.program_cntr);

        let next_program_cntr = if let Some(instruction) = Instruction::from_byte(instruction_byte)
        {
            self.execute(instruction)
        } 
        else
        {
            panic!("Unkown instruction found for: 0x{:x}", instruction_byte);
        };
      
        self.program_cntr = next_program_cntr;
        */
    }

    pub fn execute(&mut self, instruction : Instruction)
    {
        match instruction
        {
            Instruction::ADD(target) =>
            {
                let new_value = add(&mut self.registers, target);
                self.registers.set_byte(ByteRegisterLabel::A, new_value);
            }
        }
    }
}