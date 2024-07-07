pub mod alu
{
    use crate::cpu::registers::RegF;
    use crate::cpu::registers::Reg8;
    use crate::cpu::registers::Reg16;
    use crate::cpu::registers::Registers;

    //======
    // 8 Bit
    //======
    pub fn add8(regs : &mut Registers,
                from : Reg8,
                to   : Reg8)
    {
        let from_v = regs.read8(from);
        let to_v   = regs.read8(to);
        let result = from_v.wrapping_add(to_v);

        regs.write8(to, result);
        regs.writef(RegF::Z, result == 0);
        regs.writef(RegF::S, false);
        regs.writef(RegF::H, (to_v & 0xF) + (from_v & 0xF) > 0xF);
        regs.writef(RegF::C, (to_v as u16 + from_v as u16) > 0xFF);
    }
    pub fn adc8(regs : &mut Registers,
                from : Reg8,
                to   : Reg8)
    {
        let from_v = regs.read8(from);
        let to_v   = regs.read8(to);
        let carry  = if regs.readf(RegF::C) { 1 } else { 0 };
        let result = from_v.wrapping_add(to_v).wrapping_add(carry);

        regs.write8(to, result);
        regs.writef(RegF::Z, result == 0);
        regs.writef(RegF::S, false);
        regs.writef(RegF::H, (to_v & 0xF) + (from_v & 0xF) > 0xF);
        regs.writef(RegF::C, (to_v as u16 + from_v as u16) > 0xFF);
    }
    pub fn sub8(regs : &mut Registers,
                from : Reg8,
                to   : Reg8)
    {
        let from_v = regs.read8(from);
        let to_v   = regs.read8(to);
        let result = from_v.wrapping_sub(to_v);

        regs.write8(to, result);
        regs.writef(RegF::Z, result == 0);
        regs.writef(RegF::S, true);
        regs.writef(RegF::H, (to_v & 0xF) + (from_v & 0xF) > 0xF);
        regs.writef(RegF::C, (to_v as u16 + from_v as u16) > 0xFF);
    }
    pub fn sbc8(regs : &mut Registers,
                from : Reg8,
                to   : Reg8)
    {
        let from_v = regs.read8(from);
        let to_v   = regs.read8(to);
        let carry  = if regs.readf(RegF::C) { 1 } else { 0 };
        let result = from_v.wrapping_sub(to_v).wrapping_sub(carry);

        regs.write8(to, result);
        regs.writef(RegF::Z, result == 0);
        regs.writef(RegF::S, true);
        regs.writef(RegF::H, (to_v & 0xF) + (from_v & 0xF) > 0xF);
        regs.writef(RegF::C, (to_v as u16 + from_v as u16) > 0xFF);
    }
    pub fn inc(regs : &mut Registers,
               reg  : Reg8)
    {
        let mut result = regs.read8(reg);
        result = result.wrapping_add(1);

        regs.write8(reg, result);
        regs.writef(RegF::Z, result == 0);
        regs.writef(RegF::S, false);
        regs.writef(RegF::H, (result & 0xF) == 0xF);
    }
    pub fn dec(regs : &mut Registers,
               reg  : Reg8)
    {
        let mut result = regs.read8(reg);
        result = result.wrapping_sub(1);
        
        regs.write8(reg, result);
        regs.writef(RegF::Z, result == 0);
        regs.writef(RegF::S, true);
        regs.writef(RegF::H, (result & 0xF) == 0)
    }
    pub fn cpl(regs : &mut Registers,
               reg  : Reg8)
    {
        let value  = regs.read8(reg);
        let result = !value;

        regs.write8(reg, result);
        regs.writef(RegF::S, true);
        regs.writef(RegF::H, true);
    }
    pub fn da(regs : &mut Registers,
              reg  : Reg8)
    {
        let mut value  = regs.read8(reg);
        let mut adjust = 0;
        let carry      = regs.readf(RegF::C);

        if !regs.readf(RegF::S)
        {
            if regs.readf(RegF::H) ||
               (value & 0x0F) > 0x09
            {
                adjust += 0x06;
            }

            if carry ||
               (value > 0x99)
            {
                adjust += 0x60;
                regs.writef(RegF::C, true);
            }
        }
        else
        {
            if regs.readf(RegF::H)
            {
                adjust -= 0x06;
            }

            if carry
            {
                adjust -= 0x60;
            }
        }

        value = value.wrapping_add(adjust);
        regs.writef(RegF::Z, value == 0);
        regs.writef(RegF::H, false);
    }
    pub fn and8(regs : &mut Registers,
                from : Reg8,
                to   : Reg8)
    {
        let from_v = regs.read8(from);
        let to_v   = regs.read8(to);
        let result = from_v & to_v;

        regs.write8(to, result);
        regs.writef(RegF::Z, result == 0);
        regs.writef(RegF::S, false);
        regs.writef(RegF::H, false);
        regs.writef(RegF::C, false);
    }
    pub fn xor8(regs : &mut Registers,
                from : Reg8,
                to   : Reg8)
    {
        let from_v = regs.read8(from);
        let to_v   = regs.read8(to);
        let result = from_v ^ to_v;

        regs.write8(to, result);
        regs.writef(RegF::Z, result == 0);
        regs.writef(RegF::S, false);
        regs.writef(RegF::H, false);
        regs.writef(RegF::C, false);
    }
    pub fn or8(regs : &mut Registers,
               from : Reg8,
               to   : Reg8)
    {
        let from_v = regs.read8(from);
        let to_v   = regs.read8(to);
        let result = from_v | to_v;

        regs.write8(to, result);
        regs.writef(RegF::Z, result == 0);
        regs.writef(RegF::S, false);
        regs.writef(RegF::H, false);
        regs.writef(RegF::C, false);
    }
    pub fn cp8(regs : &mut Registers,
               from : Reg8,
               to   : Reg8)
    {
        let from_v = regs.read8(from);
        let to_v   = regs.read8(to);
        let result = from_v.wrapping_sub(to_v);

        regs.writef(RegF::Z, result == 0);
        regs.writef(RegF::S, true);
        regs.writef(RegF::H, (from_v & 0xF) < (to_v & 0xF));
        regs.writef(RegF::C, from_v < to_v);
    }
    //======
    // 16 Bit
    //======
    pub fn add16(regs : &mut Registers,
                 from : Reg16,
                 to  : Reg16)
    {
        let from_v = regs.read16(from);
        let to_v   = regs.read16(to);
        let result = from_v.wrapping_add(to_v);

        regs.write16(to, result);
        regs.writef(RegF::S, false);
        regs.writef(RegF::H, (to_v & 0x07FF) + (from_v & 0x07FF) > 0x07FF);
        regs.writef(RegF::C, to_v > 0xFFFF - from_v);
    }
    pub fn adc16(regs : &mut Registers,
                 from : Reg16,
                 to   : Reg16)
    {
        let from_v = regs.read16(from);
        let to_v   = regs.read16(to);
        let carry  = if regs.readf(RegF::C) { 1 } else { 0 };
        let result = from_v.wrapping_add(to_v).wrapping_add(carry);

        regs.write16(to, result);
        regs.writef(RegF::S, false);
        regs.writef(RegF::H, (to_v & 0x07FF) + (from_v & 0x07FF) > 0x07FF);
        regs.writef(RegF::C, to_v > 0xFFFF - from_v);
    }
    pub fn inc16(regs : &mut Registers,
                 reg  : Reg16)
    {
        let mut result = regs.read16(reg);
        result = result.wrapping_add(1);

        regs.write16(reg, result);
        regs.writef(RegF::Z, result == 0);
        regs.writef(RegF::S, false);
        regs.writef(RegF::H, (result & 0x0FFF) == 0x0FFF);
    }
    pub fn dec16(regs : &mut Registers,
                 reg  : Reg16)
    {
        let mut result = regs.read16(reg);
        result = result.wrapping_sub(1);

        regs.write16(reg, result);
        regs.writef(RegF::S, true);
        regs.writef(RegF::H, (result & 0xF) == 0);
    }

    //======
    // 16 to 8 Bit
    //======
    pub fn add16_to_8(regs : &mut Registers,
                      from : Reg16,
                      to   : Reg8)
    {
        let from_v = regs.read16(from) as u8;
        let to_v   = regs.read8(to);
        let result = from_v.wrapping_add(to_v);

        regs.write8(to, result);
        regs.writef(RegF::Z, result == 0);
        regs.writef(RegF::S, false);
        regs.writef(RegF::H, (to_v & 0xF) + (from_v & 0xF) > 0xF);
        regs.writef(RegF::C, (to_v as u16 + from_v as u16) > 0xFF);
    }
    pub fn adc16_to_8(regs : &mut Registers,
                      from : Reg16,
                      to   : Reg8)
    {
        let from_v = regs.read16(from) as u8;
        let to_v   = regs.read8(to);
        let carry  = if regs.readf(RegF::C) { 1 } else { 0 };
        let result = from_v.wrapping_add(to_v).wrapping_add(carry);

        regs.write8(to, result);
        regs.writef(RegF::Z, result == 0);
        regs.writef(RegF::S, false);
        regs.writef(RegF::H, (to_v & 0xF) + (from_v & 0xF) > 0xF);
        regs.writef(RegF::C, (to_v as u16 + from_v as u16) > 0xFF);
    }
    pub fn sub16_to_8(regs : &mut Registers,
                      from : Reg16,
                      to   : Reg8)
    {
        let from_v = regs.read16(from) as u8;
        let to_v   = regs.read8(to);
        let result = from_v.wrapping_sub(to_v);

        regs.write8(to, result);
        regs.writef(RegF::Z, result == 0);
        regs.writef(RegF::S, false);
        regs.writef(RegF::H, (to_v & 0xF) + (from_v & 0xF) > 0xF);
        regs.writef(RegF::C, (to_v as u16 + from_v as u16) > 0xFF);
    }
    pub fn sbc16_to_8(regs : &mut Registers,
                      from : Reg16,
                      to   : Reg8)
    {
        let from_v = regs.read16(from) as u8;
        let to_v   = regs.read8(to);
        let carry  = if regs.readf(RegF::C) { 1 } else { 0 };
        let result = from_v.wrapping_sub(to_v).wrapping_sub(carry);

        regs.write8(to, result);
        regs.writef(RegF::Z, result == 0);
        regs.writef(RegF::S, true);
        regs.writef(RegF::H, (to_v & 0xF) + (from_v & 0xF) > 0xF);
        regs.writef(RegF::C, (to_v as u16 + from_v as u16) > 0xFF);
    }
    pub fn and16_to_8(regs : &mut Registers,
                          from : Reg16,
                          to   : Reg8)
    {
        let from_v = regs.read16(from) as u8;
        let to_v   = regs.read8(to);
        let result = from_v & to_v;

        regs.write8(to, result);
        regs.writef(RegF::Z, result == 0);
        regs.writef(RegF::H, true);
    }
    pub fn xor16_to_8(regs : &mut Registers,
                      from : Reg16,
                      to   : Reg8)
    {
        let from_v = regs.read16(from) as u8;
        let to_v   = regs.read8(to);
        let result = from_v ^ to_v;

        regs.write8(to, result);
        regs.writef(RegF::Z, result == 0);
        regs.writef(RegF::H, true);
    }
    pub fn or16_to_8(regs : &mut Registers,
                     from : Reg16,
                     to   : Reg8)
    {
        let from_v = regs.read16(from) as u8;
        let to_v   = regs.read8(to);
        let result = from_v | to_v;

        regs.write8(to, result);
        regs.writef(RegF::Z, result == 0);
        regs.writef(RegF::H, true);
    }
    pub fn cp16_to_8(regs : &mut Registers,
                      from : Reg16,
                      to   : Reg8)
    {
        let from_v = regs.read16(from) as u8;
        let to_v   = regs.read8(to);
        let result = from_v.wrapping_sub(to_v);

        regs.writef(RegF::Z, result == 0);
        regs.writef(RegF::S, true);
        regs.writef(RegF::H, (from_v & 0xF) < (to_v & 0xF));
        regs.writef(RegF::C, from_v < to_v);
    }

    //======
    // Misc
    //======
    pub fn scf(regs : &mut Registers)
    {
        regs.writef(RegF::C, true);
        regs.writef(RegF::S, false);
        regs.writef(RegF::H, false);
    }

    pub fn ccf(regs : &mut Registers)
    {
        let carry_flag = regs.readf(RegF::C);
        regs.writef(RegF::C, !carry_flag);
        regs.writef(RegF::S, false);
        regs.writef(RegF::H, false);
    }
}