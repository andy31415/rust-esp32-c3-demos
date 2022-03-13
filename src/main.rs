use core::time::Duration;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use esp_idf_hal::gpio::{Gpio4, Gpio5, Unknown};
use esp_idf_hal::i2c;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::units::FromValueType;
use esp_idf_svc::netif::EspNetifStack;
use esp_idf_svc::nvs::EspDefaultNvs;
use esp_idf_svc::sysloop::EspSysLoopStack;
use esp_idf_svc::wifi::EspWifi;
use std::sync::Arc;

use embedded_svc::wifi::{
    ClientConfiguration, ClientConnectionStatus, ClientIpStatus, ClientStatus, Configuration, Wifi,
};

use anyhow::Result;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

fn display_test(i2c0: i2c::I2C0, scl: Gpio4<Unknown>, sda: Gpio5<Unknown>, ip: &str) -> Result<()> {
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

    Text::with_baseline(
        &format!("IP: {}", ip),
        Point::new(0, 16),
        text_style,
        Baseline::Top,
    )
    .draw(&mut display)
    .map_err(|e| anyhow::anyhow!("Txt2 error: {:?}", e))?;

    display
        .flush()
        .map_err(|e| anyhow::anyhow!("Flush error: {:?}", e))?;

    Ok(())
}

fn test_wifi() -> Result<String> {
    let netif_stack = Arc::new(EspNetifStack::new()?);
    let sys_look_stack = Arc::new(EspSysLoopStack::new()?);
    let nvs = Arc::new(EspDefaultNvs::new()?);

    let mut wifi = EspWifi::new(netif_stack, sys_look_stack, nvs)?;

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: "iot-test".into(),
        password: "test1234".into(),
        ..Default::default()
    }))?;

    wifi.wait_status_with_timeout(Duration::from_secs(30), |s| !s.is_transitional())
        .map_err(|e| anyhow::anyhow!("Wait timeout: {:?}", e))?;

    let status = wifi.get_status();

    println!("Status: {:?}", status);

    if let ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(
        client_settings,
    ))) = status.0
    {
        Ok(format!("{:?}", client_settings.ip))
    } else {
        Err(anyhow::anyhow!("Failed to connect in time."))
    }
}

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    println!("Hello, world from a ESP32 C3 test!");

    let peripherals = Peripherals::take().unwrap();

    let wifi = test_wifi();
    let ip = match wifi {
        Err(e) => {
            println!("Wifi error: {:?}", e);
            format!("ERR: {:?}", e)
        }
        Ok(s) => s,
    };

    if let Err(e) = display_test(
        peripherals.i2c0,
        peripherals.pins.gpio4,
        peripherals.pins.gpio5,
        &ip,
    ) {
        println!("Display error: {:?}", e)
    } else {
        println!("Display ok");
    }
}
