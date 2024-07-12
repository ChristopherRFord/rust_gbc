//pub mod load;
pub mod misc;
pub mod jump;
pub mod load;

use crate::cpu_enums::InstType;
use crate::cpu_enums::AddrMode;
use crate::cpu_enums::Reg;
use crate::cpu_enums::CondType;

use crate::cart::Cart;
use crate::mem::Mem;
use crate::regs::Regs;

pub struct Context<'a>
{
    // INSTRUCTION
    pub inst_type : InstType,
    pub addr_mode : AddrMode,
    pub reg_1     : Reg,
    pub reg_2     : Reg,
    pub cond_type : CondType,
    pub param     : u16,

    // CPU
    pub data        : u16,
    pub mem_addr    : u16,
    pub dest_is_mem : bool,
    pub int_en      : &'a mut bool,

    pub regs : &'a mut Regs,
    pub cart : &'a mut Cart,
    pub mem  : &'a mut Mem
}

#[derive(Copy, Clone)]
pub struct Instruction
{
    pub inst_type : InstType,
    pub addr_mode : AddrMode,
    pub reg_1     : Reg,
    pub reg_2     : Reg,
    pub cond_type : CondType,
    pub param     : u16
}

pub struct Mapper
{
    instructions : [Instruction; 0x100]
}

fn add
(
    instructions : &mut [Instruction; 0x100], 
    index        : u8, 
    inst_type    : InstType, 
    addr_mode    : AddrMode, 
    reg_1        : Reg, 
    reg_2        : Reg, 
    cond_type    : CondType, 
    param        : u16
)
{
    instructions[index as usize] = Instruction
    {
        inst_type : inst_type,
        addr_mode : addr_mode,
        reg_1     : reg_1,
        reg_2     : reg_2,
        cond_type : cond_type,
        param     : param
    }
}

impl Mapper
{
    pub fn new() -> Self
    {
        let default_instruction = Instruction
        {
            inst_type : InstType::NOP,
            addr_mode : AddrMode::IMP,
            reg_1     : Reg::NONE,
            reg_2     : Reg::NONE,
            cond_type : CondType::NONE,
            param     : 0,
        };

        let mut instructions = [default_instruction; 0x100];

        add(&mut instructions, 0x00, InstType::NOP, AddrMode::IMP,   Reg::NONE, Reg::NONE, CondType::NONE, 0);
        add(&mut instructions, 0x31, InstType::LD,  AddrMode::R_D16, Reg::SP,   Reg::NONE, CondType::NONE, 0);
        add(&mut instructions, 0x3E, InstType::LD,  AddrMode::R_D8,  Reg::A,    Reg::NONE, CondType::NONE, 0);
        add(&mut instructions, 0xC3, InstType::JP,  AddrMode::D16,   Reg::NONE, Reg::NONE, CondType::NONE, 0);
        add(&mut instructions, 0xCD, InstType::CALL, AddrMode::D16,  Reg::NONE, Reg::NONE, CondType::NONE, 0);
        add(&mut instructions, 0xE0, InstType::LDH, AddrMode::A8_R,  Reg::NONE, Reg::A,    CondType::NONE, 0);
        add(&mut instructions, 0xEA, InstType::LD,  AddrMode::A16_R, Reg::NONE, Reg::A,    CondType::NONE, 0);
        add(&mut instructions, 0xF3, InstType::DI,  AddrMode::IMP,   Reg::NONE, Reg::NONE, CondType::NONE, 0);

        Mapper
        {
            instructions : instructions
        }
    }

    pub fn instruction_from_opcode(&self, opcode : u8) -> &Instruction
    {
        &self.instructions[opcode as usize]
    }
}

pub mod util
{
    use crate::cpu_enums::InstType;
    use crate::cpu_enums::AddrMode;

    pub fn name_from_instruction_type(inst : &InstType) -> &str
    {
        match inst
        {
            InstType::NONE => "NONE",
            InstType::NOP  => "NOP",
            InstType::LD   => "LD",
            InstType::INC  => "INC",
            InstType::DEC  => "DEC",
            InstType::RLCA => "RLCA",
            InstType::ADD  => "ADD",
            InstType::RRCA => "RRCA",
            InstType::STOP => "STOP",
            InstType::RLA  => "RLA",
            InstType::JR   => "JR",
            InstType::RRA  => "RRA",
            InstType::DAA  => "DAA",
            InstType::CPL  => "CPL",
            InstType::SCF  => "SCF",
            InstType::CCF  => "CCF",
            InstType::HALT => "HALT",
            InstType::ADC  => "ADC",
            InstType::SUB  => "SUB",
            InstType::SBC  => "SBC",
            InstType::AND  => "AND",
            InstType::XOR  => "XOR",
            InstType::OR   => "OR",
            InstType::CP   => "CP",
            InstType::POP  => "POP",
            InstType::JP   => "JP",
            InstType::PUSH => "PUSH",
            InstType::RET  => "RET",
            InstType::CB   => "CB",
            InstType::CALL => "CALL",
            InstType::RETI => "RETI",
            InstType::LDH  => "LDH",
            InstType::JPHL => "JPHL",
            InstType::DI   => "DI",
            InstType::EI   => "EI",
            InstType::RST  => "RST",
            InstType::ERR  => "ERR",
            InstType::RLC  => "RLC",
            InstType::RRC  => "RRC",
            InstType::RL   => "RL",
            InstType::RR   => "RR",
            InstType::SLA  => "SLA",
            InstType::SRA  => "SRA",
            InstType::SWAP => "SWAP",
            InstType::SRL  => "SRL",
            InstType::BIT  => "BIT",
            InstType::RES  => "RES",
            InstType::SET  => "SET"
        }
    }

    pub fn name_from_addr_mode(addr : &AddrMode) -> &str
    {
        match addr
        {
            AddrMode::IMP    => "IMP",
            AddrMode::R_D16  => "R_D16",
            AddrMode::R_R    => "R_R",
            AddrMode::MR_R   => "MR_R",
            AddrMode::R      => "R",
            AddrMode::R_D8   => "R_D8",
            AddrMode::R_MR   => "R_MR",
            AddrMode::R_HLI  => "R_HLI",
            AddrMode::R_HLD  => "R_HLD",
            AddrMode::HLI_R  => "HLI_R",
            AddrMode::HLD_R  => "HLD_R",
            AddrMode::R_A8   => "R_A8",
            AddrMode::A8_R   => "A8_R",
            AddrMode::HL_SPR => "HL_SPR",
            AddrMode::D16    => "D16",
            AddrMode::D8     => "D8",
            AddrMode::D16_R  => "D16_R",
            AddrMode::MR_D8  => "MR_D8",
            AddrMode::MR     => "MR",
            AddrMode::A16_R  => "A16_R",
            AddrMode::R_A16  => "R_A16"
        }
    }
}