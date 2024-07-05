use crate::memory_bus::MemoryBus;

pub struct CPU<'a>
{
    registers     : CPURegisters,
    program_cntr  : u16,
    stack_pntr    : u16,
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

#[derive(Copy, Clone)]
pub enum WordRegister
{
    AF,
    BC,
    DE,
    HL
}

impl<'a> CPU<'a>
{
    pub fn new(memory_bus : MemoryBus<'a>) -> Self
    {
        CPU
        {
            registers     : CPURegisters::new(),
            program_cntr  : 0x0100,
            stack_pntr    : 0xFFFE,
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
            self.cpu_decode_instruction();
            self.cpu_execute();
        }
    }

    fn cpu_fetch_instruction(&mut self)
    {
        self.curr_opcode = self.memory_bus.read(self.program_cntr);
        self.program_cntr.wrapping_add(1);
    }
    fn cpu_decode_instruction(&self)
    {

    }
    pub fn cpu_execute(&mut self)
    {
        match self.curr_opcode
        {
            0x00 => self.cpu_nop(),                        // NOP
            0x01 => self.cpu_ld_d16(WordRegister::BC),     // LD BC, d16
            0x02 => self.cpu_ld_a_mem(WordRegister::BC),   // LD (BC), A
            0x03 => self.cpu_inc16(WordRegister::BC),      // INC BC
            0x04 => self.cpu_inc(ByteRegister::B),         // INC B
            0x05 => self.cpu_dec(ByteRegister::B),         // DEC B
            0x06 => self.cpu_ld_d8(ByteRegister::B),       // LD B,d8
            0x07 => self.cpu_rlca(),                       // RCLA
            0x08 => self.cpu_ld_a16_sp(),                  // LD (a16), SP
            0x09 => self.cpu_add16(WordRegister::BC),      // ADD HL,BC
            0x0A => self.cpu_ld_a(WordRegister::BC),       // LD A,(BC)
            0x0B => self.cpu_dec16(WordRegister::BC),      // DEC BC
            0x0C => self.cpu_inc(ByteRegister::C),         // INC C 
            0x0D => self.cpu_dec(ByteRegister::C),         // DEC C
            0x0E => self.cpu_ld_d8(ByteRegister::C),       // LD C,d8
            0x0F => self.cpu_rrca(),                       // RRCA
            0x10 => self.cpu_stop(),                       // STOP,
            0x11 => self.cpu_ld_d16(WordRegister::DE),     // LD BC, d16
            0x12 => self.cpu_ld_a_mem(WordRegister::DE),   // LD (DE), A
            0x13 => self.cpu_inc16(WordRegister::DE),      // INC DE
            0x14 => self.cpu_inc(ByteRegister::D),         // INC D
            0x15 => self.cpu_dec(ByteRegister::D),         // DEC B
            0x16 => self.cpu_ld_d8(ByteRegister::D),       // LD D,d8
            0x17 => self.cpu_rla(),                        // RLA
            0x18 => self.cpu_jr(),                         // JR r8
            0x19 => self.cpu_add16(WordRegister::DE),      // ADD HL,DE
            0x1A => self.cpu_ld_a(WordRegister::DE),       // LD A,(DE)
            0x1B => self.cpu_dec16(WordRegister::DE),      // DEC DE
            0x1C => self.cpu_inc(ByteRegister::E),         // INC E
            0x1D => self.cpu_dec(ByteRegister::E),         // DEC E
            0x1E => self.cpu_ld_d8(ByteRegister::E),       // LD E,d8
            0x1F => self.cpu_rra(),                        // RRA
            _ =>
            {
                println!("Unrecognized opcode {:02X}", self.curr_opcode)
            }
        }
    }

    // =======
    // CPU Instructions
    // =======
    fn cpu_nop(&mut self) {}
    fn cpu_ld_d16(&mut self, register : WordRegister)
    {
        let value = self.memory_bus.read16(self.program_cntr);
        self.registers.write16(register, value);
        self.program_cntr.wrapping_add(2);   
    }
    fn cpu_ld_a_mem(&mut self, register : WordRegister)
    {
        let address = self.registers.read16(register);
        let value   = self.registers.read(ByteRegister::A);
        self.memory_bus.write(address, value);
    }
    fn cpu_ld_a(&mut self, register : WordRegister)
    {
        let address = self.registers.read16(register);
        let value   = self.memory_bus.read(address);
        self.registers.write(ByteRegister::A, value); 
    }
    fn cpu_ld_a16_sp(&mut self)
    {
        let address = self.memory_bus.read16(self.program_cntr);
        self.memory_bus.write16(address, self.stack_pntr);
        self.program_cntr.wrapping_add(2);
    }
    fn cpu_inc(&mut self, register : ByteRegister)
    {
        let mut value = self.registers.read(register);
        value         = self.alu_inc(value);
        self.registers.write(register, value); 
    }
    fn cpu_inc16(&mut self, register : WordRegister)
    {
        let mut value = self.registers.read16(register);
        value         = self.alu_inc16(value);
        self.registers.write16(register, value); 
    }
    fn cpu_dec(&mut self, register : ByteRegister)
    {
        let mut value = self.registers.read(register);
        value         = self.alu_dec(value);
        self.registers.write(register, value);         
    }
    fn cpu_dec16(&mut self, register : WordRegister)
    {
        let mut value = self.registers.read16(register);
        value         = self.alu_dec16(value);
        self.registers.write16(register, value); 
    }
    fn cpu_add(&mut self, register : ByteRegister)
    {
        let value = self.registers.read(register);
        self.alu_add(value);
        self.registers.write(ByteRegister::A, value);
    }
    fn cpu_add16(&mut self, register : WordRegister)
    {
        let value = self.registers.read16(register);
        self.alu_add16(value);
        self.registers.write16(WordRegister::HL, value);
    }
    fn cpu_ld_d8(&mut self, register : ByteRegister)
    {
        let value = self.memory_bus.read(self.program_cntr);
        self.registers.write(register, value);
        self.program_cntr.wrapping_add(1);
    }

    fn cpu_stop(&mut self)
    {
        self.halted = true;
    }
    fn cpu_rlca(&mut self)
    {
        let mut value = self.registers.read(ByteRegister::A);
        value          = self.alu_rlc(value);
        self.registers.write(ByteRegister::A, value);
    }
    fn cpu_rrca(&mut self)
    {
        let mut value = self.registers.read(ByteRegister::A);
        value         = self.alu_rrc(value);
        self.registers.write(ByteRegister::A, value);
    }
    fn cpu_rla(&mut self)
    {
        let mut value = self.registers.read(ByteRegister::A);
        value         = self.alu_rl(value);
        self.registers.write(ByteRegister::A, value);
    }
    fn cpu_rra(&mut self)
    {
        let mut value = self.registers.read(ByteRegister::A);
        value = self.alu_rr(value);
        self.registers.write(ByteRegister::A, value);
    }
    fn cpu_jr(&mut self)
    {
        let offset = self.memory_bus.read(self.program_cntr);
        self.program_cntr.wrapping_add(1);
        self.program_cntr.wrapping_add(offset as u16);
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

    fn alu_or(&mut self, value : u8)
    {
        let a_value = self.registers.read(ByteRegister::A);
        let result  = a_value | value;

        self.registers.write(ByteRegister::A, result);
        self.registers.write_flag(FlagRegister::Z, result == 0);
        self.registers.write_flag(FlagRegister::S, false);
        self.registers.write_flag(FlagRegister::H, false);
        self.registers.write_flag(FlagRegister::C, false);
    }

    fn alu_xor(&mut self, value : u8)
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

    fn alu_inc16(&mut self, value : u16) -> u16
    {
        let result = value.wrapping_add(1);
        
        self.registers.write_flag(FlagRegister::S, false);
        self.registers.write_flag(FlagRegister::H, (value & 0x0FFF) == 0x0FFF);

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

    fn alu_dec16(&mut self, value : u16) -> u16
    {
        let result = value.wrapping_sub(1);
        
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
        result
    }

    fn alu_rl(&mut self, value: u8) -> u8
    {
        let carry_flag = self.registers.read_flag(FlagRegister::C);
        let new_carry_flag = value & 0x80 == 0x80;
        let result = (value << 1) | (if carry_flag { 1 } else { 0 });
    
        self.alu_srflagupdate(result, new_carry_flag);
        result
    }

    fn alu_rrc(&mut self, value : u8) -> u8
    {
        let carry_flag = value & 0x01 == 0x01;
        let result     = (value >> 1) | (if carry_flag { 0x80 } else { 0 });

        self.alu_srflagupdate(result, carry_flag);
        result
    }

    fn alu_rr(&mut self, value : u8) -> u8
    {
        let carry_flag     = self.registers.read_flag(FlagRegister::C);
        let new_carry_flag = value & 0x01 == 0x01;
        let result         = (value >> 1) | (if carry_flag { 0x80 } else { 0 });

        self.alu_srflagupdate(result, new_carry_flag);
        result
    }

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

    pub fn read16(&self, register : WordRegister) -> u16
    {
        match register
        {
            WordRegister::AF => (self.read(ByteRegister::A) as u16) << 8 | self.read(ByteRegister::F) as u16,
            WordRegister::BC => (self.read(ByteRegister::B) as u16) << 8 | self.read(ByteRegister::C) as u16,
            WordRegister::DE => (self.read(ByteRegister::D) as u16) << 8 | self.read(ByteRegister::E) as u16,
            WordRegister::HL => (self.read(ByteRegister::H) as u16) << 8 | self.read(ByteRegister::L) as u16,
        }
    }
    pub fn write16(&mut self, register : WordRegister, value : u16)
    {
        match register
        {
            WordRegister::AF => 
            {
                self.write(ByteRegister::A, (value >> 8) as u8);
                self.write(ByteRegister::F, value as u8);
            },
            WordRegister::BC =>
            {
                self.write(ByteRegister::B, (value >> 8) as u8);
                self.write(ByteRegister::C, value as u8);
            },
            WordRegister::DE =>
            {
                self.write(ByteRegister::D, (value >> 8) as u8);
                self.write(ByteRegister::E, value as u8);
            },
            WordRegister::HL =>
            {
                self.write(ByteRegister::H, (value >> 8) as u8);
                self.write(ByteRegister::L, value as u8);
            }
        }       
    }

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