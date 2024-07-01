use crate::cpu_instructions::Instruction;

use crate::registers::Registers;
use crate::registers::ByteRegisterTarget;

use crate::memory::Memory;

use crate::cpu_instructions::add;
use crate::cpu_instructions::inc8;
use crate::cpu_instructions::inc16;
use crate::cpu_instructions::dec8;
use crate::cpu_instructions::dec16;

pub struct CPU
{
    pub registers    : Registers,
    pub program_cntr : u16,
    pub memory       : Memory
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

    pub fn run(&mut self)
    {
        loop
        {
            let instruction_byte = self.memory.read_byte(self.program_cntr);
            if instruction_byte == 0x00
            {
                break;
            }
            else
            {
                self.step();

                println!("--- End of loop {}---", self.program_cntr);
                self.registers.dump();
            }
        }   
    }

    pub fn step(&mut self)
    {
        let mut instruction_byte = self.memory.read_byte(self.program_cntr);
        let next_program_cntr = if let Some(instruction) = Instruction::from_byte(instruction_byte, false)
        {
            self.program_cntr = self.execute(instruction)
        } 
        else
        {
            panic!("Unkown instruction found for: 0x{:x}", instruction_byte);
        };

    }

    pub fn execute(&mut self, instruction : Instruction)-> u16
    {
        match instruction
        {
            Instruction::ADD(target)   => { add(&mut self.registers, target); }
            Instruction::INC8(target)  => { inc8(&mut self.registers, target); }
            Instruction::INC16(target) => { inc16(&mut self.registers, target); }
            Instruction::DEC8(target)  => { dec8(&mut self.registers, target); }
            Instruction::DEC16(target)  => { dec16(&mut self.registers, target); }
        }

        self.program_cntr + 0x1
    }
}