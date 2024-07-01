pub mod cpu;
pub mod memory;

use cpu::cpu::CPU;
use cpu::cpu_registers::WordRegisterTarget;

fn main()
{
    let mut cpu = CPU::new();


    cpu.memory.write_byte(0x0008, 0x20);
    cpu.registers.set_word(WordRegisterTarget::HL, 0x0008);

    cpu.memory.write_byte(0x0000, 0x34);
    cpu.memory.write_byte(0x0001, 0x86);
    cpu.memory.write_byte(0x0002, 0x00);

    cpu.run();
}
