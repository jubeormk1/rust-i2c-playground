#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, delay::Delay, gpio::IO, i2c::I2C, peripherals::Peripherals, prelude::*,
};

use shtcx::{self, LowPower, PowerMode};

#[entry]
fn main() -> ! {
    let peripherals_core = Peripherals::take();
    let system = peripherals_core.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    let mut delay = Delay::new(&clocks);

    esp_println::logger::init_logger_from_env();

    let io = IO::new(peripherals_core.GPIO, peripherals_core.IO_MUX);
    let i2_c0 = peripherals_core.I2C0;
    let gpio_sda = io.pins.gpio10;
    let gpio_scl = io.pins.gpio8;

    let i2c_handler = I2C::new(i2_c0, gpio_sda, gpio_scl, 100.kHz(), &clocks, None);
    let mut sht = shtcx::shtc3(i2c_handler);

    log::info!("Starting SHTC3 tests.");
    log::info!("Waking up sensor.");
    log::info!("");
    if let Err(e) = sht.wakeup(&mut delay) {
        log::error!("Error waking up sensor: {:?}", e);
    };
    match sht.device_identifier() {
        Ok(id) => {
            log::info!("Device identifier: 0x{:02x}", id);
        }
        Err(e) => {
            log::error!("Failed to get device identifier: {:?}", e);
        }
    }

    match sht.raw_id_register() {
        Ok(id) => {
            log::info!("Raw ID register: 0b{:016b}", id);
        }
        Err(e) => {
            log::error!("Failed to get raw ID register: {:?}", e);
        }
    }

    log::info!("\nNormal mode measurements:");
    for _ in 0..10 {
        match sht.measure(PowerMode::NormalMode, &mut delay) {
            Ok(measurement) => {
                log::info!(
                    "  {:.2} °C | {:.2} %RH",
                    measurement.temperature.as_degrees_celsius(),
                    measurement.humidity.as_percent(),
                );
            }
            Err(e) => {
                log::error!("Error measuring: {:?}", e);
            }
        }
    }

    log::info!("\nLow power mode measurements:");
    for _ in 0..10 {
        match sht.measure(PowerMode::LowPower, &mut delay) {
            Ok(measurement) => {
                log::info!(
                    "  {:.2} °C | {:.2} %RH",
                    measurement.temperature.as_degrees_celsius(),
                    measurement.humidity.as_percent(),
                );
            }
            Err(e) => {
                log::error!("Error measuring: {:?}", e);
            }
        }
    }

    log::info!("\nTesting power management:");
    log::info!("-> Measure: ");
    match sht.measure_temperature(PowerMode::NormalMode, &mut delay) {
        Ok(temperature) => {
            log::info!("Success: {:.2} °C", temperature.as_degrees_celsius());
        }
        Err(e) => {
            log::error!("Error: {:?}", e);
        }
    }

    log::info!("-> Sleep");
    match sht.sleep() {
        Ok(_) => {
            log::info!("Sleep command succeeded");
        }
        Err(e) => {
            log::error!("Sleep command failed: {:?}", e);
        }
    }

    log::info!("-> Measure: ");
    match sht.measure_temperature(PowerMode::NormalMode, &mut delay) {
        Ok(temperature) => {
            log::info!("Success: {:.2} °C", temperature.as_degrees_celsius());
        }
        Err(e) => {
            log::error!("Error: {:?}", e);
        }
    }

    log::info!("-> Wakeup");
    match sht.wakeup(&mut delay) {
        Ok(_) => {
            log::info!("Wakeup command succeeded");
        }
        Err(e) => {
            log::error!("Wakeup command failed: {:?}", e);
        }
    }

    log::info!("-> Measure: ");
    match sht.measure_temperature(PowerMode::NormalMode, &mut delay) {
        Ok(temperature) => {
            log::info!("Success: {:.2} °C", temperature.as_degrees_celsius());
        }
        Err(e) => {
            log::error!("Error: {:?}", e);
        }
    }

    log::info!("-> Soft reset");
    match sht.reset(&mut delay) {
        Ok(_) => {
            log::info!("Reset command succeeded");
        }
        Err(e) => {
            log::error!("Reset command failed: {:?}", e);
        }
    }

    log::info!("-> Measure: ");
    match sht.measure_temperature(PowerMode::NormalMode, &mut delay) {
        Ok(temperature) => {
            log::info!("Success: {:.2} °C", temperature.as_degrees_celsius());
        }
        Err(e) => {
            log::error!("Error: {:?}", e);
        }
    }

    loop {
        delay.delay(2.secs());
    }
}
