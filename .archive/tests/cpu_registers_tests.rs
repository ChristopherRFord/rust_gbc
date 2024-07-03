use rust_gbc::cpu::cpu_registers::Registers;
use rust_gbc::cpu::cpu_registers::FlagRegister;
use rust_gbc::cpu::cpu_registers::ByteRegisterTarget;
use rust_gbc::cpu::cpu_registers::WordRegisterTarget;

#[test]
fn test_registers_new()
{
    let regs = Registers::new();

    assert_eq!(regs.raw_memory, 0);
}

#[test]
fn test_get_set_byte()
{
    let mut regs = Registers::new();
    let value = 0xAB;

    regs.set_byte(ByteRegisterTarget::A, value);
    let read_value = regs.get_byte(ByteRegisterTarget::A);

    assert_eq!(read_value, value);
}

#[test]
fn test_get_set_word()
{
    let mut regs = Registers::new();
    let value = 0xABCD;

    regs.set_word(WordRegisterTarget::BC, value);
    let read_value = regs.get_word(WordRegisterTarget::BC);

    assert_eq!(read_value, value);
}

#[test]
fn test_flag_register_conversion()
{
    let flags = FlagRegister
    {
        zero: true,
        subtract: false,
        half_carry: true,
        carry: false,
    };

    let flag_byte: u8 = flags.into();
    let converted_flags: FlagRegister = flag_byte.into();

    assert_eq!(flags, converted_flags);
    assert_eq!(flag_byte, 0b10100000);
}

#[test]
fn test_get_set_flag_byte()
{
    let mut regs = Registers::new();
    let flags = FlagRegister
    {
        zero: true,
        subtract: false,
        half_carry: true,
        carry: false,
    };

    regs.set_flag_byte(flags);
    let read_flags = regs.get_flag_byte();

    assert_eq!(flags, read_flags);
}