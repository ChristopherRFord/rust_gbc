pub mod shift
{
    use crate::cpu::registers::RegF;
    use crate::cpu::registers::Reg8;
    use crate::cpu::registers::Registers;

    fn srflagupdate(regs       : &mut Registers,
                    value      : u8,
                    carry_flag : bool)
    {
        regs.writef(RegF::Z, value == 0);
        regs.writef(RegF::S, false);
        regs.writef(RegF::H, false);
        regs.writef(RegF::C, carry_flag);
    }

    pub fn rlc(regs : &mut Registers,
               reg  : Reg8)
    {
        let value      = regs.read8(reg);
        let carry_flag = value & 0x80 == 0x80;
        let result     = (value << 1) | (if carry_flag { 1 } else { 0 });
    
        regs.write8(reg, result);
        srflagupdate(regs, result, carry_flag);
    }
    pub fn rrc(regs : &mut Registers,
               reg  : Reg8)
    {
        let value      = regs.read8(reg);
        let carry_flag = value & 0x01 == 0x01;
        let result     = (value >> 1) | (if carry_flag { 0x80 } else { 0 });

        regs.write8(reg, result);
        srflagupdate(regs, result, carry_flag);
    }
    pub fn rl(regs : &mut Registers,
              reg  : Reg8)
    {
        let value          = regs.read8(reg);
        let carry_flag     = regs.readf(RegF::C);
        let new_carry_flag = value & 0x80 == 0x80;
        let result         = (value << 1) | (if carry_flag { 1 } else { 0 });
    
        regs.write8(reg, result);
        srflagupdate(regs, result, new_carry_flag);
    }
    pub fn rr(regs : &mut Registers,
              reg  : Reg8)
    {
        let value          = regs.read8(reg);
        let carry_flag     = regs.readf(RegF::C);
        let new_carry_flag = value & 0x01 == 0x01;
        let result         = (value >> 1) | (if carry_flag { 0x80 } else { 0 });

        regs.write8(reg, result);
        srflagupdate(regs, result, new_carry_flag);
    }
}