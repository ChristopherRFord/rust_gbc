use crate::cpu::cpu_registers::Registers;
use crate::cpu::cpu_registers::FlagRegister;
use crate::cpu::cpu_registers::ByteRegisterTarget;
use crate::cpu::cpu_registers::WordRegisterTarget;

pub enum Instruction
{
    INC8(ByteRegisterTarget),
    INC16(WordRegisterTarget),
    DEC8(ByteRegisterTarget),
    DEC16(WordRegisterTarget),
    ADD(ByteRegisterTarget),
}

impl Instruction
{
    pub fn from_byte(byte : u8, prefixed : bool) -> Option<Instruction>
    {
        if prefixed
        {
            Instruction::from_byte_prefixed(byte)
        }
        else
        {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte : u8) -> Option<Instruction>
    {
        match byte
        {
            _ => None
        }
    }
    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction>
    {
        match byte
        {
            0x3C => Some(Instruction::INC8(ByteRegisterTarget::A)),
            0x04 => Some(Instruction::INC8(ByteRegisterTarget::B)),
            0x0C => Some(Instruction::INC8(ByteRegisterTarget::C)),
            0x14 => Some(Instruction::INC8(ByteRegisterTarget::D)),
            0x1C => Some(Instruction::INC8(ByteRegisterTarget::E)),
            0x24 => Some(Instruction::INC8(ByteRegisterTarget::H)),
            0x2C => Some(Instruction::INC8(ByteRegisterTarget::L)),
            //0x34 => Some(Instruction::INCHL()),
            0x03 => Some(Instruction::INC16(WordRegisterTarget::BC)),
            0x13 => Some(Instruction::INC16(WordRegisterTarget::DE)),
            0x23 => Some(Instruction::INC16(WordRegisterTarget::HL)),

            0x3D => Some(Instruction::DEC8(ByteRegisterTarget::A)),
            0x05 => Some(Instruction::DEC8(ByteRegisterTarget::B)),
            0x0D => Some(Instruction::DEC8(ByteRegisterTarget::C)),
            0x15 => Some(Instruction::DEC8(ByteRegisterTarget::D)),
            0x1D => Some(Instruction::DEC8(ByteRegisterTarget::E)),
            0x25 => Some(Instruction::DEC8(ByteRegisterTarget::H)),
            0x2D => Some(Instruction::DEC8(ByteRegisterTarget::L)),
            //0x35 => Some(Instruction::DECHL()),
            0x0B => Some(Instruction::DEC16(WordRegisterTarget::BC)),
            0x1B => Some(Instruction::DEC16(WordRegisterTarget::DE)),
            0x2B => Some(Instruction::DEC16(WordRegisterTarget::HL)),

            0x87 => Some(Instruction::ADD(ByteRegisterTarget::A)),
            0x80 => Some(Instruction::ADD(ByteRegisterTarget::B)),
            0x81 => Some(Instruction::ADD(ByteRegisterTarget::C)),
            0x82 => Some(Instruction::ADD(ByteRegisterTarget::D)),
            0x83 => Some(Instruction::ADD(ByteRegisterTarget::E)),
            0x84 => Some(Instruction::ADD(ByteRegisterTarget::H)),
            0x85 => Some(Instruction::ADD(ByteRegisterTarget::L)),
            //0x86 => Some(Instruction::ADDHL()),
            //0xD8 => TODO D8

            _ => None
        }
    }
}

fn set_flag_byte(registers : &mut Registers, zero : bool, subtract : bool, half_carry : bool, carry : bool)
{
    registers.set_flag_byte
    (
        FlagRegister
        {
            zero       : zero,
            subtract   : subtract,
            half_carry : half_carry,
            carry      : carry
        }
    )
}

pub fn inc8(registers : &mut Registers, target : ByteRegisterTarget)
{
    let value           = registers.get_byte(target);
    let new_value       = value.wrapping_add(1);
    let flag_register   = registers.get_flag_byte();
    let zero_flag       = new_value == 0;
    let half_carry_flag = (value & 0x0F) + 1 > 0x0F;

    registers.set_byte(target, new_value);
    set_flag_byte
    (
        registers,
        zero_flag,
        false,
        half_carry_flag,
        flag_register.carry
    );
}
pub fn inc16(registers: &mut Registers, target : WordRegisterTarget)
{
    let value           = registers.get_word(target);
    let new_value       = value.wrapping_add(1);
    let zero_flag       = new_value == 0;
    let half_carry_flag = (value & 0x0FFF) + 1 > 0x0FFF;
    let carry_flag      = new_value < value;

    registers.set_word(target, new_value);
    set_flag_byte
    (
        registers,
        zero_flag,
        false,
        half_carry_flag,
        carry_flag
    );
}

pub fn dec8(registers: &mut Registers, target : ByteRegisterTarget)
{
    let value           = registers.get_byte(target);
    let new_value       = value.wrapping_sub(1);
    let flag_register   = registers.get_flag_byte();
    let zero_flag       = new_value == 0;
    let half_carry_flag = (value & 0x0F) == 0x00;

    registers.set_byte(target, new_value);
    set_flag_byte
    (
        registers,
        zero_flag,
        true,
        half_carry_flag,
        flag_register.carry
    );
}
pub fn dec16(registers: &mut Registers, target : WordRegisterTarget)
{
    let value           = registers.get_word(target);
    let new_value       = value.wrapping_sub(1);
    let zero_flag       = new_value == 0;
    let half_carry_flag = (value & 0x0FFF) == 0x0000;
    let carry_flag      = value == 0x0000;

    registers.set_word(target, new_value);
    set_flag_byte
    (
        registers,
        zero_flag,
        true,
        half_carry_flag,
        carry_flag
    );
}


pub fn add(registers: &mut Registers, target : ByteRegisterTarget)
{
    let value              = registers.get_byte(target);
    let a_value            = registers.get_byte(ByteRegisterTarget::A);
    let (new_value, carry) = a_value.overflowing_add(value);
    let zero_flag          = new_value == 0;
    let half_carry_flag    = (value & 0x0F) + (a_value & 0x0F) > 0x0F;

    registers.set_byte(ByteRegisterTarget::A, new_value);
    set_flag_byte
    (
        registers,
        zero_flag,
        false,
        half_carry_flag,
        carry
    );
}