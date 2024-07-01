use crate::registers::Registers;
use crate::registers::ByteRegisterTarget;
use crate::registers::WordRegisterTarget;

pub enum Instruction
{
    ADD(ByteRegisterTarget),

    INC8(ByteRegisterTarget),
    INC16(WordRegisterTarget),
    DEC8(ByteRegisterTarget),
    DEC16(WordRegisterTarget)
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
        Some(Instruction::ADD(ByteRegisterTarget::C))
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
            //0x34 => TODO HLI
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
            //0x35 => TODO HLI
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
            //0x86 => TODO HLI
            //0xD8 => TODO D8

            _ => None
        }
    }

}


pub fn add(registers: &mut Registers, target : ByteRegisterTarget)
{
    let value = registers.get_byte(target);
    let a_value = registers.get_byte(ByteRegisterTarget::A);
    let (new_value, did_overflow) = a_value.overflowing_add(value);
    registers.set_byte(ByteRegisterTarget::A, new_value);
}

pub fn addhl(registers: &mut Registers, target : WordRegisterTarget)
{

}


pub fn inc8(registers: &mut Registers, target : ByteRegisterTarget)
{
    let new_value = registers.get_byte(target) + 1;
    registers.set_byte(target, new_value);
}
pub fn inc16(registers: &mut Registers, target : WordRegisterTarget)
{
    let new_value = registers.get_word(target) + 1;
    registers.set_word(target, new_value);
}

pub fn dec8(registers: &mut Registers, target : ByteRegisterTarget)
{
    let new_value = registers.get_byte(target) - 1;
    registers.set_byte(target, new_value);
}
pub fn dec16(registers: &mut Registers, target : WordRegisterTarget)
{
    let new_value = registers.get_word(target) - 1;
    registers.set_word(target, new_value);
}
