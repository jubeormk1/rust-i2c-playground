#![no_std]
#![no_main]

use core::cell::RefCell;
use critical_section::Mutex;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, delay::Delay, gpio::IO, i2c::I2C, peripherals::Peripherals, prelude::*,
};
use i2c_playground::tnh::SHTC3;

#[entry]
fn main() -> ! {
    let peripherals_core = Peripherals::take();
    let system = peripherals_core.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    let delay = Delay::new(&clocks);

    esp_println::logger::init_logger_from_env();

    let io = IO::new(peripherals_core.GPIO, peripherals_core.IO_MUX);
    let i2_c0 = peripherals_core.I2C0;
    let gpio1 = io.pins.gpio1;
    let gpio2 = io.pins.gpio2;

    let i2c_handler = I2C::new(i2_c0, gpio1, gpio2, 100.kHz(), &clocks, None);

    let i2c_mutex = Mutex::new(RefCell::new(i2c_handler));

    log::info!("i2c playground: Playing with TH sensor SHTC3!");

    let mut shtc3 = SHTC3::new(&i2c_mutex);
    shtc3.init();

    loop {
        match shtc3.read_temp_hum() {
            Ok((temperature, humidity)) => {
                log::info!("T: {}, H: {}", temperature, humidity);
            }
            Err(err_code) => {
                log::error!("Error getting T&H: {:?}", err_code);
            }
        };

        delay.delay(2.secs());
    }
}
