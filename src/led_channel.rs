use std::fmt;
use i2cdev::core::*;

use ::{
    constants,
    errors,
};

#[derive(Clone, Copy, Debug)]
pub struct LEDChannel {
    // Must be between 0 and 15
    channel_num: u8,
}

impl fmt::Display for LEDChannel {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "( ON_L: {:#x}, ON_H: {:#x}, OFF_L: {:#x}, OFF_H: {:#x} )",
            self.on_low(),
            self.on_high(),
            self.off_low(),
            self.off_high(),
        )
    }

}

impl LEDChannel {

    /// Creates a LEDChannel at the specified channel register.
    pub fn new(channel_num: u8) -> Result<LEDChannel, errors::ChannelRangeError> {
        if channel_num > 15 {
            return Err(errors::ChannelRangeError);
        }

        Ok(
            LEDChannel{
                channel_num: channel_num,
            }
        )
    }

    /// Calculate the register value for a given base using the set `channel_num`.
    fn register_offset(self, base: u8) -> u8 {
        base + (4 * self.channel_num)
    }

    /// Returns the first register address that is used for this channel.
    /// In the case of the PCA9685, the "base" register address would be the
    /// `LEDn_ON_LOW` register address.
    pub fn base_address(self) -> u8 {
        self.on_low()
    }

    pub fn on_low(self) -> u8 {
        self.register_offset(constants::BASE_LED_ON_LOW)
    }

    pub fn on_high(self) -> u8 {
        self.register_offset(constants::BASE_LED_ON_HIGH)
    }

    pub fn off_low(self) -> u8 {
        self.register_offset(constants::BASE_LED_OFF_LOW)
    }

    pub fn off_high(self) -> u8 {
        self.register_offset(constants::BASE_LED_OFF_HIGH)
    }

    pub fn on_addrs(self) -> (u8, u8) {
        (
            self.on_low(),
            self.on_high(),
        )
    }

    pub fn off_addrs(self) -> (u8, u8) {
        (
            self.off_low(),
            self.off_high(),
        )
    }

    /// Reads the values for a LED channel's registers.
    /// Will read one `u8` value from `on_addrs` and `off` addrs and return
    /// a slice of the value.
    pub fn read_channel<'a, T: I2CDevice + 'a>(&self, dev: &'a mut T) -> Result<[u8; 4], T::Error> {
        let mut results = Vec::with_capacity(4);

        results.push(dev.smbus_read_byte_data(self.on_low()));
        results.push(dev.smbus_read_byte_data(self.on_high()));
        results.push(dev.smbus_read_byte_data(self.off_low()));
        results.push(dev.smbus_read_byte_data(self.off_high()));

        let mut bytes = Vec::with_capacity(4);
        for result in results {
            if result.is_err() {
                return Err(result.unwrap_err());
            }

            bytes.push(result.unwrap());
        }

        let mut channel_values: [u8; 4] = [0, 0, 0, 0];
        for i in 0..4 {
            channel_values[i] = bytes[i];
        }

        Ok(channel_values)
    }

    /// Writes the values in `data` into a channel's registers.
    pub fn write_channel<'a, T: I2CDevice + 'a>(&self, dev: &'a mut T, data: [u8; 4]) -> Result<(), T::Error> {
        let mut results = Vec::with_capacity(4);

        let reg_addrs = vec![self.on_low(), self.on_high(), self.off_low(), self.off_high()];
        let write_to = reg_addrs.into_iter().zip(data.into_iter());

        for (reg, byte) in write_to.into_iter() {
            results.push(dev.smbus_write_byte_data(reg, *byte));
        }

        for result in results {
            if result.is_err() {
                return Err(result.unwrap_err());
            }
        }

        Ok(())
    }

}