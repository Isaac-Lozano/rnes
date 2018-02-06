extern crate r6502;

mod memory;
mod mapper;
mod ppu;

fn main() {
    println!("Hello, world!");
    /* structure:
     * const CPU_TO_PPU_CYCLES: i64 = 3;
     *
     * let mut cycles_till_next = 0;
     * loop
     * {
     *     cycles_till_next += ppu.run() as i64;
     *     cpu_cycles = if (cycles_till_next / CPU_TO_PPU_CYCLES) > 0
     *     {
     *         cycles_till_next -= cpu.run(cycles_till_next / CPU_TO_PPU_CYCLES) as i64 * CPU_TO_PPU_CYCLES;;
     *     }
     *
     *     if ppu.vblank()
     *     {
     *         cpu.nmi();
     *     }
     *     if ppu.redraw()
     *     {
     *         sdl.redraw();
     *     }
     * }
     */
}
