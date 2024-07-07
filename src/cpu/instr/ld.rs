pub mod ld
{
    use crate::bus::Bus;
    use crate::cpu::pcntr::PCntr;
    use crate::cpu::registers::Reg8;
    use crate::cpu::registers::Reg16;
    use crate::cpu::registers::Registers;

    //======
    // 8 Bit
    //======
    pub fn reg8_to_reg8(regs : &mut Registers,
                        from : Reg8,
                        to   : Reg8)
    {
        let value = regs.read8(from);
        regs.write8(to, value);
    }
    pub fn reg16_to_reg8(regs : &mut Registers,
                         from : Reg16,
                         to   : Reg8)
    {
        let value = regs.read16(from) as u8;
        regs.write8(to, value);
    }
    pub fn reg8_to_reg16(regs : &mut Registers,
                         from : Reg8,
                         to   : Reg16)
    {
        let value = regs.read8(from);
        regs.write16(to, value as u16);
    }

    pub fn reg8_to_mem8(bus   : &mut Bus,
                        regs  : &Registers,
                        raddr : Reg16,
                        rvalu : Reg8)
    {
        let address = regs.read16(raddr);
        let value = regs.read8(rvalu);
        bus.write8(address, value);
    }
    pub fn reg8_to_mem8_inc(bus   : &mut Bus,
                            regs  : &mut Registers,
                            raddr : Reg16,
                            rvalu : Reg8)
    {
        let mut address = regs.read16(raddr);
        let value       = regs.read8(rvalu);
        bus.write8(address, value);
        address = address.wrapping_add(1);
        regs.write16(raddr, address);
    }
    pub fn reg8_to_mem8_dec(bus   : &mut Bus,
                            regs  : &mut Registers,
                            raddr : Reg16,
                            rvalu : Reg8)
    {
        let mut address = regs.read16(raddr);
        let value       = regs.read8(rvalu);
        bus.write8(address, value);
        address = address.wrapping_add(1);
        regs.write16(raddr, address);
    }
    pub fn mem8_to_reg8(bus   : &Bus,
                        regs  : &mut Registers,
                        raddr : Reg16,
                        rvalu : Reg8)
    {
        let address = regs.read16(raddr);
        let value   = bus.read8(address);
        regs.write8(rvalu, value);
    }
    pub fn mem8_to_reg8_inc(bus   : &Bus,
                            regs  : &mut Registers,
                            raddr : Reg16,
                            rvalu : Reg8)
    {
        let mut address = regs.read16(raddr);
        let value       = bus.read8(address);
        regs.write8(rvalu, value);
        address = address.wrapping_add(1);
        regs.write16(raddr, address);
    }
    pub fn mem8_to_reg8_dec(bus   : &Bus,
                            regs  : &mut Registers,
                            raddr : Reg16,
                            rvalu : Reg8)
    {
        let mut address = regs.read16(raddr);
        let value       = bus.read8(address);
        regs.write8(rvalu, value);
        address = address.wrapping_sub(1);
        regs.write16(raddr, address);
    }
    pub fn imm8_to_reg8(bus   : &Bus,
                        pcntr : &mut PCntr,
                        regs  : &mut Registers,
                        reg   : Reg8)
    {
        let value = bus.read8(pcntr.cntr());
        regs.write8(reg, value);
        pcntr.inc(1);   
    }
    pub fn imm8_to_reg16(bus   : &Bus,
                         pcntr : &mut PCntr,
                         regs  : &mut Registers,
                         reg   : Reg16)
    {
        let value = bus.read8(pcntr.cntr()) as u16;
        regs.write16(reg, value);
        pcntr.inc(1);    
    }

    //======
    // 16 Bit
    //======
    pub fn imm16_to_reg16(bus   : &Bus,
                          pcntr : &mut PCntr,
                          regs  : &mut Registers,
                          reg   : Reg16)
    {
        let value = bus.read16(pcntr.cntr());
        regs.write16(reg, value);
        pcntr.inc(2);   
    }
    pub fn reg16_to_imm16(bus   : &mut Bus,
                          pcntr : &mut PCntr,
                          regs  : &Registers,
                          reg   : Reg16)
    {
        let value = regs.read16(reg);
        let address = bus.read16(pcntr.cntr());
        bus.write16(address, value);
        pcntr.inc(2);
    }
    pub fn pop16(regs : &mut Registers,
                 reg  : Reg16)
    {
        let value  = regs.read16(Reg16::SP);
        let result = value.wrapping_add(1);
        regs.write16(reg, result);
    }
    pub fn push16(regs : &mut Registers,
                  reg  : Reg16)
    {
        let value  = regs.read16(Reg16::SP);
        let result = value.wrapping_sub(1);
        regs.write16(reg, result);
    }
}