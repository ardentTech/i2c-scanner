#[cfg(not(feature = "async"))]
use embedded_hal as hal;
use embedded_hal::i2c::SevenBitAddress;
#[cfg(feature = "async")]
use embedded_hal_async as hal;

use hal::i2c::I2c;
pub struct SevenBitScanner<I2C> {
    i2c: I2C
}

impl<I2C: I2c<SevenBitAddress>> SevenBitScanner<I2C> {
    /// Creates a new scanner instance using the given I2C bus
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    /// Destroys the scanner and returns the used I2C bus
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Check for a device at a specific 7-bit address
    #[cfg(not(feature = "async"))]
    pub fn check(&mut self, addr: SevenBitAddress) -> bool {
        self.i2c.read(addr, &mut [0]).is_ok()
    }
    #[cfg(feature = "async")]
    pub async fn check(&mut self, addr: SevenBitAddress) -> bool {
        self.i2c.read(addr, &mut [0]).await.is_ok()
    }

    /// Scans all available 7-bit addresses
    ///
    /// Returns [u8; 128] array where 0 == miss and 1 == hit for each index as an i2c device address
    #[cfg(not(feature = "async"))]
    pub fn scan(&mut self) -> [SevenBitAddress; 128] {
        let mut addrs = [0u8; 128];

        for i in 0..addrs.len() {
            match self.i2c.read(i as SevenBitAddress, &mut [0]) {
                Ok(_) => addrs[i] = 1,
                Err(_) => {}
            }
        }

        addrs
    }

    #[cfg(feature = "async")]
    pub async fn scan(&mut self) -> [SevenBitAddress; 128] {
        let mut addrs = [0u8; 128];

        for i in 0..addrs.len() {
            match self.i2c.read(i as SevenBitAddress, &mut [0]).await {
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
    use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
    use super::*;

    #[cfg(not(feature = "async"))]
    #[test]
    fn check_hit() {
        let addr = 3u8;
        let expectations = [
            I2cTransaction::read(addr, [0x00].to_vec())
        ];
        let i2c = I2cMock::new(&expectations);
        let mut scanner = SevenBitScanner::new(i2c);
        let res = scanner.check(addr);

        assert!(res);

        scanner.destroy().done();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn check_hit() {
        let addr = 3u8;
        let expectations = [
            I2cTransaction::read(addr, [0x00].to_vec())
        ];
        let i2c = I2cMock::new(&expectations);
        let mut scanner = SevenBitScanner::new(i2c);
        let res = scanner.check(addr).await;

        assert!(res);

        scanner.destroy().done();
    }

    #[cfg(not(feature = "async"))]
    #[test]
    fn check_miss() {
        let addr = 3u8;
        let expectations = [
            I2cTransaction::read(addr, [0x00].to_vec()).with_error(ErrorKind::Other)
        ];
        let i2c = I2cMock::new(&expectations);
        let mut scanner = SevenBitScanner::new(i2c);
        let res = scanner.check(addr);

        assert!(!res);

        scanner.destroy().done();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn check_miss() {
        let addr = 3u8;
        let expectations = [
            I2cTransaction::read(addr, [0x00].to_vec()).with_error(ErrorKind::Other)
        ];
        let i2c = I2cMock::new(&expectations);
        let mut scanner = SevenBitScanner::new(i2c);
        let res = scanner.check(addr).await;

        assert!(!res);

        scanner.destroy().done();
    }

    #[cfg(not(feature = "async"))]
    #[test]
    fn scan_miss() {
        let expectations: [I2cTransaction; 128] = core::array::from_fn(|i| I2cTransaction::read(i as u8, [0x00].to_vec()).with_error(ErrorKind::Other));
        let i2c = I2cMock::new(&expectations);
        let mut scanner = SevenBitScanner::new(i2c);
        let res = scanner.scan();

        assert_eq!(res, [0; 128]);

        scanner.destroy().done();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn scan_miss() {
        let expectations: [I2cTransaction; 128] = core::array::from_fn(|i| I2cTransaction::read(i as u8, [0x00].to_vec()).with_error(ErrorKind::Other));
        let i2c = I2cMock::new(&expectations);
        let mut scanner = SevenBitScanner::new(i2c);
        let res = scanner.scan().await;

        assert_eq!(res, [0; 128]);

        scanner.destroy().done();
    }

    #[cfg(not(feature = "async"))]
    #[test]
    fn scan_one_hit() {
        let mut expectations: [I2cTransaction; 128] = core::array::from_fn(|i| I2cTransaction::read(i as u8, [0x00].to_vec()).with_error(ErrorKind::Other));
        expectations[8] = I2cTransaction::read(8u8, [0x00].to_vec());
        let i2c = I2cMock::new(&expectations);
        let mut scanner = SevenBitScanner::new(i2c);
        let res = scanner.scan();

        assert_eq!(1u8, res.iter().sum());

        scanner.destroy().done();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn scan_one_hit() {
        let mut expectations: [I2cTransaction; 128] = core::array::from_fn(|i| I2cTransaction::read(i as u8, [0x00].to_vec()).with_error(ErrorKind::Other));
        expectations[8] = I2cTransaction::read(8u8, [0x00].to_vec());
        let i2c = I2cMock::new(&expectations);
        let mut scanner = SevenBitScanner::new(i2c);
        let res = scanner.scan().await;

        assert_eq!(1u8, res.iter().sum());

        scanner.destroy().done();
    }

    #[cfg(not(feature = "async"))]
    #[test]
    fn scan_multiple_hits() {
        let mut expectations: [I2cTransaction; 128] = core::array::from_fn(|i| I2cTransaction::read(i as u8, [0x00].to_vec()).with_error(ErrorKind::Other));
        for i in [2, 4, 8, 16] {
            expectations[i] = I2cTransaction::read(i as u8, [0x00].to_vec());
        }
        let i2c = I2cMock::new(&expectations);
        let mut scanner = SevenBitScanner::new(i2c);
        let res = scanner.scan();

        assert_eq!(4u8, res.iter().sum());

        scanner.destroy().done();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn scan_multiple_hits() {
        let mut expectations: [I2cTransaction; 128] = core::array::from_fn(|i| I2cTransaction::read(i as u8, [0x00].to_vec()).with_error(ErrorKind::Other));
        for i in [2, 4, 8, 16] {
            expectations[i] = I2cTransaction::read(i as u8, [0x00].to_vec());
        }
        let i2c = I2cMock::new(&expectations);
        let mut scanner = SevenBitScanner::new(i2c);
        let res = scanner.scan().await;

        assert_eq!(4u8, res.iter().sum());

        scanner.destroy().done();
    }
}