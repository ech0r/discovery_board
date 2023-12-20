#![no_main]
#![no_std]

use discovery_board as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    discovery_board::exit()
}
