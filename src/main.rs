use esp_idf_hal::gpio::Gpio4;
use esp_idf_hal::gpio::Gpio5;
use esp_idf_hal::gpio::Unknown;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use anyhow::Result;
use esp_idf_hal::i2c;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::units::FromValueType;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

fn display_test(i2c0: i2c::I2C0, scl: Gpio4<Unknown>, sda: Gpio5<Unknown>) -> Result<()> {
    let i2c = i2c::Master::new(
        i2c0,
        i2c::MasterPins {
            scl: scl.into_output().unwrap(),       // O
            sda: sda.into_input_output().unwrap(), // I+O
        },
        i2c::config::MasterConfig::new().baudrate(400.kHz().into()),
    )?;

    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display
        .init()
        .map_err(|e| anyhow::anyhow!("Init error: {:?}", e))?;

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .map_err(|e| anyhow::anyhow!("Txt error: {:?}", e))?;

    Text::with_baseline("Hello Rust!", Point::new(0, 16), text_style, Baseline::Top)
        .draw(&mut display)
        .map_err(|e| anyhow::anyhow!("Txt2 error: {:?}", e))?;

    display
        .flush()
        .map_err(|e| anyhow::anyhow!("Flush error: {:?}", e))?;

    Ok(())
}

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    println!("Hello, world from a ESP32 C3 test!");

    let peripherals = Peripherals::take().unwrap();

    if let Err(e) = display_test(
        peripherals.i2c0,
        peripherals.pins.gpio4,
        peripherals.pins.gpio5,
    ) {
        println!("Display error: {:?}", e)
    } else {
        println!("Display ok");
    }
}
