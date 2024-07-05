use crate::memory_bus::MemoryBus;

pub struct CPU<'a>
{
    registers     : CPURegisters,
    program_cntr  : u16,
    halted        : bool,
    curr_opcode   : u8,
    memory_bus    : MemoryBus<'a>
}

pub struct CPURegisters
{
    raw_registers : u64,
    stack_pntr    : u16
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
    HL,
    SP
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
            self.cpu_decode_instruction();
            self.cpu_execute();
        }
    }

    fn inc_program_cntr(&mut self, value : u16)
    {
        self.program_cntr = self.program_cntr.wrapping_add(value);
    }

    fn cpu_fetch_instruction(&mut self)
    {
        self.curr_opcode = self.memory_bus.read(self.program_cntr);
        self.inc_program_cntr(1);
    }
    fn cpu_decode_instruction(&self)
    {

    }
    pub fn cpu_execute(&mut self)
    {
        match self.curr_opcode
        {
            0x00 => self.cpu_nop(),                          // NOP
            0x01 => self.cpu_ld_16_r(WordRegister::BC),      // LD BC, d16
            0x02 => self.cpu_ld_a_mem(WordRegister::BC),     // LD (BC), A
            0x03 => self.alu_inc16(WordRegister::BC),        // INC BC
            0x04 => self.alu_inc(ByteRegister::B),           // INC B
            0x05 => self.alu_dec(ByteRegister::B),           // DEC B
            0x06 => self.cpu_ld_8_r(ByteRegister::B),        // LD B,d8
            0x07 => self.alu_rlc(),                          // RCLA
            0x08 => self.cpu_ld_16_r(WordRegister::SP),      // LD (a16), SP
            0x09 => self.alu_add16(WordRegister::BC),        // ADD HL,BC
            0x0A => self.cpu_ld_mem_a(WordRegister::BC),     // LD A,(BC)
            0x0B => self.alu_dec16(WordRegister::BC),        // DEC BC
            0x0C => self.alu_inc(ByteRegister::C),           // INC C 
            0x0D => self.alu_dec(ByteRegister::C),           // DEC C
            0x0E => self.cpu_ld_8_r(ByteRegister::C),        // LD C,d8
            0x0F => self.alu_rrc(),                          // RRCA
            0x10 => self.cpu_stop(),                         // STOP,
            0x11 => self.cpu_ld_16_r(WordRegister::DE),      // LD BC, d16
            0x12 => self.cpu_ld_a_mem(WordRegister::DE),     // LD (DE), A
            0x13 => self.alu_inc16(WordRegister::DE),        // INC DE
            0x14 => self.alu_inc(ByteRegister::D),           // INC D
            0x15 => self.alu_dec(ByteRegister::D),           // DEC D
            0x16 => self.cpu_ld_8_r(ByteRegister::D),        // LD D,d8
            0x17 => self.alu_rl(),                           // RLA
            0x18 => self.cpu_jr(),                           // JR r8
            0x19 => self.alu_add16(WordRegister::DE),        // ADD HL,DE
            0x1A => self.cpu_ld_mem_a(WordRegister::DE),     // LD A,(DE)
            0x1B => self.alu_dec16(WordRegister::DE),        // DEC DE
            0x1C => self.alu_inc(ByteRegister::E),           // INC E
            0x1D => self.alu_dec(ByteRegister::E),           // DEC E
            0x1E => self.cpu_ld_8_r(ByteRegister::E),        // LD E,d8
            0x1F => self.alu_rr(),                           // RRA,
            0x20 => self.cpu_jr_cond(FlagRegister::Z, false),// JR NZ,r8
            0x21 => self.cpu_ld_16_r(WordRegister::HL),      // LD HL, d16
            0x22 => self.cpu_ld_a_mem_inc(WordRegister::HL), // LD (HL+), A
            0x23 => self.alu_inc16(WordRegister::HL),        // INC HL
            0x24 => self.alu_inc(ByteRegister::H),           // INC H
            0x25 => self.alu_dec(ByteRegister::H),           // DEC H
            0x26 => self.cpu_ld_8_r(ByteRegister::H),        // LD H,d8
            0x27 => self.alu_daa(),                          // DAA
            0x28 => self.cpu_jr_cond(FlagRegister::Z, true), // JR Z,r8
            0x29 => self.alu_add16(WordRegister::HL),        // ADD HL,HL
            0x2A => self.cpu_ld_mem_a(WordRegister::HL),     // LD A,(HL)
            0x2B => self.alu_dec16(WordRegister::HL),        // DEC HL
            0x2C => self.alu_inc(ByteRegister::L),           // INC L
            0x2D => self.alu_dec(ByteRegister::L),           // DEC L
            0x2E => self.cpu_ld_8_r(ByteRegister::L),        // LD L,d8
            0x2F => self.alu_cpl(),                          // CPL
            0x30 => self.cpu_jr_cond(FlagRegister::C, false),// JR NC,r8
            0x31 => self.cpu_ld_r_16(WordRegister::SP),      // LD SP,d16
            0x32 => self.cpu_ld_a_mem_dec(WordRegister::HL), // LD (HL-),A
            0x33 => self.alu_inc16(WordRegister::SP),        // INC SP,
            0x34 => self.alu_inc16(WordRegister::HL),        // INC (HL)
            0x35 => self.alu_dec16(WordRegister::HL),        // DEC (HL)
            0x36 => self.cpu_ld_16_r(WordRegister::HL),      // LD (HL),d8,
            0x37 => self.alu_scf(),                          // SCF,
            0x38 => self.cpu_jr_cond(FlagRegister::C, true), // JR C,r8
            0x39 => self.alu_add16(WordRegister::SP),        // ADD HL,SP,
            0x3A => self.cpu_ld_mem_a_inc(WordRegister::HL), // LD A,(HL-)
            0x3B => self.alu_dec16(WordRegister::SP),        // DEC SP
            0x3C => self.alu_inc(ByteRegister::A),           // INC A
            0x3D => self.alu_dec(ByteRegister::A),           // DEC A
            0x3E => self.cpu_ld_8_r(ByteRegister::A),        // LD A,d8,
            0x3F => self.alu_ccf(),                          // CCF
            _ =>
            {
                println!("Unrecognized opcode {:02X}", self.curr_opcode)
            }
        }
    }

    // =======
    // Misc/Control Instructions
    // =======
    fn cpu_nop(&mut self)
    {

    }
    fn cpu_stop(&mut self)
    {
        self.halted = true;
    }

    // =======
    // Jumps/Calls Instructions
    // =======
    fn cpu_jr(&mut self)
    {
        let offset = self.memory_bus.read(self.program_cntr);
        self.inc_program_cntr(1);
        self.inc_program_cntr(offset as u16);
    }
    fn cpu_jr_cond(&mut self, flag : FlagRegister, condition : bool)
    {
        let offset = self.memory_bus.read(self.program_cntr) as i8;
        self.inc_program_cntr(1);

        if self.registers.read_flag(flag) == condition
        {
            self.inc_program_cntr(offset as u16);
        }
    }

    // =======
    // 8 Bit Load/Store/Move Instructions
    // =======
    fn cpu_ld_a_mem(&mut self, register : WordRegister)
    {
        let address = self.registers.read16(register);
        let value   = self.registers.read(ByteRegister::A);
        self.memory_bus.write(address, value);
    }
    fn cpu_ld_mem_a(&mut self, register : WordRegister)
    {
        let address = self.registers.read16(register);
        let value   = self.memory_bus.read(address);
        self.registers.write(ByteRegister::A, value); 
    }
    fn cpu_ld_a_mem_inc(&mut self, register : WordRegister)
    {
        let address = self.registers.read16(register);
        let value   = self.registers.read(ByteRegister::A);
        self.memory_bus.write(address, value);
        self.registers.write16(register, address.wrapping_add(1));
    }
    fn cpu_ld_mem_a_inc(&mut self, register : WordRegister)
    {
        let address = self.registers.read16(register);
        let value   = self.memory_bus.read(address);
        self.registers.write(ByteRegister::A, value);   
        self.registers.write16(register, address.wrapping_add(1));
    }
    fn cpu_ld_a_mem_dec(&mut self, register : WordRegister)
    {
        let address = self.registers.read16(register);
        let value   = self.registers.read(ByteRegister::A);
        self.memory_bus.write(address, value);
        self.registers.write16(register, address.wrapping_sub(1));
    }
    fn cpu_ld_8_r(&mut self, register : ByteRegister)
    {
        let value = self.memory_bus.read(self.program_cntr);
        self.registers.write(register, value);
        self.inc_program_cntr(1);
    }

    // =======
    // 16 Bit Load/Store/Move Instructions
    // =======
    fn cpu_ld_16_r(&mut self, register : WordRegister)
    {
        let value = self.memory_bus.read16(self.program_cntr);
        self.registers.write16(register, value);
        self.inc_program_cntr(2);  
    }
    fn cpu_ld_r_16(&mut self, register : WordRegister)
    {
        let value   = self.registers.read16(WordRegister::SP);
        let address = self.memory_bus.read16(self.program_cntr);
        self.memory_bus.write16(address, value);
        self.inc_program_cntr(2);
    }

    // =======
    // 8 Bit Arithmetic/Logical Instructions
    // =======
    fn alu_add(&mut self, register : ByteRegister)
    {
        let a_value = self.registers.read(ByteRegister::A);
        let r_value = self.registers.read(register);
        let result  = a_value.wrapping_add(r_value);

        self.registers.write(ByteRegister::A, result);
        self.registers.write_flag(FlagRegister::Z, result == 0);
        self.registers.write_flag(FlagRegister::S, false);
        self.registers.write_flag(FlagRegister::H, (a_value & 0xF) + (r_value & 0xF) > 0xF);
        self.registers.write_flag(FlagRegister::C, (a_value as u16 + r_value as u16) > 0xFF);
    }
    fn alu_sub(&mut self, register : ByteRegister)
    {
        let a_value = self.registers.read(ByteRegister::A);
        let r_value = self.registers.read(register);
        let result  = a_value.wrapping_sub(r_value);

        self.registers.write(ByteRegister::A, result);
        self.registers.write_flag(FlagRegister::Z, result == 0);
        self.registers.write_flag(FlagRegister::S, true);
        self.registers.write_flag(FlagRegister::H, (a_value & 0xF) < (r_value & 0xF));
        self.registers.write_flag(FlagRegister::C, a_value < r_value);
    }
    fn alu_and(&mut self, register : ByteRegister)
    {
        let a_value = self.registers.read(ByteRegister::A);
        let r_value = self.registers.read(register);
        let result  = a_value & r_value;

        self.registers.write(ByteRegister::A, result);
        self.registers.write_flag(FlagRegister::Z, result == 0);
        self.registers.write_flag(FlagRegister::S, false);
        self.registers.write_flag(FlagRegister::H, false);
        self.registers.write_flag(FlagRegister::C, false);
    }
    fn alu_or(&mut self, register : ByteRegister)
    {
        let a_value = self.registers.read(ByteRegister::A);
        let r_value = self.registers.read(register);
        let result  = a_value | r_value;

        self.registers.write(ByteRegister::A, result);
        self.registers.write_flag(FlagRegister::Z, result == 0);
        self.registers.write_flag(FlagRegister::S, false);
        self.registers.write_flag(FlagRegister::H, false);
        self.registers.write_flag(FlagRegister::C, false);
    }
    fn alu_xor(&mut self, register : ByteRegister)
    {
        let a_value = self.registers.read(ByteRegister::A);
        let r_value = self.registers.read(register);
        let result  = a_value ^ r_value;
        
        self.registers.write(ByteRegister::A, result);
        self.registers.write_flag(FlagRegister::Z, result == 0);
        self.registers.write_flag(FlagRegister::S, false);
        self.registers.write_flag(FlagRegister::H, false);
        self.registers.write_flag(FlagRegister::C, false);
    }
    fn alu_cpl(&mut self)
    {
        let a_value = self.registers.read(ByteRegister::A);
        let result = !a_value;

        self.registers.write(ByteRegister::A, result);
        self.registers.write_flag(FlagRegister::S, true);
        self.registers.write_flag(FlagRegister::H, true);
    }
    fn alu_ccf(&mut self)
    {
        let carry_flag = self.registers.read_flag(FlagRegister::C);
        self.registers.write_flag(FlagRegister::C, !carry_flag);
        self.registers.write_flag(FlagRegister::S, false);
        self.registers.write_flag(FlagRegister::H, false);
    }
    fn alu_cp(&mut self, register : ByteRegister)
    {
        let a_value = self.registers.read(ByteRegister::A);
        let r_value = self.registers.read(register);
        let result  = a_value.wrapping_sub(r_value);
        
        self.registers.write_flag(FlagRegister::Z, result == 0);
        self.registers.write_flag(FlagRegister::S, true);
        self.registers.write_flag(FlagRegister::H, (a_value & 0xF) < (r_value & 0xF));
        self.registers.write_flag(FlagRegister::C, a_value < r_value);
    }
    fn alu_inc(&mut self, register : ByteRegister)
    {
        let mut result = self.registers.read(register);
        result = result.wrapping_add(1);

        self.registers.write(register, result);
        self.registers.write_flag(FlagRegister::Z, result == 0);
        self.registers.write_flag(FlagRegister::S, false);
        self.registers.write_flag(FlagRegister::H, (result & 0xF) == 0xF);
    }
    fn alu_dec(&mut self, register : ByteRegister)
    {
        let mut result = self.registers.read(register);
        result = result.wrapping_sub(1);
        
        self.registers.write(register, result);
        self.registers.write_flag(FlagRegister::Z, result == 0);
        self.registers.write_flag(FlagRegister::S, true);
        self.registers.write_flag(FlagRegister::H, (result & 0xF) == 0)
    }
    fn alu_daa(&mut self)
    {
        let mut value  = self.registers.read(ByteRegister::A);
        let mut adjust = 0;
        let carry      = self.registers.read_flag(FlagRegister::C);

        if !self.registers.read_flag(FlagRegister::S)
        {
            if self.registers.read_flag(FlagRegister::H) ||
               (value & 0x0F) > 0x09
            {
                adjust += 0x06;
            }

            if carry ||
               (value > 0x99)
            {
                adjust += 0x60;
                self.registers.write_flag(FlagRegister::C, true);
            }
        }
        else
        {
            if self.registers.read_flag(FlagRegister::H)
            {
                adjust -= 0x06;
            }

            if carry
            {
                adjust -= 0x60;
            }
        }

        value = value.wrapping_add(adjust);
        self.registers.write_flag(FlagRegister::Z, value == 0);
        self.registers.write_flag(FlagRegister::H, false);
    }

    fn alu_scf(&mut self)
    {
        self.registers.write_flag(FlagRegister::C, true);
        self.registers.write_flag(FlagRegister::S, false);
        self.registers.write_flag(FlagRegister::H, false);
    }

    // =======
    // 16 Bit Arithmetic/Logical Instructions
    // =======
    fn alu_inc16(&mut self, register : WordRegister)
    {
        let mut result = self.registers.read16(register);
        result = result.wrapping_add(1);

        self.registers.write16(register, result);
        self.registers.write_flag(FlagRegister::S, false);
        self.registers.write_flag(FlagRegister::H, (result & 0x0FFF) == 0x0FFF);
    }
    fn alu_dec16(&mut self, register : WordRegister)
    {
        let mut result = self.registers.read16(register);
        result = result.wrapping_sub(1);
        
        self.registers.write16(register, result);
        self.registers.write_flag(FlagRegister::S, true);
        self.registers.write_flag(FlagRegister::H, (result & 0xF) == 0);
    }
    fn alu_add16(&mut self, register : WordRegister)
    {
        let hl_value = self.registers.read16(WordRegister::HL);
        let r_value  = self.registers.read16(register);
        let result = hl_value.wrapping_add(r_value);

        self.registers.write16(WordRegister::HL, result);
        self.registers.write_flag(FlagRegister::S, false);
        self.registers.write_flag(FlagRegister::H, (hl_value & 0x07FF) + (r_value & 0x07FF) > 0x07FF);
        self.registers.write_flag(FlagRegister::C, hl_value > 0xFFFF - r_value);
    }

    // =======
    // 8 Bit Rotations/Shifts and Bit Instructions
    // =======
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
    fn alu_rlc(&mut self)
    {
        let value      = self.registers.read(ByteRegister::A);
        let carry_flag = value & 0x80 == 0x80;
        let result     = (value << 1) | (if carry_flag { 1 } else { 0 });
    
        self.registers.write(ByteRegister::A, result);
        self.alu_srflagupdate(result, carry_flag);
    }
    fn alu_rl(&mut self)
    {
        let value          = self.registers.read(ByteRegister::A);
        let carry_flag     = self.registers.read_flag(FlagRegister::C);
        let new_carry_flag = value & 0x80 == 0x80;
        let result         = (value << 1) | (if carry_flag { 1 } else { 0 });
    
        self.registers.write(ByteRegister::A, result);
        self.alu_srflagupdate(result, new_carry_flag);
    }
    fn alu_rrc(&mut self)
    {
        let value      = self.registers.read(ByteRegister::A);
        let carry_flag = value & 0x01 == 0x01;
        let result     = (value >> 1) | (if carry_flag { 0x80 } else { 0 });

        self.registers.write(ByteRegister::A, result);
        self.alu_srflagupdate(result, carry_flag);
    }
    fn alu_rr(&mut self)
    {
        let value          = self.registers.read(ByteRegister::A);
        let carry_flag     = self.registers.read_flag(FlagRegister::C);
        let new_carry_flag = value & 0x01 == 0x01;
        let result         = (value >> 1) | (if carry_flag { 0x80 } else { 0 });

        self.registers.write(ByteRegister::A, result);
        self.alu_srflagupdate(result, new_carry_flag);
    }
    fn alu_sla(&mut self)
    {
        let value      = self.registers.read(ByteRegister::A);
        let carry_flag = value & 0x80 == 0x80;
        let result     = value << 1;
    
        self.registers.write(ByteRegister::A, result);
        self.alu_srflagupdate(result, carry_flag);
    }
    fn alu_sra(&mut self)
    {
        let value      = self.registers.read(ByteRegister::A);
        let carry_flag = value & 0x01 == 0x01;
        let result     = (value >> 1) | (value & 0x80);
    
        self.registers.write(ByteRegister::A, result);
        self.alu_srflagupdate(result, carry_flag);
    }
    fn alu_srl(&mut self)
    {
        let value      = self.registers.read(ByteRegister::A);
        let carry_flag = value & 0x01 == 0x01;
        let result     = value >> 1;
    
        self.registers.write(ByteRegister::A, result);
        self.alu_srflagupdate(result, carry_flag);
    }
}

impl CPURegisters
{
    pub fn new() -> Self
    {
        CPURegisters
        {
            raw_registers : 0x00000000,
            stack_pntr    : 0xFFFE,
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
            WordRegister::SP => self.stack_pntr
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
            },
            WordRegister::SP =>
            {
                self.stack_pntr = value;
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