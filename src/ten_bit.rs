#[cfg(not(feature = "async"))]
use embedded_hal as hal;
use embedded_hal::i2c::TenBitAddress;
#[cfg(feature = "async")]
use embedded_hal_async as hal;

use hal::i2c::I2c;

pub struct TenBitScanner<I2C> {
    i2c: I2C
}

impl<I2C: I2c<TenBitAddress>> TenBitScanner<I2C> {
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
    pub fn check(&mut self, addr: TenBitAddress) -> bool {
        self.i2c.read(addr, &mut [0]).is_ok()
    }
    #[cfg(feature = "async")]
    pub async fn check(&mut self, addr: TenBitAddress) -> bool {
        self.i2c.read(addr, &mut [0]).await.is_ok()
    }

    /// Scans all available 7-bit addresses
    ///
    /// Returns [u8; 1024] array where 0 == miss and 1 == hit for each index as an i2c device address
    #[cfg(not(feature = "async"))]
    pub fn scan(&mut self) -> [TenBitAddress; 1024] {
        let mut addrs = [0u16; 1024];

        for i in 0..addrs.len() {
            match self.i2c.read(i as TenBitAddress, &mut [0]) {
                Ok(_) => addrs[i] = 1,
                Err(_) => {}
            }
        }

        addrs
    }

    #[cfg(feature = "async")]
    pub async fn scan(&mut self) -> [TenBitAddress; 1024] {
        let mut addrs = [0u16; 1024];

        for i in 0..addrs.len() {
            match self.i2c.read(i as TenBitAddress, &mut [0]).await {
                Ok(_) => addrs[i] = 1,
                Err(_) => {}
            }
        }

        addrs
    }
}

#[cfg(test)]
mod tests {
    // TODO embedded-hal-mock only supports 7-bit addresses
}