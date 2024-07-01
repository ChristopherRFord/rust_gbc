pub mod cpu;
pub use cpu::CPU;
pub use cpu::Instruction;


pub mod registers;
pub use registers::Registers;
pub use registers::FlagRegister;
pub use registers::ByteRegisterLabel;

pub mod memory;
pub use memory::Memory;

fn main()
{
    let mut cpu = CPU::new();


    let instruction = Instruction::ADD(ByteRegisterLabel::C);

    cpu.execute(instruction);

}
