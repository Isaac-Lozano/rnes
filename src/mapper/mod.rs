use r6502::memory::Memory;

pub trait Mapper: Memory
{
    fn vram_read_without_mm(&mut self, addr: u16) -> u8;
    fn vram_write_without_mm(&mut self, addr: u16, val: u8);

    fn vram_read(&mut self, addr: u16) -> u8
    {
        self.vram_read_without_mm(addr)
    }
    fn vram_write(&mut self, addr: u16, val: u8)
    {
        self.vram_write_without_mm(addr, val);
    }
}
