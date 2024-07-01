pub mod cpu_instructions;
pub use cpu_instructions::Instruction;

pub mod cpu;
pub use cpu::CPU;

pub mod registers;
pub use registers::Registers;
pub use registers::FlagRegister;
pub use registers::ByteRegisterTarget;

pub mod memory;
pub use memory::Memory;

fn main()
{
    let mut cpu = CPU::new();

    cpu.memory.write_byte(0x06, 0x00);

    cpu.run();
}
