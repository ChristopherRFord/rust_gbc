use crate::cpu_enums::Reg;
use crate::cpu_enums::RegF;

pub struct Regs
{
    raw : u64,
    sp  : u16,
    pc  : u16,
    ie  : u8
}

impl Regs
{
    pub fn new() -> Self
    {
        Regs
        {
            raw : 0x00000000,
            sp  : 0x0000,
            pc  : 0x0000,
            ie  : 0x00
        }
    }

    pub fn read(&self, reg : Reg) -> u16
    {
        match reg
        {
            Reg::NONE => 0xFF,
            Reg::A    => self.read_a() as u16,
            Reg::F    => self.read_f() as u16,
            Reg::B    => self.read_b() as u16,
            Reg::C    => self.read_c() as u16,
            Reg::D    => self.read_d() as u16,
            Reg::E    => self.read_e() as u16,
            Reg::H    => self.read_h() as u16,
            Reg::L    => self.read_l() as u16,

            Reg::AF   => self.read_af(),
            Reg::BC   => self.read_bc(),
            Reg::DE   => self.read_de(),
            Reg::HL   => self.read_hl(),

            Reg::SP   => self.read_sp(),
            Reg::PC   => self.read_pc(),
            Reg::IE   => self.read_ie() as u16,
        }
    }
    pub fn write(&mut self, reg : Reg, value : u16)
    {
        match reg
        {
            Reg::NONE => return,
            Reg::A    => self.write_a(value as u8),
            Reg::F    => self.write_f(value as u8),
            Reg::B    => self.write_b(value as u8),
            Reg::C    => self.write_c(value as u8),
            Reg::D    => self.write_d(value as u8),
            Reg::E    => self.write_e(value as u8),
            Reg::H    => self.write_h(value as u8),
            Reg::L    => self.write_l(value as u8),

            Reg::AF   => self.write_af(value),
            Reg::BC   => self.write_bc(value),
            Reg::DE   => self.write_de(value),
            Reg::HL   => self.write_hl(value),
            
            Reg::SP   => self.write_sp(value),
            Reg::PC   => self.write_pc(value),
            Reg::IE   => self.write_ie(value as u8)
        }
    }

    // ==========================
    // 8 Bit Read/Write
    // ==========================
    pub fn read_a(&self) -> u8 { ((self.raw >> 56) & 0xFF) as u8 }
    pub fn write_a(&mut self, value: u8)
    {
        self.raw &= !(0xFF << 56);
        self.raw |= (value as u64) << 56;
    }

    pub fn read_f(&self) -> u8 {  ((self.raw >> 48) & 0xFF) as u8 }
    pub fn write_f(&mut self, value: u8)
    {
        self.raw &= !(0xFF << 48);
        self.raw |= (value as u64) << 48;
    }

    pub fn read_b(&self) -> u8 {  ((self.raw >> 40) & 0xFF) as u8 }
    pub fn write_b(&mut self, value: u8)
    {
        self.raw &= !(0xFF << 40);
        self.raw |= (value as u64) << 40;
    }

    pub fn read_c(&self) -> u8 {  ((self.raw >> 32) & 0xFF) as u8 }
    pub fn write_c(&mut self, value: u8)
    {
        self.raw &= !(0xFF << 32);
        self.raw |= (value as u64) << 32;
    }

    pub fn read_d(&self) -> u8 {  ((self.raw >> 24) & 0xFF) as u8 }
    pub fn write_d(&mut self, value: u8)
    {
        self.raw &= !(0xFF << 24);
        self.raw |= (value as u64) << 24;
    }

    fn read_e(&self) -> u8 {  ((self.raw >> 16) & 0xFF) as u8 }
    fn write_e(&mut self, value: u8)
    {
        self.raw &= !(0xFF << 16);
        self.raw |= (value as u64) << 16;
    }

    fn read_h(&self) -> u8 {  ((self.raw >> 16) & 0xFF) as u8 }
    fn write_h(&mut self, value: u8)
    {
        self.raw &= !(0xFF << 8);
        self.raw |= (value as u64) << 8;
    }

    fn read_l(&self) -> u8 {  ((self.raw >> 16) & 0xFF) as u8 }
    fn write_l(&mut self, value: u8)
    {
        self.raw &= !0xFF;
        self.raw |= value as u64;
    }

    // ==========================
    // 16 Bit Read/Write
    // ==========================
    fn read_af(&self) -> u16 { ((self.read_a() as u16) << 8) | self.read_f() as u16 }
    fn write_af(&mut self, value: u16)
    {
        self.write_a((value >> 8) as u8);
        self.write_f(value as u8);
    }

    fn read_bc(&self) -> u16 { ((self.read_b() as u16) << 8) | self.read_c() as u16 }
    fn write_bc(&mut self, value: u16)
    {
        self.write_b((value >> 8) as u8);
        self.write_c(value as u8);
    }

    fn read_de(&self) -> u16 { ((self.read_d() as u16) << 8) | self.read_e() as u16 }
    fn write_de(&mut self, value: u16)
    {
        self.write_d((value >> 8) as u8);
        self.write_e(value as u8);
    }

    fn read_hl(&self) -> u16{ ((self.read_h() as u16) << 8) | self.read_l() as u16 }
    fn write_hl(&mut self, value: u16) 
    {
        self.write_h((value >> 8) as u8);
        self.write_l(value as u8);
    }

    fn read_sp(&self) -> u16 { self.sp }
    fn write_sp(&mut self, value: u16) { self.sp = value; }

    // ==========================
    // Flag Register
    // ==========================
    pub fn read_flag(&self, flag : RegF) -> bool
    {
        let flag_register = self.read_f();
        match flag
        {
            RegF::Z => (flag_register & (1 << 0x7)) != 0,
            RegF::S => (flag_register & (1 << 0x6)) != 0,
            RegF::H => (flag_register & (1 << 0x5)) != 0,
            RegF::C => (flag_register & (1 << 0x4)) != 0,
        }
    } 
    pub fn write_flag(&mut self, flag : RegF, value : bool)
    {
        let mut flag_register = self.read_f() as u8;
        let bit_position = match flag
        {
            RegF::Z => 0x7,
            RegF::S => 0x6,
            RegF::H => 0x5,
            RegF::C => 0x4,
        };

        if value
        {
            flag_register |= 1 << bit_position;
        }
        else
        {
            flag_register &= !(1 << bit_position);
        }

        self.write_f(flag_register);
    }

    // ==========================
    // Program Counter
    // ==========================
    fn read_pc(&self) -> u16 { self.pc }
    fn write_pc(&mut self, value: u16) { self.pc = value; }
    pub fn inc_pc(&mut self, value : u16) { self.pc = self.pc.wrapping_add(value); }
    pub fn dec_pc(&mut self, value : u16) { self.pc = self.pc.wrapping_add(value); }

    // ==========================
    // IE Register
    // ==========================
    pub fn read_ie(&self) -> u8 { self.ie  }
    pub fn write_ie(&mut self, value : u8) { self.ie = value; }
}