pub struct PCntr
{
    cntr : u16
}

impl PCntr
{
    pub fn new() -> Self
    {
        PCntr
        {
            cntr : 0x0100
        }
    }

    pub fn cntr(&self) -> u16 { self.cntr }
    pub fn inc(&mut self, value : u16)
    {
        self.cntr = self.cntr.wrapping_add(value);
    }
    pub fn dec(&mut self, value : u16)
    {
        self.cntr = self.cntr.wrapping_sub(value);
    }
    pub fn set(&mut self, value : u16)
    {
        self.cntr = value;
    }
}