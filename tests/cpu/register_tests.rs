use rust_gbc::cpu::registers::Registers;
use rust_gbc::cpu::FlagRegister;
use rust_gbc::cpu::ByteRegister;

#[test]
fn read_write_register()
{
    let mut reg = Registers::new();
    
    reg.write_register(ByteRegister::B, 0x12);
    assert_eq!(reg.read_register(ByteRegister::B), 0x12);
    
    reg.write_register(ByteRegister::C, 0x34);
    assert_eq!(reg.read_register(ByteRegister::C), 0x34);
}

#[test]
fn read_write_flag_zero()
{
    let mut reg = Registers::new();

    reg.write_flag_register(FlagRegister::Z, true);
    assert!(reg.read_flag_register(FlagRegister::Z));
    
    reg.write_flag_register(FlagRegister::Z, false);
    assert!(!reg.read_flag_register(FlagRegister::Z));
}

#[test]
fn read_write_flag_subtract()
{
    let mut reg = Registers::new();
    
    reg.write_flag_register(FlagRegister::S, true);
    assert!(reg.read_flag_register(FlagRegister::S));
    
    reg.write_flag_register(FlagRegister::S, false);
    assert!(!reg.read_flag_register(FlagRegister::S));
}

#[test]
fn read_write_flag_half_carry()
{
    let mut reg = Registers::new();
    
    reg.write_flag_register(FlagRegister::H, true);
    assert!(reg.read_flag_register(FlagRegister::H));
    
    reg.write_flag_register(FlagRegister::H, false);
    assert!(!reg.read_flag_register(FlagRegister::H));
}

#[test]
fn read_write_flag_carry()
{
    let mut reg = Registers::new();

    reg.write_flag_register(FlagRegister::C, true);
    assert!(reg.read_flag_register(FlagRegister::C));
    
    reg.write_flag_register(FlagRegister::C, false);
    assert!(!reg.read_flag_register(FlagRegister::C));
}

#[test]
fn read_write_register_hl()
{
    let mut reg = Registers::new();
    
    reg.write_register_hl(0x1234);
    assert_eq!(reg.read_register_hl(), 0x1234);
    
    reg.write_register_hl(0xABCD);
    assert_eq!(reg.read_register_hl(), 0xABCD);
}