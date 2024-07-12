pub mod bus
{
    use crate::cart::Cart;
    use crate::mem::Mem;
    use crate::regs::Regs;
    use crate::cpu_enums::RamType;
    use crate::cpu_enums::Reg;

// 0x0000 - 0x3FFF : ROM Bank 0
// 0x4000 - 0x7FFF : ROM Bank 1 - Switchable
// 0x8000 - 0x97FF : CHR RAM
// 0x9800 - 0x9BFF : BG Map 1
// 0x9C00 - 0x9FFF : BG Map 2
// 0xA000 - 0xBFFF : Cartridge RAM
// 0xC000 - 0xCFFF : RAM Bank 0
// 0xD000 - 0xDFFF : RAM Bank 1-7 - switchable - Color only
// 0xE000 - 0xFDFF : Reserved - Echo RAM
// 0xFE00 - 0xFE9F : Object Attribute Memory
// 0xFEA0 - 0xFEFF : Reserved - Unusable
// 0xFF00 - 0xFF7F : I/O Registers
// 0xFF80 - 0xFFFE : Zero Page

    pub fn read8(cart    : &Cart, 
                 mem     : &Mem,
                 regs    : &Regs,
                 address : u16) -> u8
    {
        // ROM
        if address < 0x8000
        {
            cart.read8(address)
        }
        // Unsupported
        else if address < 0xA000
        {
            // todo
            panic!("Unsupported BUS Read {:04X}", address)
        }
        // CART
        else if address < 0xC000
        {
            cart.read8(address)
        }
        else if address < 0xE000
        {
            mem.read8(&RamType::WRAM, address)
        }
        else if address < 0xFE00
        {
            0
        }
        else if address < 0xFEA0
        {
            //todo
            panic!("Unsupported BUS Read {:04X}", address)
        }
        else if address < 0xFF00
        {
            0
        }
        else if address  < 0xFF80
        {
            //todo
            panic!("Unsupported BUS Read {:04X}", address)
        }
        else if address == 0xFFFF
        {
            regs.read_ie()
        }
        else
        {
            //hram todo
            0
        }
    }
    pub fn read16(cart    : &Cart, 
                  mem     : &Mem,
                  regs    : &Regs,
                  address : u16) -> u16
    {
        let low_byte  = read8(cart, mem, regs, address) as u16;
        let high_byte = read8(cart, mem, regs, address.wrapping_add(1)) as u16;
        (high_byte << 8) | low_byte
    }

    pub fn write8(cart    : &mut Cart, 
                  mem     : &mut Mem,
                  regs    : &mut Regs,
                  address : u16,
                  value   : u8)
    {
        if address < 0x8000
        {
            cart.write8(address, value);
        }
        else if address < 0xA000
        {
            // todo
            println!("Unsupported BUS Write {:04X}", address);
        }
        else if address < 0xC000
        {
            cart.write8(address, value);
        }
        else if address < 0xE000
        {
            mem.write8(&RamType::WRAM, address, value);
        }
        else if address < 0xFE00
        {

        }
        else if address < 0xFEA0
        {
            // todo
            println!("Unsupported BUS Write {:04X}", address);
        }
        else if address < 0xFF00
        {

        }
        else if address < 0xFF80
        {
            // todo
            println!("Unsupported BUS Write {:04X}", address);
        }
        else if address == 0xFFFF
        {
            regs.write_ie(value);
        }
        else
        {
            mem.write8(&RamType::HRAM, address, value);
        }
    }

    pub fn push8(cart    : &mut Cart,
                 mem     : &mut Mem,
                 regs    : &mut Regs,
                 value   : u8)
    {
        let sp = regs.read(Reg::SP).wrapping_sub(0x1);
        write8(cart, mem, regs, sp, value);
        regs.write(Reg::SP, sp);
    }
    pub fn push16(cart  : &mut Cart,
                  mem   : &mut Mem,
                  regs  : &mut Regs,
                  value : u16)
    {
        push8(cart, mem, regs, ((value >> 8) & 0xFF) as u8);
        push8(cart, mem, regs, (value & 0xFF) as u8);
    }

    pub fn pop8(cart    : &mut Cart,
                 mem     : &mut Mem,
                 regs    : &mut Regs) -> u8
    {
        let sp = regs.read(Reg::SP).wrapping_add(0x1);
        regs.write(Reg::SP, sp);
        read8(cart, mem, regs, sp)
    }

    pub fn pop16(cart    : &mut Cart,
                 mem     : &mut Mem,
                 regs    : &mut Regs) -> u16
    {
        let lo = pop8(cart, mem, regs) as u16;
        let hi = pop8(cart, mem, regs) as u16;

        (hi << 0x8) | lo
    }
}