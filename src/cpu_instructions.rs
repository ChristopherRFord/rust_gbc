use crate::registers::Registers;
use crate::registers::ByteRegisterLabel;

pub fn add(registers: &mut Registers, target : ByteRegisterLabel) -> u8
{
    let value = registers.get_byte(target);
    let a_value = registers.get_byte(ByteRegisterLabel::A);
    let (new_value, did_overflow) = a_value.overflowing_add(value);
    new_value
}
