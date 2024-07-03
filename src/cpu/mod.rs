pub mod cpu;
pub mod registers;
pub mod alu;

#[derive(Copy, Clone)]
pub enum ByteRegister
{
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L
}

#[derive(Copy, Clone)]
pub enum FlagRegister
{
    Z,
    S,
    H,
    C
}

impl ByteRegister
{
    fn shift(&self) -> u8
    {
        match self
        {
            ByteRegister::A => 0,
            ByteRegister::B => 8,
            ByteRegister::C => 16,
            ByteRegister::D => 24,
            ByteRegister::E => 32,
            ByteRegister::F => 40,
            ByteRegister::H => 48,
            ByteRegister::L => 56,
        }
    }
}
