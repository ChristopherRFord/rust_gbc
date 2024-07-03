use rust_gbc::memory::Memory;

#[test]
fn test_memory_new()
{
    let mem = Memory::new();

    assert_eq!(mem.memory.len(), 0xFFFF);
    assert!(mem.memory.iter().all(|&byte| byte == 0));
}

#[test]
fn test_read_write_byte()
{
    let mut mem = Memory::new();
    let address = 0x1234;
    let value   = 0xAB;

    mem.write_byte(address, value);
    let read_value = mem.read_byte(address);

    assert_eq!(read_value, value);
}

#[test]
fn test_read_write_word()
{
    let mut mem = Memory::new();
    let address = 0x1234;
    let value   = 0xABCD;

    mem.write_word(address, value);
    let read_value = mem.read_word(address);

    assert_eq!(read_value, value);
}
