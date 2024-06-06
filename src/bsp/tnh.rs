use core::cell::RefCell;

use critical_section::Mutex;

use crate::{i2c_devs::I2CType, QuickError};

// static I2C_H: Mutex<RefCell<Option<I2CType>>> = Mutex::new(RefCell::new(None));

static SHTC3_ADDR: u8 = 0x70;
static CMD_SLEEP: [u8; 2] = [0xb0, 0x98];
static CMD_WAKEUP: [u8; 2] = [0x35, 0x17];
static CMD_SOFT_RESET: [u8; 2] = [080, 0x5d];
static CMD_READ_ID: [u8; 2] = [0xef, 0xc8];

pub struct SHTC3<'a> {
    i2c_h: &'a Mutex<RefCell<I2CType<'a>>>,
    id: Option<[u8; 3]>,
}

impl<'a> SHTC3<'a> {
    pub fn new(i2c_h: &'a Mutex<RefCell<I2CType<'a>>>) -> Self {
        Self { i2c_h, id: None }
    }

    pub fn init(&mut self) {
        // Add your initialization code here
        self.write(&CMD_SOFT_RESET);
        self.write(&CMD_SLEEP);
        self.write(&CMD_WAKEUP);
        let mut buffer_id = [0; 3];
        self.write_read(&CMD_READ_ID, &mut buffer_id);
        if buffer_id != [0; 3] {
            self.id = Some(buffer_id);
        }
    }

    pub fn read_temp_hum(&self) -> Result<(f32, f32), QuickError> {
        Err(QuickError::FeatureNotImplemented)
    }

    pub fn read_temp(&self) -> Result<f32, QuickError> {
        Err(QuickError::FeatureNotImplemented)
    }
    pub fn soft_reset(&self) -> Result<(), QuickError> {
        Err(QuickError::FeatureNotImplemented)
    }

    fn write(&mut self, cmd: &[u8; 2]) {
        critical_section::with(|cs| {
            let mut i2c = self.i2c_h.borrow_ref_mut(cs);
            if let Err(e) = i2c.write(SHTC3_ADDR, cmd) {
                log::error!("error writting {:?}: {:?}", cmd, e);
            } else {
                log::info!("wrote {:?}", cmd);
            };
        });
    }
    fn write_read(&mut self, cmd: &[u8; 2], buffer: &mut [u8]) {
        critical_section::with(|cs| {
            let mut i2c = self.i2c_h.borrow_ref_mut(cs);
            if let Err(e) = i2c.write_read(SHTC3_ADDR, cmd, buffer) {
                log::error!("error writting reading {:?}: {:?}", cmd, e);
            } else {
                log::info!("wrote {:?}, read{:?}", cmd, buffer);
            };
        });
    }
}
