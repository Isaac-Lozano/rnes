const PPU_MEM_SIZE: usize = 0x4000;
const PPU_OAM_SIZE: usize = 0x100;

#[derive(Debug,Clone,Copy)]
enum PpuState
{
    Startup,
    Drawing{x: usize, y: usize},
    VBlank,
}

#[derive(Debug,Clone,Copy)]
struct PpuCtrl(val: u8);
/* nmi_enable: bool,
 * master: bool,
 * sprite_height: u16,
 * background_pt: u16,
 * sprite_pt: u16,
 * increment: u16,
 * base_nametable: usize,
 */

#[derive(Debug,Clone,Copy)]
struct PpuMask(val: u8)
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
struct PpuStatus(val: u8)
/* vblank_started: bool,
 * sprite0_hit: bool,
 * sprite_overflow: bool,
 * Blah
 */

#[derive(Debug)]
pub struct Ppu
{
    state: PpuState,

    ppuctrl: PpuCtrl,
    ppumask: PpuMask,
    ppustatus: PpuStatus,
    oamaddr: u8,
    oamdata: u8,

    ppuscroll_latch: bool,
    ppuscroll_x: u8,
    ppuscroll_y: u8,

    ppuaddr_latch: bool,
    ppuaddr: u16,

    ppudata: u8,
    oamdma: u8,

    /* PPU RAM */
    mem: [u8; PPU_MEM_SIZE],
    oam: [u8; PPU_OAM_SIZE],
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
        }
    }

    pub fn set_ppuctrl(&mut self, val: u8)
    {
        self.ppuctrl = val;
    }

    pub fn set_ppumask(&mut self, val: u8)
    {
        self.ppumask = val;
    }

    pub fn set_ppustatus(&mut self, val: u8)
    {
        self.ppustatus = val;
    }

    pub fn set_oamaddr(&mut self, val: u8)
    {
        self.oamaddr = val;
    }

    pub fn set_oamdata(&mut self, val: u8)
    {
        self.oamdata = val;
    }

    pub fn set_ppuscroll(&mut self, val: u8)
    {
        self.ppuscroll = val;
    }

    pub fn set_ppusaddr(&mut self, val: u8)
    {
        self.ppusaddr = val;
    }

    pub fn set_ppudata(&mut self, val: u8)
    {
        self.ppudata = val;
    }

    pub fn set_oamdata(&mut self, val: u8)
    {
        self.oamdata = val;
    }
}
