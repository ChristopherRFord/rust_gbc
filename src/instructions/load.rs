pub mod load
{
    use crate::bus::bus;
    use crate::cpu_enums::Reg;
    use crate::instructions::Context;

    pub fn ld(ctx : &mut Context)
    {
        if ctx.dest_is_mem
        {
            match ctx.reg_2
            {
                Reg::AF =>
                {

                },
                _ =>
                {
                    bus::write8(ctx.cart, ctx.mem, ctx.regs, ctx.mem_addr, ctx.data as u8); 
                }
            }
        }
        else
        {
            ctx.regs.write(ctx.reg_1, ctx.data);
        }
    }

    pub fn ldh(ctx : &mut Context)
    {
        if ctx.reg_1 == Reg::A
        {

        }
        else
        {
            bus::write8(ctx.cart, ctx.mem, ctx.regs, ctx.mem_addr | 0xFF00, ctx.data as u8);
        }
    }
}