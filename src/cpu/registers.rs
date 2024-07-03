use super::ByteRegister;
use super::FlagRegister;

pub struct Registers
{
    pub raw_registers : u64
}

const ZERO_FLAG_BYTE_POSITION       : u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION   : u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION : u8 = 5;
const CARRY_FLAG_BYTE_POSITION      : u8 = 4;

impl Registers
{
    pub fn new() -> Self
    {
        Registers
        {
            raw_registers : 0x00000000
        }
    }

    fn read_raw_register(&self, shift : u8) -> u8 { ((self.raw_registers >> shift) & 0xFF) as u8 }
    fn write_raw_register(&mut self, shift : u8, value : u8)
    {
        let mask = !(0xFFu64 << shift);
        self.raw_registers = (self.raw_registers & mask) | ((value as u64) << shift);
    }

    pub fn read_register(&self, label : ByteRegister) -> u8            { self.read_raw_register(label.shift()) }
    pub fn write_register(&mut self, label : ByteRegister, value : u8) { self.write_raw_register(label.shift(), value); }

    pub fn read_flag_register(&self, flag : FlagRegister) -> bool
    {
        let flag_register = self.read_register(ByteRegister::F);
        match flag
        {
            FlagRegister::Z => (flag_register & (1 << ZERO_FLAG_BYTE_POSITION))       != 0,
            FlagRegister::S => (flag_register & (1 << SUBTRACT_FLAG_BYTE_POSITION))   != 0,
            FlagRegister::H => (flag_register & (1 << HALF_CARRY_FLAG_BYTE_POSITION)) != 0,
            FlagRegister::C => (flag_register & (1 << CARRY_FLAG_BYTE_POSITION))      != 0,
        }
    }
    pub fn write_flag_register(&mut self, flag : FlagRegister, value : bool)
    {
        let mut flag_register = self.read_register(ByteRegister::F);
        let bit_position = match flag
        {
            FlagRegister::Z => ZERO_FLAG_BYTE_POSITION,
            FlagRegister::S => SUBTRACT_FLAG_BYTE_POSITION,
            FlagRegister::H => HALF_CARRY_FLAG_BYTE_POSITION,
            FlagRegister::C => CARRY_FLAG_BYTE_POSITION,
        };

        if value
        {
            flag_register |= 1 << bit_position;
        }
        else
        {
            flag_register &= !(1 << bit_position);
        }

        self.write_register(ByteRegister::F, flag_register);
    }

    pub fn read_register_hl(&self) -> u16
    { 
        let high = self.read_register(ByteRegister::H);
        let low  = self.read_register(ByteRegister::L);
        ((high as u16) << 8) | (low as u16)
    }
    pub fn write_register_hl(&mut self, value : u16)
    {
        let high = ((value >> 8) & 0xFF) as u8;
        let low  = (value & 0xFF) as u8;
        self.write_register(ByteRegister::H, high);
        self.write_register(ByteRegister::L, low);
    }
}