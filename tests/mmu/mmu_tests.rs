use rust_gbc::mmu::mmu::MMU;

#[test]
fn read_write_byte()
{
    let mut mmu = MMU::new();
        
    mmu.write_byte(0x0000, 0xAB);
    assert_eq!(mmu.read_byte(0x0000), 0xAB);
}

#[test]
fn read_write_word()
{
    let mut mmu = MMU::new();
        
    mmu.write_word(0x0000, 0xABCD);
    assert_eq!(mmu.read_word(0x0000), 0xABCD);
}