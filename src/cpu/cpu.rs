use crate::cart::Cart;
use crate::mmu::MMU;

use crate::bus::Bus;
use crate::cpu::pcntr::PCntr;
use crate::cpu::registers::Registers;
use crate::cpu::registers::RegF;
use crate::cpu::registers::Reg8;
use crate::cpu::registers::Reg16;

use crate::cpu::instr::alu::*;
use crate::cpu::instr::jump::*;
use crate::cpu::instr::ld::*;
use crate::cpu::instr::misc::*;
use crate::cpu::instr::shift::*;

pub struct CPU<'a>
{
    pcntr  : PCntr,
    opcode : u8,
    regs   : Registers,
    bus    : Bus<'a>,
    halted : bool
}

impl<'a> CPU<'a>
{
    pub fn new(cart : &'a mut Cart, mmu : &'a mut MMU) -> Self
    {
        CPU
        {
            pcntr  : PCntr::new(),
            opcode : 0x00,
            regs   : Registers::new(),
            bus    : Bus::new(cart, mmu),
            halted : false
        }
    }

    pub fn start(&mut self)
    {
        self.regs.write8(Reg8::A, 1);
    }

    pub fn step(&mut self)
    {
        if self.halted == false
        {
            self.fetch_instruction();
            self.execute();
        }
    }

    fn fetch_instruction(&mut self)
    {
        self.opcode = self.bus.read8(self.pcntr.cntr());
        self.pcntr.inc(1);
    }

    fn execute(&mut self)
    {
        match self.opcode
        {
            0x00 => misc::nop(),
            0x01 => ld::imm16_to_reg16(&self.bus, &mut self.pcntr, &mut self.regs, Reg16::BC),
            0x02 => ld::reg8_to_mem8(&mut self.bus, &self.regs, Reg16::BC, Reg8::A),
            0x03 => alu::inc16(&mut self.regs, Reg16::BC),
            0x04 => alu::inc(&mut self.regs, Reg8::B),
            0x05 => alu::dec(&mut self.regs, Reg8::B),
            0x06 => ld::imm8_to_reg8(&self.bus, &mut self.pcntr, &mut self.regs, Reg8::B),
            0x07 => shift::rlc(&mut self.regs, Reg8::A),
            0x08 => ld::reg16_to_imm16(&mut self.bus, &mut self.pcntr, &self.regs, Reg16::SP),
            0x09 => alu::add16(&mut self.regs, Reg16::BC, Reg16::HL),
            0x0A => ld::mem8_to_reg8(&self.bus, &mut self.regs, Reg16::BC, Reg8::A),
            0x0B => alu::dec16(&mut self.regs, Reg16::BC),
            0x0C => alu::inc(&mut self.regs, Reg8::C),
            0x0D => alu::dec(&mut self.regs, Reg8::C),
            0x0E => ld::imm8_to_reg8(&self.bus, &mut self.pcntr, &mut self.regs, Reg8::C),
            0x0F => shift::rrc(&mut self.regs, Reg8::A),
            //-------
            0x10 => misc::stop(),
            0x11 => ld::imm16_to_reg16(&self.bus, &mut self.pcntr, &mut self.regs, Reg16::DE),
            0x12 => ld::reg8_to_mem8(&mut self.bus, &self.regs, Reg16::DE, Reg8::A),
            0x13 => alu::inc16(&mut self.regs, Reg16::DE),
            0x14 => alu::inc(&mut self.regs, Reg8::D),
            0x15 => alu::dec(&mut self.regs, Reg8::D),
            0x16 => ld::imm8_to_reg8(&self.bus, &mut self.pcntr, &mut self.regs, Reg8::D),
            0x17 => shift::rl(&mut self.regs, Reg8::A),
            0x18 => jump::jr8(&self.bus, &mut self.pcntr),
            0x19 => alu::add16(&mut self.regs, Reg16::DE, Reg16::HL),
            0x1A => ld::mem8_to_reg8(&self.bus, &mut self.regs, Reg16::DE, Reg8::A),
            0x1B => alu::dec16(&mut self.regs, Reg16::DE),
            0x1C => alu::inc(&mut self.regs, Reg8::E),
            0x1D => alu::dec(&mut self.regs, Reg8::E),
            0x1E => ld::imm8_to_reg8(&self.bus, &mut self.pcntr, &mut self.regs, Reg8::E),
            0x1F => shift::rr(&mut self.regs, Reg8::A),
            //-------
            0x20 => jump::jr8_cond(&self.bus, &mut self.pcntr, &self.regs, RegF::Z, false),
            0x21 => ld::imm16_to_reg16(&self.bus, &mut self.pcntr, &mut self.regs, Reg16::HL),
            0x22 => ld::reg8_to_mem8_inc(&mut self.bus, &mut self.regs, Reg16::HL, Reg8::A),
            0x23 => alu::inc16(&mut self.regs, Reg16::HL),
            0x24 => alu::inc(&mut self.regs, Reg8::H),
            0x25 => alu::dec(&mut self.regs, Reg8::H),
            0x26 => ld::imm8_to_reg8(&self.bus, &mut self.pcntr, &mut self.regs, Reg8::H),
            0x27 => alu::da(&mut self.regs, Reg8::A),
            0x28 => jump::jr8_cond(&self.bus, &mut self.pcntr, &self.regs, RegF::Z, true),
            0x29 => alu::add16(&mut self.regs, Reg16::HL, Reg16::HL),
            0x2A => ld::mem8_to_reg8_inc(&self.bus, &mut self.regs, Reg16::HL, Reg8::A),
            0x2B => alu::dec16(&mut self.regs, Reg16::HL),
            0x2C => alu::inc(&mut self.regs, Reg8::L),
            0x2D => alu::dec(&mut self.regs, Reg8::L),
            0x2E => ld::imm8_to_reg8(&self.bus, &mut self.pcntr, &mut self.regs, Reg8::L),
            0x2F => alu::cpl(&mut self.regs, Reg8::A),
            //-------
            0x30 => jump::jr8_cond(&self.bus, &mut self.pcntr, &self.regs, RegF::C, false),
            0x31 => ld::imm16_to_reg16(&self.bus, &mut self.pcntr, &mut self.regs, Reg16::SP),
            0x32 => ld::reg8_to_mem8_dec(&mut self.bus, &mut self.regs, Reg16::HL, Reg8::A),
            0x33 => alu::inc16(&mut self.regs, Reg16::SP),
            0x34 => alu::inc16(&mut self.regs, Reg16::HL),
            0x35 => alu::dec16(&mut self.regs, Reg16::HL),
            0x36 => ld::imm8_to_reg16(&self.bus, &mut self.pcntr, &mut self.regs, Reg16::HL),
            0x37 => alu::scf(&mut self.regs),
            0x38 => jump::jr8_cond(&self.bus, &mut self.pcntr, &self.regs, RegF::C, true),
            0x39 => alu::add16(&mut self.regs, Reg16::SP, Reg16::HL),
            0x3A => ld::mem8_to_reg8_dec(&self.bus, &mut self.regs, Reg16::HL, Reg8::A),
            0x3B => alu::dec16(&mut self.regs, Reg16::SP),
            0x3C => alu::inc(&mut self.regs, Reg8::A),
            0x3D => alu::dec(&mut self.regs, Reg8::A),
            0x3E => ld::imm8_to_reg8(&self.bus, &mut self.pcntr, &mut self.regs, Reg8::A),
            0x3F => alu::ccf(&mut self.regs),
            //-------
            0x40 => misc::nop(),
            0x41 => ld::reg8_to_reg8(&mut self.regs, Reg8::C, Reg8::B),
            0x42 => ld::reg8_to_reg8(&mut self.regs, Reg8::D, Reg8::B),
            0x43 => ld::reg8_to_reg8(&mut self.regs, Reg8::E, Reg8::B),
            0x44 => ld::reg8_to_reg8(&mut self.regs, Reg8::H, Reg8::B),
            0x45 => ld::reg8_to_reg8(&mut self.regs, Reg8::L, Reg8::B),
            0x46 => ld::reg16_to_reg8(&mut self.regs, Reg16::HL, Reg8::B),
            0x47 => ld::reg8_to_reg8(&mut self.regs, Reg8::A, Reg8::B),
            0x48 => ld::reg8_to_reg8(&mut self.regs, Reg8::B, Reg8::C),
            0x49 => misc::nop(),
            0x4A => ld::reg8_to_reg8(&mut self.regs, Reg8::D, Reg8::C),
            0x4B => ld::reg8_to_reg8(&mut self.regs, Reg8::E, Reg8::C),
            0x4C => ld::reg8_to_reg8(&mut self.regs, Reg8::H, Reg8::C),
            0x4D => ld::reg8_to_reg8(&mut self.regs, Reg8::L, Reg8::C),
            0x4E => ld::reg16_to_reg8(&mut self.regs, Reg16::HL, Reg8::C),
            0x4F => ld::reg8_to_reg8(&mut self.regs, Reg8::A, Reg8::C),
            //-------
            0x50 => ld::reg8_to_reg8(&mut self.regs, Reg8::B, Reg8::D),
            0x51 => ld::reg8_to_reg8(&mut self.regs, Reg8::C, Reg8::D),
            0x52 => misc::nop(),
            0x53 => ld::reg8_to_reg8(&mut self.regs, Reg8::E, Reg8::D),
            0x54 => ld::reg8_to_reg8(&mut self.regs, Reg8::H, Reg8::D),
            0x55 => ld::reg8_to_reg8(&mut self.regs, Reg8::L, Reg8::D),
            0x56 => ld::reg16_to_reg8(&mut self.regs, Reg16::HL, Reg8::D),
            0x57 => ld::reg8_to_reg8(&mut self.regs, Reg8::A, Reg8::D),
            0x58 => ld::reg8_to_reg8(&mut self.regs, Reg8::B, Reg8::E),
            0x59 => ld::reg8_to_reg8(&mut self.regs, Reg8::C, Reg8::E),
            0x5A => ld::reg8_to_reg8(&mut self.regs, Reg8::D, Reg8::E),
            0x5B => misc::nop(),
            0x5C => ld::reg8_to_reg8(&mut self.regs, Reg8::H, Reg8::E),
            0x5D => ld::reg8_to_reg8(&mut self.regs, Reg8::L, Reg8::E),
            0x5E => ld::reg16_to_reg8(&mut self.regs, Reg16::HL, Reg8::E),
            0x5F => ld::reg8_to_reg8(&mut self.regs, Reg8::A, Reg8::E),
            //-------
            0x60 => ld::reg8_to_reg8(&mut self.regs, Reg8::B, Reg8::H),
            0x61 => ld::reg8_to_reg8(&mut self.regs, Reg8::C, Reg8::H),
            0x62 => ld::reg8_to_reg8(&mut self.regs, Reg8::D, Reg8::H),
            0x63 => ld::reg8_to_reg8(&mut self.regs, Reg8::E, Reg8::H),
            0x64 => misc::nop(),
            0x65 => ld::reg8_to_reg8(&mut self.regs, Reg8::L, Reg8::H),
            0x66 => ld::reg16_to_reg8(&mut self.regs, Reg16::HL, Reg8::H),
            0x67 => ld::reg8_to_reg8(&mut self.regs, Reg8::A, Reg8::H),
            0x68 => ld::reg8_to_reg8(&mut self.regs, Reg8::B, Reg8::L),
            0x69 => ld::reg8_to_reg8(&mut self.regs, Reg8::C, Reg8::L),
            0x6A => ld::reg8_to_reg8(&mut self.regs, Reg8::D, Reg8::L),
            0x6B => ld::reg8_to_reg8(&mut self.regs, Reg8::E, Reg8::L),
            0x6C => ld::reg8_to_reg8(&mut self.regs, Reg8::H, Reg8::L),
            0x6D => misc::nop(),
            0x6E => ld::reg16_to_reg8(&mut self.regs, Reg16::HL, Reg8::L),
            0x6F => ld::reg8_to_reg8(&mut self.regs, Reg8::A, Reg8::L),
            //-------
            0x70 => ld::reg8_to_reg16(&mut self.regs, Reg8::B, Reg16::HL),
            0x71 => ld::reg8_to_reg16(&mut self.regs, Reg8::C, Reg16::HL),
            0x72 => ld::reg8_to_reg16(&mut self.regs, Reg8::D, Reg16::HL),
            0x73 => ld::reg8_to_reg16(&mut self.regs, Reg8::E, Reg16::HL),
            0x74 => ld::reg8_to_reg16(&mut self.regs, Reg8::H, Reg16::HL),
            0x75 => ld::reg8_to_reg16(&mut self.regs, Reg8::L, Reg16::HL),
            0x76 => misc::halt(&mut self.halted),
            0x77 => ld::reg8_to_reg16(&mut self.regs, Reg8::A, Reg16::HL),
            0x78 => ld::reg8_to_reg8(&mut self.regs, Reg8::B, Reg8::A),
            0x79 => ld::reg8_to_reg8(&mut self.regs, Reg8::C, Reg8::A),
            0x7A => ld::reg8_to_reg8(&mut self.regs, Reg8::D, Reg8::A),
            0x7B => ld::reg8_to_reg8(&mut self.regs, Reg8::E, Reg8::A),
            0x7C => ld::reg8_to_reg8(&mut self.regs, Reg8::H, Reg8::A),
            0x7D => ld::reg8_to_reg8(&mut self.regs, Reg8::L, Reg8::A),
            0x7E => ld::reg16_to_reg8(&mut self.regs, Reg16::HL, Reg8::A),
            0x7F => misc::nop(),
            //-------
            0x80 => alu::add8(&mut self.regs, Reg8::B, Reg8::A),
            0x81 => alu::add8(&mut self.regs, Reg8::C, Reg8::A),
            0x82 => alu::add8(&mut self.regs, Reg8::D, Reg8::A),
            0x83 => alu::add8(&mut self.regs, Reg8::E, Reg8::A),
            0x84 => alu::add8(&mut self.regs, Reg8::H, Reg8::A),
            0x85 => alu::add8(&mut self.regs, Reg8::L, Reg8::A),
            0x86 => alu::add16_to_8(&mut self.regs, Reg16::HL, Reg8::A),
            0x87 => alu::add8(&mut self.regs, Reg8::A, Reg8::A),
            0x88 => alu::adc8(&mut self.regs, Reg8::B, Reg8::A),
            0x89 => alu::adc8(&mut self.regs, Reg8::C, Reg8::A),
            0x8A => alu::adc8(&mut self.regs, Reg8::D, Reg8::A),
            0x8B => alu::adc8(&mut self.regs, Reg8::E, Reg8::A),
            0x8C => alu::adc8(&mut self.regs, Reg8::H, Reg8::A),
            0x8D => alu::adc8(&mut self.regs, Reg8::L, Reg8::A),
            0x8E => alu::adc16_to_8(&mut self.regs, Reg16::HL, Reg8::A),
            0x8F => alu::adc8(&mut self.regs, Reg8::A, Reg8::A),
            //-------
            0x90 => alu::sub8(&mut self.regs, Reg8::B, Reg8::A),
            0x91 => alu::sub8(&mut self.regs, Reg8::C, Reg8::A),
            0x92 => alu::sub8(&mut self.regs, Reg8::D, Reg8::A),
            0x93 => alu::sub8(&mut self.regs, Reg8::E, Reg8::A),
            0x94 => alu::sub8(&mut self.regs, Reg8::H, Reg8::A),
            0x95 => alu::sub8(&mut self.regs, Reg8::L, Reg8::A),
            0x96 => alu::sub16_to_8(&mut self.regs, Reg16::HL, Reg8::A),
            0x97 => alu::sub8(&mut self.regs, Reg8::A, Reg8::A),
            0x98 => alu::sbc8(&mut self.regs, Reg8::B, Reg8::A),
            0x99 => alu::sbc8(&mut self.regs, Reg8::C, Reg8::A),
            0x9A => alu::sbc8(&mut self.regs, Reg8::D, Reg8::A),
            0x9B => alu::sbc8(&mut self.regs, Reg8::E, Reg8::A),
            0x9C => alu::sbc8(&mut self.regs, Reg8::H, Reg8::A),
            0x9D => alu::sbc8(&mut self.regs, Reg8::L, Reg8::A),
            0x9D => alu::sbc16_to_8(&mut self.regs, Reg16::HL, Reg8::A),
            0x9F => alu::sbc8(&mut self.regs, Reg8::A, Reg8::A),
            //-------
            0xA0 => alu::and8(&mut self.regs, Reg8::B, Reg8::A),
            0xA1 => alu::and8(&mut self.regs, Reg8::C, Reg8::A),
            0xA2 => alu::and8(&mut self.regs, Reg8::D, Reg8::A),
            0xA3 => alu::and8(&mut self.regs, Reg8::E, Reg8::A),
            0xA4 => alu::and8(&mut self.regs, Reg8::H, Reg8::A),
            0xA5 => alu::and8(&mut self.regs, Reg8::L, Reg8::A),
            0xA6 => alu::and16_to_8(&mut self.regs, Reg16::HL, Reg8::A),
            0xA7 => alu::and8(&mut self.regs, Reg8::A, Reg8::A),
            0xA8 => alu::xor8(&mut self.regs, Reg8::B, Reg8::A),
            0xA9 => alu::xor8(&mut self.regs, Reg8::C, Reg8::A),
            0xAA => alu::xor8(&mut self.regs, Reg8::D, Reg8::A),
            0xAB => alu::xor8(&mut self.regs, Reg8::E, Reg8::A),
            0xAC => alu::xor8(&mut self.regs, Reg8::H, Reg8::A),
            0xAD => alu::xor8(&mut self.regs, Reg8::L, Reg8::A),
            0xAE => alu::xor16_to_8(&mut self.regs, Reg16::HL, Reg8::A),
            0xAF => alu::xor8(&mut self.regs, Reg8::A, Reg8::A),
            //-------
            0xB0 => alu::or8(&mut self.regs, Reg8::B, Reg8::A),
            0xB1 => alu::or8(&mut self.regs, Reg8::C, Reg8::A),
            0xB2 => alu::or8(&mut self.regs, Reg8::D, Reg8::A),
            0xB3 => alu::or8(&mut self.regs, Reg8::E, Reg8::A),
            0xB4 => alu::or8(&mut self.regs, Reg8::H, Reg8::A),
            0xB5 => alu::or8(&mut self.regs, Reg8::L, Reg8::A),
            0xB6 => alu::or16_to_8(&mut self.regs, Reg16::HL, Reg8::A),
            0xB7 => alu::or8(&mut self.regs, Reg8::A, Reg8::A),
            0xB8 => alu::cp8(&mut self.regs, Reg8::B, Reg8::A),
            0xB9 => alu::cp8(&mut self.regs, Reg8::C, Reg8::A),
            0xBA => alu::cp8(&mut self.regs, Reg8::D, Reg8::A),
            0xBB => alu::cp8(&mut self.regs, Reg8::E, Reg8::A),
            0xBC => alu::cp8(&mut self.regs, Reg8::H, Reg8::A),
            0xBD => alu::cp8(&mut self.regs, Reg8::L, Reg8::A),
            0xBE => alu::cp16_to_8(&mut self.regs, Reg16::HL, Reg8::A),
            0xBF => alu::cp8(&mut self.regs, Reg8::A, Reg8::A),
            //-------
            //0xC0
            0xC1 => ld::pop16(&mut self.regs, Reg16::BC),
            0xC2 => jump::jr16_cond(&self.bus, &mut self.pcntr, &self.regs, RegF::Z, false),
            0xC3 => jump::jr16(&self.bus, &mut self.pcntr),
            //0xC4
            0xC5 => ld::push16(&mut self.regs, Reg16::BC),
            0xC6 => alu::add8_mem(&self.bus, &mut self.pcntr, &mut self.regs, Reg8::A),
            //0xC7
            //0xC8
            //0xC9
            0xCA => jump::jr16_cond(&self.bus, &mut self.pcntr, &self.regs, RegF::Z, true),
            //0xCB
            //0xCC
            //0xCD
            0xCE => alu::add8_mem(&self.bus, &mut self.pcntr, &mut self.regs, Reg8::A),
            //0xCF
            //-------
            //0xD0
            0xD1 => ld::pop16(&mut self.regs, Reg16::DE),
            0xD2 => jump::jr16_cond(&self.bus, &mut self.pcntr, &self.regs, RegF::C, false),
            //0xD4
            0xD5 => ld::push16(&mut self.regs, Reg16::DE),
            0xD6 => alu::sub8_mem(&self.bus, &mut self.pcntr, &mut self.regs, Reg8::A),
            //0xD7
            //0xD8
            //0xD9
            0xDA => jump::jr16_cond(&self.bus, &mut self.pcntr, &self.regs, RegF::C, true),
            //0xDC
            0xDE => alu::sbc8_mem(&self.bus, &mut self.pcntr, &mut self.regs, Reg8::A),
            //0xDF
            _ =>
            {
                println!("Asd");
            }
        }
    }
}