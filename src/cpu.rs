use crate::memory_bus::MemoryBus;

pub struct CPU<'a>
{
    pub registers : CPURegisters,
    program_cntr  : u16,
    halted        : bool,
    curr_opcode   : u8,
    memory_bus    : MemoryBus<'a>
}

pub struct CPURegisters
{
    raw_registers : u64,
}

const ZERO_FLAG_BYTE_POSITION       : u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION   : u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION : u8 = 5;
const CARRY_FLAG_BYTE_POSITION      : u8 = 4;
#[derive(Copy, Clone)]
pub enum FlagRegister
{
    Z,
    S,
    H,
    C
}

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

impl<'a> CPU<'a>
{
    pub fn new(memory_bus : MemoryBus<'a>) -> Self
    {
        CPU
        {
            registers     : CPURegisters::new(),
            program_cntr  : 0x0100,
            halted        : false,
            curr_opcode   : 0x0,
            memory_bus : memory_bus
        }
    }

    // =======
    // CPU
    // =======
    pub fn cpu_start(&mut self)
    {
        self.registers.write(ByteRegister::A, 1);
    }

    pub fn cpu_step(&mut self)
    {
        if !self.halted
        {
            self.cpu_fetch_instruction();
            self.cpu_fetch_data();
            self.cpu_execute();
        }
    }
    fn cpu_fetch_instruction(&mut self)
    {
        self.curr_opcode = self.memory_bus.read(self.program_cntr);
        self.program_cntr += 1;
    }
    fn cpu_fetch_data(&self)
    {

    }
    fn cpu_execute(&mut self)
    {
        match self.curr_opcode
        {
            0x00 =>
            {
                println!("a");
            } // NOP
            0x05 =>    // DECB
            {
                println!("b");
            }
            0x0E =>
            {
                println!("c");
            }
            0xAF =>
            {
                println!("d");
            }
            0xC3 =>
            {
                println!("e");
            }
            0xF3 =>
            {
                println!("f");
            }
            _ => {}
        }
    }

    // =======
    // ALU
    // =======
    fn alu_add(&mut self, value : u8)
    {
        let a_value   = self.registers.read(ByteRegister::A);
        let result    = a_value.wrapping_add(value);

        self.registers.write(ByteRegister::A, result);
        self.registers.write_flag(FlagRegister::Z, result == 0);
        self.registers.write_flag(FlagRegister::S, false);
        self.registers.write_flag(FlagRegister::H, (a_value & 0xF) + (value & 0xF) > 0xF);
        self.registers.write_flag(FlagRegister::C, (a_value as u16 + value as u16) > 0xFF);
    }

    fn alu_sub(&mut self, value : u8)
    {
        let a_value = self.registers.read(ByteRegister::A);
        let result = a_value.wrapping_sub(value);

        self.registers.write(ByteRegister::A, result);
        self.registers.write_flag(FlagRegister::Z, result == 0);
        self.registers.write_flag(FlagRegister::S, true);
        self.registers.write_flag(FlagRegister::H, (a_value & 0xF) < (value & 0xF));
        self.registers.write_flag(FlagRegister::C, a_value < value);
    }

    fn alu_and(&mut self, value : u8)
    {
        let a_value = self.registers.read(ByteRegister::A);
        let result  = a_value & value;

        self.registers.write(ByteRegister::A, result);
        self.registers.write_flag(FlagRegister::Z, result == 0);
        self.registers.write_flag(FlagRegister::S, false);
        self.registers.write_flag(FlagRegister::H, false);
        self.registers.write_flag(FlagRegister::C, false);
    }

    fn or(&mut self, value : u8)
    {
        let a_value = self.registers.read(ByteRegister::A);
        let result  = a_value | value;

        self.registers.write(ByteRegister::A, result);
        self.registers.write_flag(FlagRegister::Z, result == 0);
        self.registers.write_flag(FlagRegister::S, false);
        self.registers.write_flag(FlagRegister::H, false);
        self.registers.write_flag(FlagRegister::C, false);
    }

    fn xor(&mut self, value : u8)
    {
        let a_value = self.registers.read(ByteRegister::A);
        let result  = a_value ^ value;
        
        self.registers.write(ByteRegister::A, result);
        self.registers.write_flag(FlagRegister::Z, result == 0);
        self.registers.write_flag(FlagRegister::S, false);
        self.registers.write_flag(FlagRegister::H, false);
        self.registers.write_flag(FlagRegister::C, false);
    }

    fn alu_cp(&mut self, value : u8)
    {
        let a_value = self.registers.read(ByteRegister::A);
        let result  = a_value.wrapping_sub(value);
        
        self.registers.write_flag(FlagRegister::Z, result == 0);
        self.registers.write_flag(FlagRegister::S, true);
        self.registers.write_flag(FlagRegister::H, (a_value & 0xF) < (value & 0xF));
        self.registers.write_flag(FlagRegister::C, a_value < value);
    }

    fn alu_inc(&mut self, value : u8) -> u8
    {
        let result = value.wrapping_add(1);
        
        self.registers.write_flag(FlagRegister::Z, result == 0);
        self.registers.write_flag(FlagRegister::S, false);
        self.registers.write_flag(FlagRegister::H, (value & 0xF) == 0xF);

        return result
    }

    fn alu_dec(&mut self, value : u8) -> u8
    {
        let result = value.wrapping_sub(1);
        
        self.registers.write_flag(FlagRegister::Z, result == 0);
        self.registers.write_flag(FlagRegister::S, true);
        self.registers.write_flag(FlagRegister::H, (value & 0xF) == 0);

        return result
    }

    fn alu_add16(&mut self, value : u16)
    {
        let hl_value = self.registers.read_hl();
        let result = hl_value.wrapping_add(value);

        self.registers.write_hl(result);
        self.registers.write_flag(FlagRegister::S, false);
        self.registers.write_flag(FlagRegister::H, (hl_value & 0x07FF) + (value & 0x07FF) > 0x07FF);
        self.registers.write_flag(FlagRegister::C, hl_value > 0xFFFF - value);
    }

    // TODO
    // pub fn alu_add16imm(registers : &mut Registers, value : u16) -> u16
    //{
    //
    //}

    fn alu_swap(&mut self, value : u8) -> u8
    {
        self.registers.write_flag(FlagRegister::Z,  value == 0);
        self.registers.write_flag(FlagRegister::S, false);
        self.registers.write_flag(FlagRegister::H, false);
        self.registers.write_flag(FlagRegister::C, false);
        (value >> 4) | (value << 4)
    }

    fn alu_srflagupdate(&mut self, value : u8, carry_flag : bool)
    {
        self.registers.write_flag(FlagRegister::Z, value == 0);
        self.registers.write_flag(FlagRegister::S, false);
        self.registers.write_flag(FlagRegister::H, false);
        self.registers.write_flag(FlagRegister::C, carry_flag);
    }
    
    fn alu_rlc(&mut self, value : u8) -> u8
    {
        let carry_flag = value & 0x80 == 0x80;
        let result     = (value << 1) | (if carry_flag { 1 } else { 0 });
    
        self.alu_srflagupdate(result, carry_flag);
        return result
    }

    // TODO
    //pub fn alu_rl(registers : &mut Registers, value : u8) -> u8{}
    //{
    //
    //}

    fn alu_rrc(&mut self, value : u8) -> u8
    {
        let carry_flag = value & 0x01 == 0x01;
        let result     = (value >> 1) | (if carry_flag { 0x80 } else { 0 });

        self.alu_srflagupdate(result, carry_flag);
        return result
    }

    // TODO
    //fn alu_rr(&mut self, a: u8) -> u8
    //{
    //
    //}

    fn alu_sla(&mut self, value : u8) -> u8
    {
        let carry_flag = value & 0x80 == 0x80;
        let result     = value << 1;
    
        self.alu_srflagupdate(result, carry_flag);
        return result;
    }

    fn alu_sra(&mut self, value : u8) -> u8
    {
        let carry_flag = value & 0x01 == 0x01;
        let result     = (value >> 1) | (value & 0x80);
    
        self.alu_srflagupdate(result, carry_flag);
        return result;
    }

    fn alu_srl(&mut self, value : u8) -> u8
    {
        let carry_flag = value & 0x01 == 0x01;
        let result     = value >> 1;
    
        self.alu_srflagupdate(result, carry_flag);
        return result;
    }
    
    fn alu_bit(&mut self, value_a : u8, value_b : u8)
    {
        let zero_flag = value_a & (1 << (value_b as u32)) == 0;
    
        self.registers.write_flag(FlagRegister::S, false);
        self.registers.write_flag(FlagRegister::H, true);
        self.registers.write_flag(FlagRegister::Z, zero_flag);
    }

    // TODO
    //fn alu_daa(&mut self)
    //{
    //
    //}

    // TODO
    //fn cpu_jr(&mut self)
    //{
    //
    //}
}

impl CPURegisters
{
    pub fn new() -> Self
    {
        CPURegisters
        {
            raw_registers : 0x00000000
        }
    }

    fn read_raw(&self, shift : u8) -> u8 { ((self.raw_registers >> shift) & 0xFF) as u8 }
    fn write_raw(&mut self, shift : u8, value : u8)
    {
        let mask = !(0xFFu64 << shift);
        self.raw_registers = (self.raw_registers & mask) | ((value as u64) << shift);
    }

    pub fn read(&self, label : ByteRegister) -> u8            { self.read_raw(label.shift()) }
    pub fn write(&mut self, label : ByteRegister, value : u8) { self.write_raw(label.shift(), value); }

    pub fn read_flag(&self, flag : FlagRegister) -> bool
    {
        let flag_register = self.read(ByteRegister::F);
        match flag
        {
            FlagRegister::Z => (flag_register & (1 << ZERO_FLAG_BYTE_POSITION))       != 0,
            FlagRegister::S => (flag_register & (1 << SUBTRACT_FLAG_BYTE_POSITION))   != 0,
            FlagRegister::H => (flag_register & (1 << HALF_CARRY_FLAG_BYTE_POSITION)) != 0,
            FlagRegister::C => (flag_register & (1 << CARRY_FLAG_BYTE_POSITION))      != 0,
        }
    }
    pub fn write_flag(&mut self, flag : FlagRegister, value : bool)
    {
        let mut flag_register = self.read(ByteRegister::F);
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

        self.write(ByteRegister::F, flag_register);
    }

    pub fn read_hl(&self) -> u16
    { 
        let high = self.read(ByteRegister::H);
        let low  = self.read(ByteRegister::L);
        ((high as u16) << 8) | (low as u16)
    }
    pub fn write_hl(&mut self, value : u16)
    {
        let high = ((value >> 8) & 0xFF) as u8;
        let low  = (value & 0xFF) as u8;
        self.write(ByteRegister::H, high);
        self.write(ByteRegister::L, low);
    }
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