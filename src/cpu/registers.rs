const ZERO_FLAG_BYTE_POSITION       : u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION   : u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION : u8 = 5;
const CARRY_FLAG_BYTE_POSITION      : u8 = 4;

#[derive(Copy, Clone)]
pub enum RegF
{
    Z,
    S,
    H,
    C
}

#[derive(Copy, Clone)]
pub enum Reg8
{
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
    HL
}

impl Reg8
{
    fn shift(&self) -> u8
    {
        match self
        {
            Reg8::A  => 0,
            Reg8::B  => 8,
            Reg8::C  => 16,
            Reg8::D  => 24,
            Reg8::E  => 32,
            Reg8::F  => 40,
            Reg8::H  => 48,
            Reg8::L  => 56,
            Reg8::HL => 48
        }
    }
}

#[derive(Copy, Clone)]
pub enum Reg16
{
    AF,
    BC,
    DE,
    HL,
    SP
}

pub struct Registers
{
    raw_registers : u64,
    stack_pointer : u16
}

impl Registers
{
    pub fn new() -> Self
    {
        Registers
        {
            raw_registers : 0x00000000,
            stack_pointer : 0x0000
        }
    }


    fn read_raw(&self, shift : u8) -> u8 { ((self.raw_registers >> shift) & 0xFF) as u8 }
    fn write_raw(&mut self, shift : u8, value : u8)
    {
        let mask = !(0xFFu64 << shift);
        self.raw_registers = (self.raw_registers & mask) | ((value as u64) << shift);
    }

    pub fn read8(&self, label : Reg8) -> u8            { self.read_raw(label.shift()) }
    pub fn write8(&mut self, label : Reg8, value : u8) { self.write_raw(label.shift(), value); }

    pub fn read16(&self, register : Reg16) -> u16
    {
        match register
        {
            Reg16::AF => (self.read8(Reg8::A) as u16) << 8 | self.read8(Reg8::F) as u16,
            Reg16::BC => (self.read8(Reg8::B) as u16) << 8 | self.read8(Reg8::C) as u16,
            Reg16::DE => (self.read8(Reg8::D) as u16) << 8 | self.read8(Reg8::E) as u16,
            Reg16::HL => (self.read8(Reg8::H) as u16) << 8 | self.read8(Reg8::L) as u16,
            Reg16::SP => self.stack_pointer 
        }
    }
    pub fn write16(&mut self, register : Reg16, value : u16)
    {
        match register
        {
            Reg16::AF => 
            {
                self.write8(Reg8::A, (value >> 8) as u8);
                self.write8(Reg8::F, value as u8);
            },
            Reg16::BC =>
            {
                self.write8(Reg8::B, (value >> 8) as u8);
                self.write8(Reg8::C, value as u8);
            },
            Reg16::DE =>
            {
                self.write8(Reg8::D, (value >> 8) as u8);
                self.write8(Reg8::E, value as u8);
            },
            Reg16::HL =>
            {
                self.write8(Reg8::H, (value >> 8) as u8);
                self.write8(Reg8::L, value as u8);
            },
            Reg16::SP =>
            {
                self.stack_pointer = value;
            }
        }       
    }

    pub fn readf(&self, flag : RegF) -> bool
    {
        let flag_register = self.read8(Reg8::F);
        match flag
        {
            RegF::Z => (flag_register & (1 << ZERO_FLAG_BYTE_POSITION))       != 0,
            RegF::S => (flag_register & (1 << SUBTRACT_FLAG_BYTE_POSITION))   != 0,
            RegF::H => (flag_register & (1 << HALF_CARRY_FLAG_BYTE_POSITION)) != 0,
            RegF::C => (flag_register & (1 << CARRY_FLAG_BYTE_POSITION))      != 0,
        }
    }
    pub fn writef(&mut self, flag : RegF, value : bool)
    {
        let mut flag_register = self.read8(Reg8::F);
        let bit_position = match flag
        {
            RegF::Z => ZERO_FLAG_BYTE_POSITION,
            RegF::S => SUBTRACT_FLAG_BYTE_POSITION,
            RegF::H => HALF_CARRY_FLAG_BYTE_POSITION,
            RegF::C => CARRY_FLAG_BYTE_POSITION,
        };

        if value
        {
            flag_register |= 1 << bit_position;
        }
        else
        {
            flag_register &= !(1 << bit_position);
        }

        self.write8(Reg8::F, flag_register);
    }
}