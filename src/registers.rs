pub struct Registers
{
    raw_memory : u64
}

pub struct FlagRegister
{
    pub zero       : bool,
    pub subtract   : bool,
    pub half_carry : bool,
    pub carry      : bool
}

pub enum ByteRegisterLabel
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

impl Registers
{
    pub fn new() -> Self
    {
        Registers
        {
            raw_memory : 0,
        }
    }

    // Helpers
    fn get_register_byte(&self, shift : u8) -> u8    { ((self.raw_memory >> shift) & 0xFF) as u8 }
    fn set_register_byte(&mut self, shift: u8, value : u8)
    {
        let mask = !(0xFFu64 << shift);
        self.raw_memory = (self.raw_memory & mask) | ((value as u64) << shift);
    }

    // 8 BIT
    pub fn get_byte(&self, label: ByteRegisterLabel) -> u8          { self.get_register_byte(label.shift()) }
    pub fn set_byte(&mut self, label: ByteRegisterLabel, value: u8) { self.set_register_byte(label.shift(), value); }

    pub fn get_flag_byte(&self) -> FlagRegister                     { self.get_register_byte(ByteRegisterLabel::F.shift()).into() }
    pub fn set_flag_byte(&mut self, flags: FlagRegister)            { self.set_register_byte(ByteRegisterLabel::F.shift(), flags.into()); }
}

impl ByteRegisterLabel
{
    fn shift(&self) -> u8
    {
        match self
        {
            ByteRegisterLabel::A => 0,
            ByteRegisterLabel::B => 8,
            ByteRegisterLabel::C => 16,
            ByteRegisterLabel::D => 24,
            ByteRegisterLabel::E => 32,
            ByteRegisterLabel::F => 40,
            ByteRegisterLabel::H => 48,
            ByteRegisterLabel::L => 56,
        }
    }
}

const ZERO_FLAG_BYTE_POSITION       : u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION   : u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION : u8 = 5;
const CARRY_FLAG_BYTE_POSITION      : u8 = 4;

impl From<FlagRegister> for u8
{
    fn from(flag: FlagRegister) -> u8
    {
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl From<u8> for FlagRegister
{
    fn from(byte: u8) -> Self
    {
        let zero       = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract   = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry      = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagRegister
        {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}