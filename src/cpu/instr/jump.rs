pub mod jump
{
    use crate::bus::Bus;
    use crate::cpu::pcntr::PCntr;
    use crate::cpu::registers::RegF;
    use crate::cpu::registers::Registers;
    
    //======
    // 8 Bit
    //======
    pub fn jr8(bus   : &Bus,
              pcntr : &mut PCntr)
    {
        let offset = bus.read8(pcntr.cntr());
        pcntr.inc(1);
        pcntr.inc(offset as u16);
    }
    pub fn jr8_cond(bus   : &Bus,
                   pcntr : &mut PCntr,
                   regs  : &Registers,
                   flag  : RegF,
                   cond  : bool)
    {
        let offset = bus.read8(pcntr.cntr());
        pcntr.inc(1);

        if regs.readf(flag) == cond
        {
            pcntr.inc(offset as u16);
        }
    }

    //======
    // 16 Bit
    //======
    pub fn jr16(bus   : &Bus,
                pcntr : &mut PCntr)
    {
        let offset = bus.read16(pcntr.cntr());
        pcntr.inc(2);
        pcntr.inc(offset);
    }
    pub fn jr16_cond(bus   : &Bus,
                     pcntr : &mut PCntr,
                     regs  : &Registers,
                     flag  : RegF,
                     cond  : bool)
    {
        let offset = bus.read16(pcntr.cntr());
        pcntr.inc(2);

        if regs.readf(flag) == cond
        {
            pcntr.inc(offset);
        }
    }
}