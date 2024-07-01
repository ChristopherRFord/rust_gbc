pub mod cpu;
pub mod memory;

use cpu::cpu::CPU;

fn main()
{
    let mut cpu = CPU::new();

    cpu.memory.write_byte(0x00, 0x00);

    cpu.run();
}
