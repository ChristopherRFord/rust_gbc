pub mod misc
{

    pub fn nop()
    {

    }
    pub fn stop()
    {

    }
    pub fn halt(halted : &mut bool)
    {
        *halted = true;
    }
}