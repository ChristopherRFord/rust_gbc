pub mod misc
{
    use crate::instructions::Context;

    pub fn nop(){}

    pub fn di(context : &mut Context)
    {
        *context.int_en = false;
    }
}