use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::prelude::*;
use anyhow::Result;

// MAX30102 I2C address according to the datasheet
const MAX30102_ADDR: u8 = 0x57;

fn main() -> Result<()> {
    // Initialize the ESP-IDF runtime
    esp_idf_svc::sys::link_patches();

    // Take ownership of the peripherals
    let peripherals = Peripherals::take()?;

    // Configure I2C pins
    let data_pin = peripherals.pins.gpio21;
    let clock_pin = peripherals.pins.gpio22;

    // Configure the I2C interface
    // Baudrate: 400 kHz (Fast Mode according to the MAX30102 datasheet)
    let config = I2cConfig::new().baudrate(400.kHz().into());

    let mut i2c = I2cDriver::new(
        peripherals.i2c0,
        data_pin,
        clock_pin,
        &config,
    )?;

    println!("I2C bus successfully initialized at 400kHz!");

    // Test communication with the MAX30102 sensor
    let mut part_id_buf = [0u8; 1];
    let reg_address = [0xFFu8];

    // Perform a write-read transaction: tell it which register we want, then read 1 byte back
    match i2c.write_read(MAX30102_ADDR, &reg_address, &mut part_id_buf, FreeRtos::TICK_PERIOD) {
        Ok(_) => {
            if part_id_buf[0] == 0x15 {
                println!("Success! Found MAX30102 sensor. Part ID: 0x{:02X}", part_id_buf[0]);
            } else {
                println!("Warning: Device responded, but Part ID 0x{:02X} does not match MAX30102.", part_id_buf[0]);
            }
        }
        Err(e) => {
            println!("Failed to communicate with sensor over I2C: {:?}", e);
            println!("Check your pull-up resistors and wiring connections!");
        }
    }
    // ESP32 logic
    loop {
        FreeRtos::delay_ms(1000);
    }
}
