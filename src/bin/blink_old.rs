#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f3xx_hal::{self as hal, pac, prelude::*};
use discovery_board as _; // global logger + panicking-behavior + memory layout

fn initialize_leds() -> [Pin; 8] {
    let mut led1 = gpioe
        .pe13
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led2 = gpioe
        .pe14
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led3 = gpioe
        .pe15
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led4 = gpioe
        .pe8
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led5 = gpioe
        .pe9
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led6 = gpioe
        .pe10
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led7 = gpioe
        .pe11
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led8 = gpioe
        .pe12
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    [led1, led2, led3, led4, led5, led6, led7, led8]
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    
    let led_ring = initialize_leds();

    loop {
        for led in led_ring {
            led.toggle().unwrap();
            asm::delay(1_000_000);
            led.toggle().unwrap();
        }
    }
}
