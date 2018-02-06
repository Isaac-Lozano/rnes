use r6502::memory::Memory;
use mapper::Mapper;

const PPU_MEM_SIZE: usize = 0x4000;
const PPU_OAM_SIZE: usize = 0x100;
const PPU_NUM_REGS: usize = 8;

const PPUCTRL: u16 = 0x2000;
const PPUMASK: u16 = 0x2001;
const PPUSTATUS: u16 = 0x2002;
const OAMADDR: u16 = 0x2003;
const OAMDATA: u16 = 0x2004;
const PPUSCROLL: u16 = 0x2005;
const PPUADDR: u16 = 0x2006;
const PPUDATA: u16 = 0x2007;

const PPU_WIDTH: usize = 255;
const PPU_HEIGHT: usize = 240;

const PPU_NAMETABLE_SIZE: usize = 0x800;
const PPU_PALETTE_SIZE: usize = 0x20;

pub struct Vram
{
    mapper: Rc<RefCell<Box<Mapper>>>,
    nametable: [u8; PPU_NAMETABLE_SIZE],
    palette: [u8; PPU_PALETTE_SIZE],
}

impl Memory<u8> for Vram
{
    fn read_without_mm(&mut self, addr: u16)
    {
        match addr % 0x4000
        {
            0 ... 0x2000 =>
            {
            }
        }
    }
}

#[derive(Debug,Clone,Copy)]
enum PpuState
{
    Startup,
    Drawing{scanline: usize},
    VBlank,
}

#[derive(Debug,Clone,Copy)]
pub struct PpuCtrl(u8);
/* nmi_enable: bool,
 * master: bool,
 * sprite_height: u16,
 * background_pt: u16,
 * sprite_pt: u16,
 * increment: u16,
 * base_nametable: usize,
 */

#[derive(Debug,Clone,Copy)]
pub struct PpuMask(u8);
/* emphasize_red: bool,
 * emphasize_blue: bool,
 * emphasize_green: bool,
 * show_sprites: bool,
 * show_background: bool,
 * show_sprites_left: bool,
 * show_background_left: bool,
 * greyscale: bool,
 */

#[derive(Debug,Clone,Copy)]
pub struct PpuStatus(u8);
/* vblank_started: bool,
 * sprite0_hit: bool,
 * sprite_overflow: bool,
 * Blah
 */

#[derive(Debug)]
pub struct Ppu
{
    pub state: PpuState,

    pub ppuctrl: PpuCtrl,
    pub ppumask: PpuMask,
    pub ppustatus: PpuStatus,
    pub oamaddr: u8,
    pub oamdata: u8,

    pub ppuscroll_latch: bool,
    pub ppuscroll_x: u8,
    pub ppuscroll_y: u8,

    pub ppuaddr_latch: bool,
    pub ppuaddr: u16,

    pub ppudata: u8,
    pub oamdma: u8,

    /* PPU RAM */
    pub mem: [u8; PPU_MEM_SIZE],
    pub oam: [u8; PPU_OAM_SIZE],
    pub screen: [u8; 
}

impl Ppu
{
    pub fn new() -> Ppu
    {
        /* TODO: constants */
        Ppu
        {
            state: PpuState::Startup,

            ppuctrl: PpuCtrl(0),
            ppumask: PpuMask(0),
            ppustatus: PpuStatus(0),
            oamaddr: 0x00,
            oamdata: 0x00,

            ppuscroll_latch: false,
            ppuscroll_x: 0x00,
            ppuscroll_y: 0x00,

            ppuaddr_latch: false,
            ppuaddr: 0x0000,

            ppudata: 0x00,
            oamdma: 0x00,

            mem: [0; PPU_MEM_SIZE],
            oam: [0; PPU_OAM_SIZE],
        }
    }

    pub fn run(&mut self) -> u32
    {
        let new_state = None;
        let cycles_run;

        match self.state
        {
            PpuState::Startup =>
            {
                new_state = PpuState::Drawing{scanline: 0};
                cycles_run = 
            },
        }

        if let Some(state) = new_state
        {
            self.state = state;
        }
    }
}

impl Memory<u8> for Ppu
{
    fn read_without_mm(&mut self, addr: u16) -> u8
    {
        match addr % PPU_NUM_REGS as u16
        {
            PPUCTRL - 0x2000 =>
            {
                self.ppuctrl.0
            },
            PPUMASK - 0x2000 =>
            {
                self.ppumask.0
            },
            PPUSTATUS - 0x2000 =>
            {
                self.ppustatus.0
            },
            OAMADDR - 0x2000 =>
            {
                self.oamaddr
            },
            OAMDATA - 0x2000 =>
            {
                self.oamdata
            },
            PPUSCROLL - 0x2000 =>
            {
                if self.ppuscroll_latch
                {
                    self.ppuscroll_y
                }
                else
                {
                    self.ppuscroll_x
                }
            },
            PPUADDR - 0x2000 =>
            {
                if self.ppuaddr_latch
                {
                    (self.ppuaddr >> 8) as u8
                }
                else
                {
                    self.ppuaddr as u8
                }
            },
            PPUDATA - 0x2000 =>
            {
                self.ppudata
            },
            _ => unreachable!(),
        }
    }

    fn write_without_mm(&mut self, addr: u16, val: u8)
    {
        match addr % PPU_NUM_REGS as u16
        {
            PPUCTRL - 0x2000 =>
            {
                self.ppuctrl.0 = val;
            },
            PPUMASK - 0x2000 =>
            {
                self.ppumask.0 = val;
            },
            PPUSTATUS - 0x2000 =>
            {
                self.ppustatus.0 = val;
            },
            OAMADDR - 0x2000 =>
            {
                self.oamaddr = val;
            },
            OAMDATA - 0x2000 =>
            {
                self.oamdata = val;
            },
            PPUSCROLL - 0x2000 =>
            {
                if self.ppuscroll_latch
                {
                    self.ppuscroll_y = val;
                }
                else
                {
                    self.ppuscroll_x = val;
                }
            },
            PPUADDR - 0x2000 =>
            {
                if self.ppuaddr_latch
                {
                    self.ppuaddr = (self.ppuaddr & 0xFF00) | val as u16;
                }
                else
                {
                    self.ppuaddr = ((val as u16) << 8) | (self.ppuaddr & 0x00FF);
                }
            },
            PPUDATA - 0x2000 =>
            {
                self.ppudata = val;
            },
            _ => unreachable!(),
        }
    }

    fn read(&mut self, addr: u16) -> u8
    {
        match addr % PPU_NUM_REGS as u16
        {
            PPUCTRL - 0x2000 =>
            {
                self.ppuctrl.0
            },
            PPUMASK - 0x2000 =>
            {
                self.ppumask.0
            },
            PPUSTATUS - 0x2000 =>
            {
                self.ppustatus.0
            },
            OAMADDR - 0x2000 =>
            {
                self.oamaddr
            },
            OAMDATA - 0x2000 =>
            {
                self.oamdata
            },
            PPUSCROLL - 0x2000 =>
            {
                let res = if self.ppuscroll_latch
                {
                    self.ppuscroll_y
                }
                else
                {
                    self.ppuscroll_x
                };
                self.ppuscroll_latch = !self.ppuscroll_latch;
                res
            },
            PPUADDR - 0x2000 =>
            {
                let res = if self.ppuaddr_latch
                {
                    (self.ppuaddr >> 8) as u8
                }
                else
                {
                    self.ppuaddr as u8
                };
                self.ppuaddr_latch = !self.ppuaddr_latch;
                res
            },
            PPUDATA - 0x2000 =>
            {
                self.ppudata
            },
            _ => unreachable!(),
        }
    }

    fn write(&mut self, addr: u16, val: u8)
    {
        match addr % PPU_NUM_REGS as u16
        {
            PPUCTRL - 0x2000 =>
            {
                self.ppuctrl.0 = val;
            },
            PPUMASK - 0x2000 =>
            {
                self.ppumask.0 = val;
            },
            PPUSTATUS - 0x2000 =>
            {
                self.ppustatus.0 = val;
            },
            OAMADDR - 0x2000 =>
            {
                self.oamaddr = val;
            },
            OAMDATA - 0x2000 =>
            {
                self.oamdata = val;
            },
            PPUSCROLL - 0x2000 =>
            {
                if self.ppuscroll_latch
                {
                    self.ppuscroll_y = val;
                }
                else
                {
                    self.ppuscroll_x = val;
                }
                self.ppuscroll_latch = !self.ppuscroll_latch;
            },
            PPUADDR - 0x2000 =>
            {
                if self.ppuaddr_latch
                {
                    self.ppuaddr = (self.ppuaddr & 0xFF00) | val as u16;
                }
                else
                {
                    self.ppuaddr = ((val as u16) << 8) | (self.ppuaddr & 0x00FF);
                }
                self.ppuaddr_latch = !self.ppuaddr_latch;
            },
            PPUDATA - 0x2000 =>
            {
                self.ppudata = val;
            },
            _ => unreachable!(),
        }
    }
}
