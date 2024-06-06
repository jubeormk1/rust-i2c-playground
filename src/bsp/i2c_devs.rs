use esp_hal::{i2c::I2C, peripherals::I2C0, Blocking};

pub type I2CType<'a> = I2C<'a, I2C0, Blocking>;

// Half cooked
// pub fn get_i2c_mutex<'a>(
//     pin_sda: impl Peripheral<P = OutputPin + InputPin> + 'a,
//     pin_scl: impl Peripheral<P = OutputPin + InputPin> + 'a,
//     i2c0: I2C0,
//     clocks: &Clocks,
// ) -> Mutex<RefCell<I2CType<'a>>> {
//     let i2c_handler = I2C::new(i2c0, io.pins.gpio1, io.pins.gpio2, 100.kHz(), &clocks, None);
//     Mutex::new(RefCell::new(i2c_handler))
// }
// let i2c_mutex = Mutex::new(RefCell::new(i2c_handler));
