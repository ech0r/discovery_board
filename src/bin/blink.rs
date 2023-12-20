#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f3xx_hal::{self as hal, prelude::*};
use discovery_board as _; // global logger + panicking-behavior + memory layout

#[entry]
fn main() -> ! {
    let dp = hal::pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let led1 = gpioe.pe13.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade();
    let led2 = gpioe.pe14.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade();
    let led3 = gpioe.pe15.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade();
    let led4 = gpioe.pe8.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade();
    let led5 = gpioe.pe9.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade();
    let led6 = gpioe.pe10.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade();
    let led7 = gpioe.pe11.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade();
    let led8 = gpioe.pe12.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade();
    let mut led_ring = [led1, led2, led3, led4, led5, led6, led7, led8];


    loop {
        defmt::println!("{:?}", rcc);
        led_ring[0].toggle().unwrap();
        asm::delay(1_000_000);
        led_ring[0].toggle().unwrap();
        led_ring[1].toggle().unwrap();
        asm::delay(1_000_000);
        led_ring[1].toggle().unwrap();
        led_ring[2].toggle().unwrap();
        asm::delay(1_000_000);
        led_ring[2].toggle().unwrap();
        led_ring[3].toggle().unwrap();
        asm::delay(1_000_000);
        led_ring[3].toggle().unwrap();
        led_ring[4].toggle().unwrap();
        asm::delay(1_000_000);
        led_ring[4].toggle().unwrap();
        led_ring[5].toggle().unwrap();
        asm::delay(1_000_000);
        led_ring[5].toggle().unwrap();
        led_ring[6].toggle().unwrap();
        asm::delay(1_000_000);
        led_ring[6].toggle().unwrap();
        led_ring[7].toggle().unwrap();
        asm::delay(1_000_000);
        led_ring[7].toggle().unwrap();
    }
}

