//! Scans magnetometer over I2C and displays the result
//! Target board: STM32F3DISCOVERY

#![no_std]
#![no_main]

use core::{convert::TryInto, ops::Range};

use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f3xx_hal::{self as hal, pac, i2c, prelude::*};
use discovery_board as _;

const VALID_ADDR_RANGE: Range<u8> = 0x08..0x78;

#[entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    let dp = hal::pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Configure the clocks
    let clocks = rcc
        .cfgr
        .sysclk(64.MHz())
        .pclk1(32.MHz())
        .freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    // I2C1 pins
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    // I2C configuration
    let i2c_config = i2c::Config::new(100.kHz(), clocks);
    let mut i2c = pac::i2c1(dp.I2C1, (scl, sda), i2c_config, clocks, &mut rcc.apb1);

    let magnetometer_address = 0x1E;  // Replace with your device's I2C address
    let register_address = 0x00;  // Replace with the specific register you want to read/write

    // Write to the magnetometer
    let write_data: [u8; 2] = [register_address, 0x01]; // Replace 0x01 with the data to write
    if i2c.write(magnetometer_address, &write_data).is_ok() {
         // Successfully wrote to the magnetometer
    }

    // Read from the magnetometer
    let mut read_data: [u8; 1] = [0];
    if i2c.write_read(magnetometer_address, &[register_address], &mut read_data).is_ok() {
        // Successfully read from the magnetometer
        let value = read_data[0];
    }

    loop {
        asm::wfi();
    }
}
