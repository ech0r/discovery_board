#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use discovery_board as _; // global logger + panicking-behavior + memory layout

// Memory-mapped register addresses for GPIOE on the STM32F3 Discovery board.
// USB address range: 0x4000 5C00 - 0x4000 5FFF 
const USB_BASE_ADDR: u32 = 0x4000_5C00;
const RCC_BASE: u32 = 0x4002_1000;

// USBPRE at bit 22
const RCC_CFGR: *mut u32 = (RCC_BASE + 0x04) as *mut u32;
// USBRST at bit 23
const RCC_APB1RSTR: *mut u32 = (RCC_BASE + 0x010) as *mut u32;
// USBEN at bit 23
const RCC_APB1ENR: *mut u32 = (RCC_BASE + 0x1C) as *mut u32;

const GPIOE_BASE_ADDR: u32 = 0x4800_1000;
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
    // Enable clock for USB in the RCC
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
        for i in 0..8 {
            unsafe {
                *GPIOE_BSRR = 1 << (i +8);
            }
            delay();
            unsafe {
                defmt::println!("{}", GPIOE_BSRR.read_volatile());
            }
            unsafe {
                *GPIOE_BSRR = 1 << (i + 24);
            }
        }
    }
}
