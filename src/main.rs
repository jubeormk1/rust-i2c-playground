#![no_std]
#![no_main]

use defmt::println;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, delay::Delay, gpio::IO, i2c::I2C, peripherals::Peripherals, prelude::*,
};

use shtcx::{self, LowPower};

#[entry]
fn main() -> ! {
    let peripherals_core = Peripherals::take();
    let system = peripherals_core.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    let mut delay = Delay::new(&clocks);

    esp_println::logger::init_logger_from_env();

    let mut io = IO::new(peripherals_core.GPIO, peripherals_core.IO_MUX);
    let i2_c0 = peripherals_core.I2C0;
    let gpio1 = io.pins.gpio1.internal_pull_up(true);
    let gpio2 = io.pins.gpio2.internal_pull_up(true);

    let i2c_handler = I2C::new(i2_c0, gpio1, gpio2, 100.kHz(), &clocks, None);
    let mut sht = shtcx::shtc3(i2c_handler);

    println!("Starting SHTC3 tests.");
    println!("Waking up sensor.");
    println!("");
    sht.wakeup(&mut delay).expect("Wakeup failed");

    println!(
        "Device identifier: 0x{:02x}",
        sht.device_identifier()
            .expect("Failed to get device identifier")
    );
    println!(
        "Raw ID register:   0b{:016b}",
        sht.raw_id_register()
            .expect("Failed to get raw ID register")
    );

    println!("\nNormal mode measurements:");

    loop {
        // for _ in 0..3 {
        //     let measurement = sht
        //         .measure(PowerMode::NormalMode, &mut delay)
        //         .expect("Normal mode measurement failed");
        //     println!(
        //         "  {:.2} °C | {:.2} %RH",
        //         measurement.temperature.as_degrees_celsius(),
        //         measurement.humidity.as_percent(),
        //     );
        // }
        println!("WIP");
        delay.delay(2.secs());
    }
}
