use memory::NesMemory;
use ppu::Ppu;

use r6502::Cpu6502;

pub struct Nes
{
    cpu: Cpu6502<NesMemory>,
    ppu: Ppu,
}
