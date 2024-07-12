
pub mod jump
{
    use crate::bus::*;
    use crate::cpu_enums::Reg;
    use crate::cpu_enums::CondType;
    use crate::cpu_enums::RegF;
    use crate::instructions::Context;

    fn check_condition(ctx : &Context) -> bool
    {
        let z_flag = ctx.regs.read_flag(RegF::Z);
        let c_flag = ctx.regs.read_flag(RegF::C);

        match ctx.cond_type
        {
            CondType::NONE => true,
            CondType::NZ   => !z_flag,
            CondType::Z    => z_flag,
            CondType::NC   => !c_flag,
            CondType::C    => c_flag
        }
    }
    
    fn goto(ctx  : &mut Context, 
            addr : u16, 
            push_pc : bool)
    {
        if check_condition(ctx)
        {
            if push_pc
            {
                let sp = ctx.regs.read(Reg::SP);
                bus::push16(ctx.cart, ctx.mem, ctx.regs, sp);
            }

            ctx.regs.write(Reg::PC, addr);
        } 
    }

    pub fn jp(ctx : &mut Context)
    {
        goto(ctx, ctx.data, false);
    }


    pub fn call(ctx : &mut Context)
    {
        goto(ctx, ctx.data, true);
    }
}