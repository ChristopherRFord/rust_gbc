#[derive(Copy, Clone)]
pub enum InstType
{
    NONE,
    NOP,
    LD,
    INC,
    DEC,
    RLCA,
    ADD,
    RRCA,
    STOP,
    RLA,
    JR,
    RRA,
    DAA,
    CPL,
    SCF,
    CCF,
    HALT,
    ADC,
    SUB,
    SBC,
    AND,
    XOR,
    OR,
    CP,
    POP,
    JP,
    PUSH,
    RET,
    CB,
    CALL,
    RETI,
    LDH,
    JPHL,
    DI,
    EI,
    RST,
    ERR,
    RLC, 
    RRC,
    RL, 
    RR,
    SLA, 
    SRA,
    SWAP, 
    SRL,
    BIT, 
    RES, 
    SET
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum AddrMode
{
    IMP,
    R_D16,
    R_R,
    MR_R,
    R,
    R_D8,
    R_MR,
    R_HLI,
    R_HLD,
    HLI_R,
    HLD_R,
    R_A8,
    A8_R,
    HL_SPR,
    D16,
    D8,
    D16_R,
    MR_D8,
    MR,
    A16_R,
    R_A16
}

#[derive(Copy, Clone, PartialEq)]
pub enum Reg
{
    NONE,
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
    IE
}

#[derive(Copy, Clone)]
pub enum CondType
{
    NONE,
    NZ,
    Z,
    NC,
    C
}

#[derive(Copy, Clone)]
pub enum RegF
{
    Z,
    S,
    H,
    C
}

pub enum RamType
{
    WRAM,
    HRAM
}