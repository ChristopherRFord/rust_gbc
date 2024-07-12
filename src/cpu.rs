use crate::cpu_enums::InstType;
use crate::cpu_enums::AddrMode;
use crate::cpu_enums::Reg;

use crate::instructions::Mapper;
use crate::instructions::Instruction;
use crate::instructions::Context;
use crate::instructions::util;

use crate::instructions::jump::*;
use crate::instructions::load::*;
use crate::instructions::misc::*;
/*
use crate::instructions::InstructionMapper;
use crate::instructions::Context;
use crate::instructions::load::*;
*/

use crate::bus::*;
use crate::cart::Cart;
use crate::mem::Mem;
use crate::regs::Regs;


pub struct CPU
{
    regs   : Regs, 
    mapper : Mapper,
    int_en : bool,

    curr_opcode : u8,

    // CTX
    ctx_data     : u16,
    ctx_mem_addr : u16,
    dest_is_mem  : bool
}

impl CPU
{
    pub fn new() -> Self
    {
        CPU
        {
            regs   : Regs::new(),
            mapper : Mapper::new(),
            int_en : true,

            curr_opcode : 0x00,

            // memory
            ctx_data     : 0x0000,
            ctx_mem_addr : 0x0000,
            dest_is_mem  : false
        }
    }
    
    pub fn start(&mut self, cart : &mut Cart, mem : &mut Mem)
    {
        self.regs.write(Reg::A, 0x01);
        self.regs.write(Reg::PC, 0x0100);

        loop
        {
            self.step(cart, mem);
        }
    }

    fn print_step(&self, cart : &mut Cart, inst : &Instruction)
    {
        let pc = self.regs.read(Reg::PC);
        println!
        (
            //r#"-- Instruction Start
            "{:04X}: {} {} ({:02X} {:02X} {:02X} {:02X}) A: {:02X} BC: {:04X} DE: {:04X} HL: {:04X} SP: {:04X}",
            pc,
            util::name_from_instruction_type(&inst.inst_type),
            util::name_from_addr_mode(&inst.addr_mode),
            self.curr_opcode,
            cart.read8(pc + 1),
            cart.read8(pc + 2),
            cart.read8(pc + 3),
            self.regs.read(Reg::A) as u8,
            self.regs.read(Reg::BC),
            self.regs.read(Reg::DE),
            self.regs.read(Reg::HL),
            self.regs.read(Reg::SP)
        );
    }

    fn step(&mut self, cart : &mut Cart, mem : &mut Mem)
    {
        self.clear();
        self.fetch_instruction(cart);

        let instruction = self.mapper.instruction_from_opcode(self.curr_opcode);
        self.print_step(cart,  &instruction);

        self.regs.inc_pc(1);

        self.fetch_data(cart, mem);
        self.execute(cart, mem);
    }
    fn clear(&mut self)
    {
        self.ctx_data = 0;
        self.ctx_mem_addr = 0;
        self.dest_is_mem = false;
    }
    fn fetch_instruction(&mut self, cart : &mut Cart)
    {
        self.curr_opcode = cart.read8(self.regs.read(Reg::PC));
    }
    fn fetch_data(&mut self, cart : &mut Cart, mem : &mut Mem)
    {
        let instruction = self.mapper.instruction_from_opcode(self.curr_opcode);
        self.dest_is_mem = false;
        match instruction.addr_mode
        {
            AddrMode::IMP    => return,

            AddrMode::R_D16 |
            AddrMode::D16 =>
            {
                let address = self.regs.read(Reg::PC);
                self.ctx_data = bus::read16(cart, mem, &mut self.regs, address);
                self.regs.inc_pc(2);
            },

            AddrMode::A16_R |
            AddrMode::D16_R  =>
            {
                let pc      = self.regs.read(Reg::PC);
                let address = bus::read16(cart, mem, &mut self.regs, pc);
                let value   = self.regs.read(instruction.reg_2);

                self.dest_is_mem  = true;
                self.ctx_mem_addr = address;
                self.ctx_data     = value; 
                self.regs.inc_pc(2);
            },

            AddrMode::R_R    => self.TODO_fd(instruction),
            AddrMode::MR_R   => self.TODO_fd(instruction),
            AddrMode::R      => self.ctx_data = self.regs.read(instruction.reg_1),
            AddrMode::R_D8   =>
            {
                let pc = self.regs.read(Reg::PC);
                self.ctx_data = bus::read16(cart, mem, &mut self.regs, pc);
                self.regs.inc_pc(1);
            }
            AddrMode::R_MR   => self.TODO_fd(instruction),
            AddrMode::R_HLI  => self.TODO_fd(instruction),
            AddrMode::R_HLD  => self.TODO_fd(instruction),
            AddrMode::HLI_R  => self.TODO_fd(instruction),
            AddrMode::HLD_R  => self.TODO_fd(instruction),
            AddrMode::R_A8   => self.TODO_fd(instruction),
            AddrMode::A8_R   =>
            {
                let pc      = self.regs.read(Reg::PC);
                let address = bus::read16(cart, mem, &mut self.regs, pc) as u8;
                let value   = self.regs.read(instruction.reg_2);

                self.dest_is_mem  = true;
                self.ctx_mem_addr = address as u16;
                self.ctx_data     = value; 
                self.regs.inc_pc(1);
            },
            AddrMode::HL_SPR => self.TODO_fd(instruction),
            AddrMode::D8     => self.TODO_fd(instruction),
            AddrMode::MR_D8  => self.TODO_fd(instruction),
            AddrMode::MR     => self.TODO_fd(instruction),
            AddrMode::R_A16  => self.TODO_fd(instruction)
        }
    }

    fn execute(&mut self, cart : &mut Cart, mem : &mut Mem)
    {
        let instruction = self.mapper.instruction_from_opcode(self.curr_opcode);
        let mut ctx = Context
        {
            inst_type : instruction.inst_type,
            addr_mode : instruction.addr_mode,
            reg_1     : instruction.reg_1,
            reg_2     : instruction.reg_2,
            cond_type : instruction.cond_type,
            param     : instruction.param,

            data        : self.ctx_data,
            mem_addr    : self.ctx_mem_addr,
            dest_is_mem : self.dest_is_mem,
            int_en      : &mut self.int_en,

            regs   : &mut self.regs,
            cart   : cart,
            mem    : mem
        };

        match instruction.inst_type
        {
            InstType::NONE => println!("NONE"),
            InstType::NOP  => misc::nop(),
            InstType::LD   => load::ld(&mut ctx),
            InstType::INC  => self.TODO_exe("INC"),
            InstType::DEC  => self.TODO_exe("DEC"),
            InstType::RLCA => self.TODO_exe("RCLA"),
            InstType::ADD  => self.TODO_exe("ADD"),
            InstType::RRCA => self.TODO_exe("RRCA"),
            InstType::STOP => self.TODO_exe("STOP"),
            InstType::RLA  => self.TODO_exe("RLA"),
            InstType::JR   => self.TODO_exe("JR"),
            InstType::RRA  => self.TODO_exe("RRA"),
            InstType::DAA  => self.TODO_exe("DAA"),
            InstType::CPL  => self.TODO_exe("CPL"),
            InstType::SCF  => self.TODO_exe("SCF"),
            InstType::CCF  => self.TODO_exe("CCF"),
            InstType::HALT => self.TODO_exe("HALT"),
            InstType::ADC  => self.TODO_exe("ADC"),
            InstType::SUB  => self.TODO_exe("SUB"),
            InstType::SBC  => self.TODO_exe("SBC"),
            InstType::AND  => self.TODO_exe("AND"),
            InstType::XOR  => self.TODO_exe("XOR"),
            InstType::OR   => self.TODO_exe("OR"),
            InstType::CP   => self.TODO_exe("CP"),
            InstType::POP  => self.TODO_exe("POP"),
            InstType::JP   => jump::jp(&mut ctx),
            InstType::PUSH => self.TODO_exe("PUSH"),
            InstType::RET  => self.TODO_exe("RET"),
            InstType::CB   => self.TODO_exe("CB"),
            InstType::CALL => jump::call(&mut ctx),
            InstType::RETI => self.TODO_exe("RETI"),
            InstType::LDH  => load::ldh(&mut ctx),
            InstType::JPHL => self.TODO_exe("JPHL"),
            InstType::DI   => misc::di(&mut ctx),
            InstType::EI   => self.TODO_exe("EI"),
            InstType::RST  => self.TODO_exe("RST"),
            InstType::ERR  => self.TODO_exe("ERR"),
            InstType::RLC  => self.TODO_exe("RLC"),
            InstType::RRC  => self.TODO_exe("RRC"),
            InstType::RL   => self.TODO_exe("RL"),
            InstType::RR   => self.TODO_exe("RR"),
            InstType::SLA  => self.TODO_exe("SLA"),
            InstType::SRA  => self.TODO_exe("SRA"),
            InstType::SWAP => self.TODO_exe("WRAP"),
            InstType::SRL  => self.TODO_exe("SRL"),
            InstType::BIT  => self.TODO_exe("BIT"),
            InstType::RES  => self.TODO_exe("RES"),
            InstType::SET  => self.TODO_exe("SET")
        }
    }

    // ==========================
    // TODO
    // ==========================
    fn TODO_fd(&self, instruction : &Instruction)
    {
        let opcode = format!("Unresolved Fetch Data Opcode {:02X} {} TODO", 
                             instruction.inst_type as u8,
                             util::name_from_addr_mode(&instruction.addr_mode));
        println!("{}", opcode);
        panic!("{}", opcode);
    }
    
    fn TODO_exe(&self, msg : &str)
    {
        let opcode = format!("Unresolved Execute {} TODO", msg);
        println!("{}", opcode);
        panic!("{}", opcode);
    }

}