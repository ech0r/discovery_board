#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use discovery_board as _; // global logger + panicking-behavior + memory layout

// Memory-mapped register addresses for GPIOE on the STM32F3 Discovery board.
const GPIOE_BASE_ADDR: u32 = 0x48001000;
const GPIOE_MODER: *mut u32 = (GPIOE_BASE_ADDR + 0x00) as *mut u32;
const GPIOE_OTYPER: *mut u32 = (GPIOE_BASE_ADDR + 0x04) as *mut u32;
const GPIOE_BSRR: *mut u32 = (GPIOE_BASE_ADDR + 0x18) as *mut u32;

fn delay() {
    for _ in 0..100_000 {
        // A simple delay loop
        asm::nop();
    }
}

#[entry]
fn main() -> ! {
    // Enable clock for GPIOE in the RCC_AHBENR register (bit 21)
    let rcc_ahbenr = 0x4002_1014 as *mut u32;
    unsafe {
        *rcc_ahbenr |= 1 << 21;
    }

    // Configure GPIOE pins as push-pull outputs (bits 26-27 for PE8-PE15)
    unsafe {
        *GPIOE_MODER |= 0b0101_0101_0101_0101_0101_0101_0101_0101;
    }

    // Initialize LED states to off (clear bits in BSRR for PE8-PE15)
    unsafe {
        *GPIOE_BSRR = 0xFF << 8;
    }

    loop {
        // Toggle each LED in sequence
        for i in 0..8 {
            // Set the corresponding bit in BSRR to turn on the LED
            unsafe {
                *GPIOE_BSRR = 1 << (i + 8); // Turn on LED
            }
            delay();
            // Clear the corresponding bit in BSRR to turn off the LED
            unsafe {
                *GPIOE_BSRR = 1 << (i + 24); // Turn off LED
            }
        }
    }
}
