use rust_gbc::cpu::cpu_registers::Registers;
use rust_gbc::cpu::cpu_registers::ByteRegisterTarget;
use rust_gbc::cpu::cpu_registers::WordRegisterTarget;
use rust_gbc::memory::Memory;

use rust_gbc::cpu::cpu_instructions::inc8;
use rust_gbc::cpu::cpu_instructions::inc16;
use rust_gbc::cpu::cpu_instructions::dec8;
use rust_gbc::cpu::cpu_instructions::dec16;
use rust_gbc::cpu::cpu_instructions::add;

#[test]
fn test_inc8()
{
    let mut regs = Registers::new();

    regs.set_byte(ByteRegisterTarget::C, 0x00);
    inc8(&mut regs, ByteRegisterTarget::C);
    assert_eq!(regs.get_byte(ByteRegisterTarget::C), 0x01, "INC8 - NORMAL OPERATION");
    assert_eq!(regs.get_flag_byte().zero, false, "INC8 - ZERO - NORMAL OPERATION");
    assert_eq!(regs.get_flag_byte().subtract, false, "INC8 - SUBTRACT - NORMAL OPERATION");
    assert_eq!(regs.get_flag_byte().half_carry, false, "INC8 - HALF_CARRY - NORMAL OPERATION");

    regs.set_byte(ByteRegisterTarget::C, 0x0F);
    inc8(&mut regs, ByteRegisterTarget::C);
    assert_eq!(regs.get_byte(ByteRegisterTarget::C), 0x10, "INC8 - HALF CARRY");
    assert_eq!(regs.get_flag_byte().zero, false, "INC8 - ZERO - HALF CARRY");
    assert_eq!(regs.get_flag_byte().subtract, false, "INC8 - SUBTRACT - HALF CARRY");
    assert_eq!(regs.get_flag_byte().half_carry, true, "INC8 - HALF CARRY - HALF CARRY");
    
    regs.set_byte(ByteRegisterTarget::C, 0xFF);
    inc8(&mut regs, ByteRegisterTarget::C);
    assert_eq!(regs.get_byte(ByteRegisterTarget::C), 0x00, "INC8 - OVERFLOW");
    assert_eq!(regs.get_flag_byte().zero, true, "INC8 - ZERO - OVERFLOW");
    assert_eq!(regs.get_flag_byte().subtract, false, "INC8 - SUBTRACT - OVERFLOW");
    assert_eq!(regs.get_flag_byte().half_carry, true, "INC8 - HALF_CARRY - OVERFLOW");
}
#[test]
fn test_inc16()
{
    let mut regs = Registers::new();

    regs.set_word(WordRegisterTarget::BC, 0x0000);
    inc16(&mut regs, WordRegisterTarget::BC);
    assert_eq!(regs.get_word(WordRegisterTarget::BC), 0x0001, "INC16 - NORMAL OPERATION");
    assert_eq!(regs.get_flag_byte().zero, false, "INC16 - ZERO - NORMAL OPERATION");
    assert_eq!(regs.get_flag_byte().subtract, false, "INC16 - SUBTRACT - NORMAL OPERATION");
    assert_eq!(regs.get_flag_byte().half_carry, false, "INC16 - HALF_CARRY - NORMAL OPERATION");

    regs.set_word(WordRegisterTarget::BC, 0xFFFF);
    inc16(&mut regs, WordRegisterTarget::BC);
    assert_eq!(regs.get_word(WordRegisterTarget::BC), 0x0000, "INC16 - OVERFLOW");
    assert_eq!(regs.get_flag_byte().zero, true, "INC16 - ZERO - OVERFLOW");
    assert_eq!(regs.get_flag_byte().subtract, false, "INC16 - SUBTRACT - OVERFLOW");
    assert_eq!(regs.get_flag_byte().half_carry, true, "INC16 - HALF_CARRY - OVERFLOW");
}


#[test]
fn test_dec8()
{
    let mut regs = Registers::new();

    regs.set_byte(ByteRegisterTarget::C, 0x01);
    dec8(&mut regs, ByteRegisterTarget::C);
    assert_eq!(regs.get_byte(ByteRegisterTarget::C), 0x00, "DEC8 - NORMAL OPERATION");
    assert_eq!(regs.get_flag_byte().zero, true, "DEC8 - ZERO - NORMAL OPERATION");
    assert_eq!(regs.get_flag_byte().subtract, true, "DEC8 - SUBTRACT - NORMAL OPERATION");
    assert_eq!(regs.get_flag_byte().half_carry, false, "DEC8 - HALF_CARRY - NORMAL OPERATION");
    
    regs.set_byte(ByteRegisterTarget::C, 0x00);
    dec8(&mut regs, ByteRegisterTarget::C);
    assert_eq!(regs.get_byte(ByteRegisterTarget::C), 0xFF, "DEC8 - WRAPPING");
    assert_eq!(regs.get_flag_byte().zero, false, "DEC8 - ZERO - WRAPPING");
    assert_eq!(regs.get_flag_byte().subtract, true, "DEC8 - SUBTRACT - WRAPPING");
    assert_eq!(regs.get_flag_byte().half_carry, true, "DEC8 - HALF_CARRY - WRAPPING");
}

#[test]
fn test_dec16()
{
    let mut regs = Registers::new();

    regs.set_word(WordRegisterTarget::BC, 0x0001);
    dec16(&mut regs, WordRegisterTarget::BC);
    assert_eq!(regs.get_word(WordRegisterTarget::BC), 0x0000, "DEC16 - NORMAL OPERATION");
    assert_eq!(regs.get_flag_byte().zero, true, "DEC16 - ZERO - NORMAL OPERATION");
    assert_eq!(regs.get_flag_byte().subtract, true, "DEC16 - SUBTRACT - NORMAL OPERATION");
    assert_eq!(regs.get_flag_byte().half_carry, false, "DEC16 - HALF_CARRY - NORMAL OPERATION");
    
    regs.set_word(WordRegisterTarget::BC, 0x0000);
    dec16(&mut regs, WordRegisterTarget::BC);
    assert_eq!(regs.get_word(WordRegisterTarget::BC), 0xFFFF, "DEC16 - WRAPPING");
    assert_eq!(regs.get_flag_byte().zero, false, "DEC16 - ZERO - WRAPPING");
    assert_eq!(regs.get_flag_byte().subtract, true, "DEC16 - SUBTRACT - WRAPPING");
    assert_eq!(regs.get_flag_byte().half_carry, true, "DEC16 - HALF_CARRY - WRAPPING");
}


#[test]
fn test_add()
{
    let mut regs = Registers::new();

    regs.set_byte(ByteRegisterTarget::A, 0x00);
    regs.set_byte(ByteRegisterTarget::B, 0x00);
    add(&mut regs, ByteRegisterTarget::B);
    assert_eq!(regs.get_byte(ByteRegisterTarget::A), 0x00, "ADD - ZERO RESULT");
    assert_eq!(regs.get_flag_byte().zero, true, "ADD - ZERO - ZERO RESULT");
    assert_eq!(regs.get_flag_byte().subtract, false, "ADD - SUBTRACT - ZERO RESULT");
    assert_eq!(regs.get_flag_byte().half_carry, false, "ADD HALF CARRY - ZERO RESULT");
    assert_eq!(regs.get_flag_byte().carry, false, "ADD - CARRY - ZERO RESULT");

    regs.set_byte(ByteRegisterTarget::A, 0x20);
    regs.set_byte(ByteRegisterTarget::B, 0x20);
    add(&mut regs, ByteRegisterTarget::B);
    assert_eq!(regs.get_byte(ByteRegisterTarget::A), 0x40, "ADD - NORMAL OPERATION");
    assert_eq!(regs.get_flag_byte().zero, false, "ADD - ZERO - NORMAL OPERATION");
    assert_eq!(regs.get_flag_byte().subtract, false, "ADD - SUBTRACT - NORMAL OPERATION");
    assert_eq!(regs.get_flag_byte().half_carry, false, "ADD HALF CARRY - NORMAL OPERATION");
    assert_eq!(regs.get_flag_byte().carry, false, "ADD - CARRY - NORMAL OPERATION");

    regs.set_byte(ByteRegisterTarget::A, 0x80);
    regs.set_byte(ByteRegisterTarget::B, 0x80);
    add(&mut regs, ByteRegisterTarget::B);
    assert_eq!(regs.get_byte(ByteRegisterTarget::A), 0x00, "ADD - BIT 7 OVERFLOW");
    assert_eq!(regs.get_flag_byte().zero, true, "ADD - ZERO - BIT 7 OVERFLOW");
    assert_eq!(regs.get_flag_byte().subtract, false, "ADD - SUBTRACT - BIT 7 OVERFLOW");
    assert_eq!(regs.get_flag_byte().half_carry, false, "ADD - HALF_CARRY - BIT 7 OVERFLOW");
    assert_eq!(regs.get_flag_byte().carry, true, "ADD - carry - BIT 7 OVERFLOW");
}