pub mod jump
{
    use crate::bus::Bus;
    use crate::cpu::pcntr::PCntr;
    use crate::cpu::registers::RegF;
    use crate::cpu::registers::Registers;
    
    pub fn jr(bus   : &Bus,
              pcntr : &mut PCntr)
    {
        let offset = bus.read8(pcntr.cntr());
        pcntr.inc(1);
        pcntr.inc(offset as u16);
    }
    pub fn jr_cond(bus   : &Bus,
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
}