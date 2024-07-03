pub mod alu
{
    use super::super::registers::Registers;
    use super::super::FlagRegister;
    use super::super::ByteRegister;

    pub fn add(registers : &mut Registers, value : u8)
    {
        let a_value   = registers.read_register(ByteRegister::A);
        let result    = a_value.wrapping_add(value);

        registers.write_register(ByteRegister::A, result);
        registers.write_flag_register(FlagRegister::Z, result == 0);
        registers.write_flag_register(FlagRegister::S, false);
        registers.write_flag_register(FlagRegister::H, (a_value & 0xF) + (value & 0xF) > 0xF);
        registers.write_flag_register(FlagRegister::C, (a_value as u16 + value as u16) > 0xFF);
    }

    pub fn sub(registers : &mut Registers, value : u8)
    {
        let a_value = registers.read_register(ByteRegister::A);
        let result = a_value.wrapping_sub(value);

        registers.write_register(ByteRegister::A, result);
        registers.write_flag_register(FlagRegister::Z, result == 0);
        registers.write_flag_register(FlagRegister::S, true);
        registers.write_flag_register(FlagRegister::H, (a_value & 0xF) < (value & 0xF));
        registers.write_flag_register(FlagRegister::C, a_value < value);
    }

    pub fn and(registers : &mut Registers, value : u8)
    {
        let a_value = registers.read_register(ByteRegister::A);
        let result  = a_value & value;

        registers.write_register(ByteRegister::A, result);
        registers.write_flag_register(FlagRegister::Z, result == 0);
        registers.write_flag_register(FlagRegister::S, false);
        registers.write_flag_register(FlagRegister::H, false);
        registers.write_flag_register(FlagRegister::C, false);
    }

    pub fn or(registers : &mut Registers, value : u8)
    {
        let a_value = registers.read_register(ByteRegister::A);
        let result  = a_value | value;

        registers.write_register(ByteRegister::A, result);
        registers.write_flag_register(FlagRegister::Z, result == 0);
        registers.write_flag_register(FlagRegister::S, false);
        registers.write_flag_register(FlagRegister::H, false);
        registers.write_flag_register(FlagRegister::C, false);
    }

    pub fn xor(registers : &mut Registers, value : u8)
    {
        let a_value = registers.read_register(ByteRegister::A);
        let result  = a_value ^ value;
        
        registers.write_register(ByteRegister::A, result);
        registers.write_flag_register(FlagRegister::Z, result == 0);
        registers.write_flag_register(FlagRegister::S, false);
        registers.write_flag_register(FlagRegister::H, false);
        registers.write_flag_register(FlagRegister::C, false);
    }

    pub fn cp(registers : &mut Registers, value : u8)
    {
        let a_value = registers.read_register(ByteRegister::A);
        let result  = a_value.wrapping_sub(value);
        
        registers.write_flag_register(FlagRegister::Z, result == 0);
        registers.write_flag_register(FlagRegister::S, true);
        registers.write_flag_register(FlagRegister::H, (a_value & 0xF) < (value & 0xF));
        registers.write_flag_register(FlagRegister::C, a_value < value);
    }

    pub fn inc(registers : &mut Registers, value : u8) -> u8
    {
        let result = value.wrapping_add(1);
        
        registers.write_flag_register(FlagRegister::Z, result == 0);
        registers.write_flag_register(FlagRegister::S, false);
        registers.write_flag_register(FlagRegister::H, (value & 0xF) == 0xF);

        return result
    }

    pub fn dec(registers : &mut Registers, value : u8) -> u8
    {
        let result = value.wrapping_sub(1);
        
        registers.write_flag_register(FlagRegister::Z, result == 0);
        registers.write_flag_register(FlagRegister::S, true);
        registers.write_flag_register(FlagRegister::H, (value & 0xF) == 0);

        return result
    }

    // NEEDTEST
    pub fn add16(registers : &mut Registers, value : u16)
    {
        let hl_value = registers.read_register_hl();
        let result = hl_value.wrapping_add(value);

        registers.write_register_hl(result);
        registers.write_flag_register(FlagRegister::S, false);
        registers.write_flag_register(FlagRegister::H, (hl_value & 0x07FF) + (value & 0x07FF) > 0x07FF);
        registers.write_flag_register(FlagRegister::C, hl_value > 0xFFFF - value);
    }

    // NEEDTEST
    // pub fn add16imm(registers : &mut Registers, value : u16) -> u16 {} TODO

    // NEEDTEST
    pub fn swap(registers : &mut Registers, value : u8) -> u8
    {
        registers.write_flag_register(FlagRegister::Z,  value == 0);
        registers.write_flag_register(FlagRegister::S, false);
        registers.write_flag_register(FlagRegister::H, false);
        registers.write_flag_register(FlagRegister::C, false);
        (value >> 4) | (value << 4)
    }

    // NEEDTEST
    pub fn srflagupdate(registers : &mut Registers, value : u8, carry_flag : bool)
    {
        registers.write_flag_register(FlagRegister::Z, value == 0);
        registers.write_flag_register(FlagRegister::S, false);
        registers.write_flag_register(FlagRegister::H, false);
        registers.write_flag_register(FlagRegister::C, carry_flag);
    }

    // NEEDTEST
    pub fn rlc(registers : &mut Registers, value : u8) -> u8
    {
        let carry_flag = value & 0x80 == 0x80;
        let result     = (value << 1) | (if carry_flag { 1 } else { 0 });

        srflagupdate(registers, result, carry_flag);
        return result
    }

    // NEEDTEST
    //pub fn rl(registers : &mut Registers, value : u8) -> u8{} TODO

    // NEEDTEST
    pub fn rrc(registers : &mut Registers, value : u8) -> u8
    {
        let carry_flag = value & 0x01 == 0x01;
        let result     = (value >> 1) | (if carry_flag { 0x80 } else { 0 });

        srflagupdate(registers, result, carry_flag);
        return result
    }

    // NEEDTEST
    //fn alu_rr(&mut self, a: u8) -> u8 {} TODO

    // NEEDTEST
    pub fn sla(registers : &mut Registers, value : u8) -> u8
    {
        let carry_flag = value & 0x80 == 0x80;
        let result     = value << 1;

        srflagupdate(registers, result, carry_flag);
        return result;
    }

    // NEEDTEST
    pub fn sra(registers : &mut Registers, value : u8) -> u8
    {
        let carry_flag = value & 0x01 == 0x01;
        let result     = (value >> 1) | (value & 0x80);

        srflagupdate(registers, result, carry_flag);
        return result;
    }

    // NEEDTEST
    pub fn srl(registers : &mut Registers, value : u8) -> u8
    {
        let carry_flag = value & 0x01 == 0x01;
        let result     = value >> 1;

        srflagupdate(registers, result, carry_flag);
        return result;
    }

    // NEEDTEST
    pub fn bit(registers : &mut Registers, value_a : u8, value_b : u8)
    {
        let zero_flag = value_a & (1 << (value_b as u32)) == 0;

        registers.write_flag_register(FlagRegister::S, false);
        registers.write_flag_register(FlagRegister::H, true);
        registers.write_flag_register(FlagRegister::Z, zero_flag);
    }

    // NEEDTEST
    //pub fn daa(){}

    // NEEDTEST
    //pub fn cpu_jr(){}
}

