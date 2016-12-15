pub trait Mapper
{
    fn read_expansion_rom_without_mm(&mut self, addr: u16) -> u8;
    fn read_expansion_ram_without_mm(&mut self, addr: u16) -> u8;
    fn read_low_rom_bank_without_mm(&mut self, addr: u16) -> u8;
    fn read_high_rom_bank_without_mm(&mut self, addr: u16) -> u8;

    fn read_expansion_rom(&mut self, addr: u16) -> u8
    {
        self.read_expansion_rom_without_mm(addr)
    }

    fn read_expansion_ram(&mut self, addr: u16) -> u8
    {
        self.read_expansion_ram_without_mm(addr)
    }

    fn read_low_rom_bank(&mut self, addr: u16) -> u8
    {
        self.read_low_rom_bank_without_mm(addr)
    }

    fn read_high_rom_bank(&mut self, addr: u16) -> u8
    {
        self.read_high_rom_bank_without_mm(addr)
    }

    fn write_expansion_rom_without_mm(&mut self, addr: u16, val: u8) -> u8;
    fn write_expansion_ram_without_mm(&mut self, addr: u16, val: u8) -> u8;
    fn write_low_rom_bank_without_mm(&mut self, addr: u16, val: u8) -> u8;
    fn write_high_rom_bank_without_mm(&mut self, addr: u16, val: u8) -> u8;

    fn write_expansion_rom(&mut self, addr: u16, val: u8)
    {
        self.write_expansion_rom_without_mm(addr, val);
    }

    fn write_expansion_ram(&mut self, addr: u16, val: u8)
    {
        self.write_expansion_ram_without_mm(addr, val);
    }

    fn write_low_rom_bank(&mut self, addr: u16, val: u8)
    {
        self.write_low_rom_bank_without_mm(addr, val);
    }

    fn write_high_rom_bank(&mut self, addr: u16, val: u8)
    {
        self.write_high_rom_bank_without_mm(addr, val);
    }
}
