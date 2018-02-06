use r6502::memory::Memory;
use mapper::Mapper;
use ppu::Ppu;

const RAM_SIZE: usize = 0x0800;
const LOW_IO_SIZE: usize = 0x0008;
const SRAM_SIZE: usize = 0x2000;

pub struct NesMemory
{
    ram: [u8; RAM_SIZE],
    ppu: Ppu,
    mapper: Box::<Mapper>
}

impl NesMemory
{
    fn new(mapper: M) -> NesMemory
        where M: Mapper
    {
        NesMemory
        {
            ram: [0; RAM_SIZE],
            mapper: mapper,
        }
    }
}

impl Memory<u8> for NesMemory
{
    fn read_without_mm(&mut self, addr: u16) -> u8
    {
        match addr
        {
            0x0000...0x1fff =>
            {
                self.ram[addr as usize % RAM_SIZE]
            },
            0x2000...0x3fff =>
            {
                self.ppu.read_without_mm(addr)
            },
            0x4000...0x401f =>
            {
                /* TODO: high IO regs */
                0
            },
            0x4020...0x5fff =>
            {
                self.mapper.read_expansion_rom_without_mm(addr)
            },
            0x6000...0x7fff =>
            {
                self.mapper.read_expansion_ram_without_mm(addr)
            },
            0x8000...0xbfff =>
            {
                self.mapper.read_low_rom_bank_without_mm(addr)
            },
            0xc000...0xffff =>
            {
                self.mapper.read_high_rom_bank_without_mm(addr)
            },
            _ => unreachable!(),
        }
    }

    fn write_without_mm(&mut self, addr: u16, val: u8)
    {
        match addr
        {
            0x0000...0x1fff =>
            {
                self.ram[addr as usize % RAM_SIZE] = val;
            },
            0x2000...0x3fff =>
            {
                self.ppu.write_without_mm(addr, val);
            },
            0x4000...0x401f =>
            {
                /* TODO: high IO regs */
            },
            0x4020...0x5fff =>
            {
                self.mapper.write_expansion_rom_without_mm(addr, val);
            },
            0x6000...0x7fff =>
            {
                self.mapper.write_expansion_ram_without_mm(addr, val);
            },
            0x8000...0xbfff =>
            {
                self.mapper.write_low_rom_bank_without_mm(addr, val);
            },
            0xc000...0xffff =>
            {
                self.mapper.write_high_rom_bank_without_mm(addr, val);
            },
            _ => unreachable!(),
        }
    }

    fn read(&mut self, addr: u16) -> u8
    {
        match addr
        {
            0x0000...0x1fff =>
            {
                self.ram[addr as usize % RAM_SIZE]
            },
            0x2000...0x3fff =>
            {
                self.ppu.read(addr)
            },
            0x4000...0x401f =>
            {
                /* TODO: high IO regs */
                0
            },
            0x4020...0x5fff =>
            {
                self.mapper.read_expansion_rom(addr)
            },
            0x6000...0x7fff =>
            {
                self.mapper.read_expansion_ram(addr)
            },
            0x8000...0xbfff =>
            {
                self.mapper.read_low_rom_bank(addr)
            },
            0xc000...0xffff =>
            {
                self.mapper.read_high_rom_bank(addr)
            },
            _ => unreachable!(),
        }
    }

    fn write(&mut self, addr: u16, val: u8)
    {
        match addr
        {
            0x0000...0x1fff =>
            {
                self.ram[addr as usize % RAM_SIZE] = val;
            },
            0x2000...0x3fff =>
            {
                self.ppu.write(addr, val);
            },
            0x4000...0x401f =>
            {
                /* TODO: high IO regs */
            },
            0x4020...0x5fff =>
            {
                self.mapper.write_expansion_rom(addr, val);
            },
            0x6000...0x7fff =>
            {
                self.mapper.write_expansion_ram(addr, val);
            },
            0x8000...0xbfff =>
            {
                self.mapper.write_low_rom_bank(addr, val);
            },
            0xc000...0xffff =>
            {
                self.mapper.write_high_rom_bank(addr, val);
            },
            _ => unreachable!(),
        }
    }
}
