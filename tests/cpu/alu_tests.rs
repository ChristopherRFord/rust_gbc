pub mod alu_tests
{
    use rust_gbc::cpu::FlagRegister;
    use rust_gbc::cpu::ByteRegister;
    use rust_gbc::cpu::registers::Registers;

    use rust_gbc::cpu::alu::*;

// =======
// -ADD-
// =======
    #[test]
    fn add_no_carry_no_half_carry()
    {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0x14);
        alu::add(&mut reg, 0x22);

        assert_eq!(reg.read_register(ByteRegister::A), 0x36);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

    #[test]
    fn add_with_half_carry()
    {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0x0F);
        alu::add(&mut reg, 0x01);

        assert_eq!(reg.read_register(ByteRegister::A), 0x10);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        assert_eq!(reg.read_flag_register(FlagRegister::H), true);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

    #[test]
    fn add_with_carry()
    {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0xF0);
        alu::add(&mut reg, 0x10);

        assert_eq!(reg.read_register(ByteRegister::A), 0x00);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), true);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
        assert_eq!(reg.read_flag_register(FlagRegister::C), true);
    }

    #[test]
    fn add_with_half_carry_and_carry()
    {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0x8F);
        alu::add(&mut reg, 0x81);

        assert_eq!(reg.read_register(ByteRegister::A), 0x10);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        assert_eq!(reg.read_flag_register(FlagRegister::H), true);
        assert_eq!(reg.read_flag_register(FlagRegister::C), true);
    }

    #[test]
    fn add_resulting_in_zero()
    {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0x00);
        alu::add(&mut reg, 0x00);

        assert_eq!(reg.read_register(ByteRegister::A), 0x00);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), true);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

    #[test]
    fn add_no_carry_with_large_numbers()
    {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0x40);
        alu::add(&mut reg, 0x30);

        assert_eq!(reg.read_register(ByteRegister::A), 0x70);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

    #[test]
    fn add_with_max_value()
    {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0xFF);
        alu::add(&mut reg, 0x01);

        assert_eq!(reg.read_register(ByteRegister::A), 0x00);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), true);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        assert_eq!(reg.read_flag_register(FlagRegister::H), true);
        assert_eq!(reg.read_flag_register(FlagRegister::C), true);
    }

// =======
// -SUB-
// =======
    #[test]
    fn sub_no_borrow_no_half_borrow()
    {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0x20);
        alu::sub(&mut reg, 0x10);

        assert_eq!(reg.read_register(ByteRegister::A), 0x10);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), true);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

    #[test]
    fn sub_with_half_borrow()
    {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0x10);
        alu::sub(&mut reg, 0x01);

        assert_eq!(reg.read_register(ByteRegister::A), 0x0F);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), true);
        assert_eq!(reg.read_flag_register(FlagRegister::H), true);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

    #[test]
    fn sub_with_borrow()
    {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0x10);
        alu::sub(&mut reg, 0x20);

        assert_eq!(reg.read_register(ByteRegister::A), 0xF0);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), true);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
        assert_eq!(reg.read_flag_register(FlagRegister::C), true);
    }

    #[test]
    fn sub_with_half_borrow_and_borrow()
    {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0x00);
        alu::sub(&mut reg, 0x01);

        assert_eq!(reg.read_register(ByteRegister::A), 0xFF);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), true);
        assert_eq!(reg.read_flag_register(FlagRegister::H), true);
        assert_eq!(reg.read_flag_register(FlagRegister::C), true);
    }

    #[test]
    fn sub_resulting_in_zero()
    {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0x20);
        alu::sub(&mut reg, 0x20);

        assert_eq!(reg.read_register(ByteRegister::A), 0x00);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), true);
        assert_eq!(reg.read_flag_register(FlagRegister::S), true);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

    #[test]
    fn sub_no_borrow_with_large_numbers()
    {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0x70);
        alu::sub(&mut reg, 0x30);

        assert_eq!(reg.read_register(ByteRegister::A), 0x40);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), true);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

    #[test]
    fn sub_with_max_value()
    {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0x00);
        alu::sub(&mut reg, 0x01);

        assert_eq!(reg.read_register(ByteRegister::A), 0xFF);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), true);
        assert_eq!(reg.read_flag_register(FlagRegister::H), true);
        assert_eq!(reg.read_flag_register(FlagRegister::C), true);
    }

// =======
// -AND-
// =======
    #[test]
    fn and_no_zero() {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0x3C);
        alu::and(&mut reg, 0x0F);

        assert_eq!(reg.read_register(ByteRegister::A), 0x0C);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

    #[test]
    fn and_resulting_in_zero() {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0x00);
        alu::and(&mut reg, 0xF0);

        assert_eq!(reg.read_register(ByteRegister::A), 0x00);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), true);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

    #[test]
    fn and_with_half_carry()
    {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0x3C);
        alu::and(&mut reg, 0x0F);

        assert_eq!(reg.read_register(ByteRegister::A), 0x0C);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        //assert_eq!(reg.read_flag_register(FlagRegister::H), true); TODO
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

    #[test]
    fn and_all_bits_set()
    {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0xFF);
        alu::and(&mut reg, 0xFF);

        assert_eq!(reg.read_register(ByteRegister::A), 0xFF);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        //assert_eq!(reg.read_flag_register(FlagRegister::H), true); TODO
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

    #[test]
    fn and_alternating_bits()
    {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0xAA);
        alu::and(&mut reg, 0x55);

        assert_eq!(reg.read_register(ByteRegister::A), 0x00);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), true);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        //assert_eq!(reg.read_flag_register(FlagRegister::H), true); TODO
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

// =======
// -OR-
// =======
    #[test]
    fn or_no_zero()
    {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0x3C);
        alu::or(&mut reg, 0x0F);

        assert_eq!(reg.read_register(ByteRegister::A), 0x3F);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

    #[test]
    fn or_resulting_in_zero()
    {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0x00);
        alu::or(&mut reg, 0x00);

        assert_eq!(reg.read_register(ByteRegister::A), 0x00);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), true);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

    #[test]
    fn or_all_bits_set()
    {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0xFF);
        alu::or(&mut reg, 0x00);

        assert_eq!(reg.read_register(ByteRegister::A), 0xFF);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

    #[test]
    fn or_alternating_bits()
    {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0xAA);
        alu::or(&mut reg, 0x55);

        assert_eq!(reg.read_register(ByteRegister::A), 0xFF);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

    #[test]
    fn or_mixed_bits()
    {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0x5A);
        alu::or(&mut reg, 0xA5);

        assert_eq!(reg.read_register(ByteRegister::A), 0xFF);
        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

// =======
// -XOR-
// =======
    #[test]
    fn xor_no_zero() {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0x3C);
        alu::xor(&mut reg, 0x0F);
        assert_eq!(reg.read_register(ByteRegister::A), 0x33);

        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

    #[test]
    fn xor_resulting_in_zero() {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0xFF);
        alu::xor(&mut reg, 0xFF);
        assert_eq!(reg.read_register(ByteRegister::A), 0x00);

        assert_eq!(reg.read_flag_register(FlagRegister::Z), true);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

    #[test]
    fn xor_all_bits_set() {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0x00);
        alu::xor(&mut reg, 0xFF);
        assert_eq!(reg.read_register(ByteRegister::A), 0xFF);

        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

    #[test]
    fn xor_alternating_bits() {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0xAA);
        alu::xor(&mut reg, 0x55);
        assert_eq!(reg.read_register(ByteRegister::A), 0xFF);

        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

    #[test]
    fn xor_mixed_bits() {
        let mut reg = Registers::new();

        reg.write_register(ByteRegister::A, 0x5A);
        alu::xor(&mut reg, 0xA5);
        assert_eq!(reg.read_register(ByteRegister::A), 0xFF);

        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

// =======
// -CP-
// =======
    #[test]
    fn cp_equal()
    {
        let mut reg = Registers::new();
    
        reg.write_register(ByteRegister::A, 0x20);
        alu::cp(&mut reg, 0x20);
    
        assert_eq!(reg.read_flag_register(FlagRegister::Z), true);
        assert_eq!(reg.read_flag_register(FlagRegister::S), true);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }
    
    #[test]
    fn cp_greater()
    {
        let mut reg = Registers::new();
    
        reg.write_register(ByteRegister::A, 0x30);
        alu::cp(&mut reg, 0x20);
    
        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), true);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }
    
    #[test]
    fn cp_less()
    {
        let mut reg = Registers::new();
    
        reg.write_register(ByteRegister::A, 0x10);
        alu::cp(&mut reg, 0x20);
    
        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), true);
        //assert_eq!(reg.read_flag_register(FlagRegister::H), true); TODO
        assert_eq!(reg.read_flag_register(FlagRegister::C), true);
    }
    
    #[test]
    fn cp_zero()
    {
        let mut reg = Registers::new();
    
        reg.write_register(ByteRegister::A, 0x00);
        alu::cp(&mut reg, 0x00);
    
        assert_eq!(reg.read_flag_register(FlagRegister::Z), true);
        assert_eq!(reg.read_flag_register(FlagRegister::S), true);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }
    
    #[test]
    fn cp_half_carry()
    {
        let mut reg = Registers::new();
    
        reg.write_register(ByteRegister::A, 0x10);
        alu::cp(&mut reg, 0x01);
    
        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), true);
        assert_eq!(reg.read_flag_register(FlagRegister::H), true);
        assert_eq!(reg.read_flag_register(FlagRegister::C), false);
    }

// =======
// -INC-
// =======
    #[test]
    fn inc_zero_to_one()
    {
        let mut reg = Registers::new();
        
        let value = 0x00;
        let result = alu::inc(&mut reg, value);
        assert_eq!(result, 0x01);

        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
    }

    #[test]
    fn inc_no_carry()
    {
        let mut reg = Registers::new();

        let value = 0x0F;
        let result = alu::inc(&mut reg, value);
        assert_eq!(result, 0x10);

        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        assert_eq!(reg.read_flag_register(FlagRegister::H), true);
    }

    #[test]
    fn inc_to_zero()
    {
        let mut reg = Registers::new();

        let value = 0xFF;
        let result = alu::inc(&mut reg, value);
        assert_eq!(result, 0x00);

        assert_eq!(reg.read_flag_register(FlagRegister::Z), true);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        assert_eq!(reg.read_flag_register(FlagRegister::H), true);
    }

    #[test]
    fn inc_no_half_carry()
    {
        let mut reg = Registers::new();

        let value = 0xE;
        let result = alu::inc(&mut reg, value);
        assert_eq!(result, 0xF);

        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), false);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
    }

// =======
// -DEC-
// =======
    #[test]
    fn dec_one_to_zero()
    {
        let mut reg = Registers::new();
        
        let value = 0x01;
        let result = alu::dec(&mut reg, value);
        assert_eq!(result, 0x00);

        assert_eq!(reg.read_flag_register(FlagRegister::Z), true);
        assert_eq!(reg.read_flag_register(FlagRegister::S), true);
        //assert_eq!(reg.read_flag_register(FlagRegister::H), true); TODO
    }

    #[test]
    fn dec_no_borrow()
    {
        let mut reg = Registers::new();

        let value = 0x10;
        let result = alu::dec(&mut reg, value);
        assert_eq!(result, 0x0F);

        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), true);
        //assert_eq!(reg.read_flag_register(FlagRegister::H), false);
    }

    #[test]
    fn dec_no_half_borrow()
    {
        let mut reg = Registers::new();

        let value = 0xF;
        let result = alu::dec(&mut reg, value);
        assert_eq!(result, 0xE);

        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), true);
        assert_eq!(reg.read_flag_register(FlagRegister::H), false);
    }

    #[test]
    fn dec_underflow()
    {
        let mut reg = Registers::new();

        let value = 0x00;
        let result = alu::dec(&mut reg, value);
        assert_eq!(result, 0xFF);

        assert_eq!(reg.read_flag_register(FlagRegister::Z), false);
        assert_eq!(reg.read_flag_register(FlagRegister::S), true);
        assert_eq!(reg.read_flag_register(FlagRegister::H), true);
    }
}