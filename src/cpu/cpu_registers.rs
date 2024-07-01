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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ByteRegisterTarget
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WordRegisterTarget
{
    AF,
    BC,
    DE,
    HL
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

    pub fn dump(&self)
    {
        println!("A: {:02X}", self.get_byte(ByteRegisterTarget::A));
        println!("B: {:02X}", self.get_byte(ByteRegisterTarget::B));
        println!("C: {:02X}", self.get_byte(ByteRegisterTarget::C));
        println!("D: {:02X}", self.get_byte(ByteRegisterTarget::D));
        println!("E: {:02X}", self.get_byte(ByteRegisterTarget::E));
        println!("F: {:02X}", self.get_byte(ByteRegisterTarget::F));
        println!("H: {:02X}", self.get_byte(ByteRegisterTarget::H));
        println!("L: {:02X}", self.get_byte(ByteRegisterTarget::L));
        println!("AF: {:04X}", self.get_word(WordRegisterTarget::AF));
        println!("BC: {:04X}", self.get_word(WordRegisterTarget::BC));
        println!("DE: {:04X}", self.get_word(WordRegisterTarget::DE));
        println!("HL: {:04X}", self.get_word(WordRegisterTarget::HL));
    }

    // Helpers
    fn get_register_byte(&self, shift : u8) -> u8    { ((self.raw_memory >> shift) & 0xFF) as u8 }
    fn set_register_byte(&mut self, shift: u8, value : u8)
    {
        let mask = !(0xFFu64 << shift);
        self.raw_memory = (self.raw_memory & mask) | ((value as u64) << shift);
    }

    // 8 BIT
    pub fn get_byte(&self, label : ByteRegisterTarget) -> u8           { self.get_register_byte(label.shift()) }
    pub fn set_byte(&mut self, label : ByteRegisterTarget, value : u8) { self.set_register_byte(label.shift(), value); }

    pub fn get_flag_byte(&self) -> FlagRegister                     { self.get_register_byte(ByteRegisterTarget::F.shift()).into() }
    pub fn set_flag_byte(&mut self, flags : FlagRegister)            { self.set_register_byte(ByteRegisterTarget::F.shift(), flags.into()); }

    // 16 BIT
    pub fn get_word(&self, label : WordRegisterTarget) -> u16
    {
        match label
        {
            WordRegisterTarget::AF => (self.get_byte(ByteRegisterTarget::A) as u16) << 8 | self.get_byte(ByteRegisterTarget::F) as u16,
            WordRegisterTarget::BC => (self.get_byte(ByteRegisterTarget::B) as u16) << 8 | self.get_byte(ByteRegisterTarget::C) as u16,
            WordRegisterTarget::DE => (self.get_byte(ByteRegisterTarget::D) as u16) << 8 | self.get_byte(ByteRegisterTarget::E) as u16,
            WordRegisterTarget::HL => (self.get_byte(ByteRegisterTarget::H) as u16) << 8 | self.get_byte(ByteRegisterTarget::L) as u16,
        }
    }
    pub fn set_word(&mut self, label : WordRegisterTarget, value : u16)
    {
        match label
        {
            WordRegisterTarget::AF => 
            {
                self.set_byte(ByteRegisterTarget::A, (value >> 8) as u8);
                self.set_byte(ByteRegisterTarget::F, value as u8);
            },
            WordRegisterTarget::BC =>
            {
                self.set_byte(ByteRegisterTarget::B, (value >> 8) as u8);
                self.set_byte(ByteRegisterTarget::C, value as u8);
            },
            WordRegisterTarget::DE =>
            {
                self.set_byte(ByteRegisterTarget::D, (value >> 8) as u8);
                self.set_byte(ByteRegisterTarget::E, value as u8);
            },
            WordRegisterTarget::HL =>
            {
                self.set_byte(ByteRegisterTarget::H, (value >> 8) as u8);
                self.set_byte(ByteRegisterTarget::L, value as u8);
            }
        }
    }

}

impl ByteRegisterTarget
{
    fn shift(&self) -> u8
    {
        match self
        {
            ByteRegisterTarget::A => 0,
            ByteRegisterTarget::B => 8,
            ByteRegisterTarget::C => 16,
            ByteRegisterTarget::D => 24,
            ByteRegisterTarget::E => 32,
            ByteRegisterTarget::F => 40,
            ByteRegisterTarget::H => 48,
            ByteRegisterTarget::L => 56,
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