#![no_std]
#[cfg(not(feature = "async"))]
use embedded_hal as hal;
#[cfg(feature = "async")]
use embedded_hal_async as hal;


use hal::i2c::I2c;

pub struct I2cScanner<I2C> {
    i2c: I2C
}

impl<I2C: I2c> I2cScanner<I2C> {
    /// Creates a new scanner instance using the given I2C bus
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    /// Destroys the scanner and returns the used I2C bus
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Check for a device at a specific 7-bit address
    pub async fn check(&mut self, addr: u8) -> bool {
        self.i2c.read(addr, &mut [0]).await.is_ok()
    }

    /// Scans all available 7-bit addresses
    ///
    /// Returns [u8; 128] array where 0 == miss and 1 == hit for each index as an i2c device address
    pub async fn scan(&mut self) -> [u8; 128] {
        let mut addrs = [0u8; 128];

        for i in 0..addrs.len() {
            match self.i2c.read(i as u8, &mut [0]).await {
                Ok(_) => addrs[i] = 1,
                Err(_) => {}
            }
        }

        addrs
    }
}

#[cfg(test)]
mod tests {
    use embedded_hal::i2c::ErrorKind;
    use super::*;
    use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

    #[tokio::test]
    async fn check_hit() {
        let addr = 3u8;
        let expectations = [
            I2cTransaction::read(addr, [0x00].to_vec())
        ];
        let i2c = I2cMock::new(&expectations);
        let mut scanner = I2cScanner::new(i2c);
        let res = scanner.check(addr).await;
        assert!(res);
        let mut i2c = scanner.destroy();
        i2c.done();
    }

    #[tokio::test]
    async fn check_miss() {
        let addr = 3u8;
        let expectations = [
            I2cTransaction::read(addr, [0x00].to_vec()).with_error(ErrorKind::Other)
        ];
        let i2c = I2cMock::new(&expectations);
        let mut scanner = I2cScanner::new(i2c);
        let res = scanner.check(addr).await;
        assert!(!res);
        let mut i2c = scanner.destroy();
        i2c.done();
    }
}