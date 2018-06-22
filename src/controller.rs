use i2cdev::core::*;

use ::constants;
use ::led_channel::LEDChannel;

#[derive(Debug)]
pub struct Controller<'a, T: I2CDevice + 'a> {
    device: &'a mut T,
}

impl<'a, T: I2CDevice + 'a> Controller<'a, T> {

    pub fn new(dev: &'a mut T) -> Controller<'a, T> {
        Controller{
            device: dev,
        }
    }

    /// Set `channel_num`'s registers to the on/off values given.
    /// Each channel has two 12-bit registers -- one for ON and one for OFF.
    /// `set_channel` takes 2 `u16` values for on and off times and they are modified as such:
    /// 
    /// ```
    /// let on: u16 = 0xfca;
    /// let off: u16 = 0xaba;
    /// 
    /// let on_low: u8 = (on & 0xff) as u8;
    /// let on_high: u8 = (on >> 8) as u8;
    /// let off_low: u8 = (off & 0xff) as u8;
    /// let off_high: u8 = (off >> 8) as u8;
    /// 
    /// assert_eq!(0xca, on_low);
    /// assert_eq!(0x0f, on_high);
    /// assert_eq!(0xba, off_low);
    /// assert_eq!(0x0a, off_high);
    /// ```
    pub fn set_channel(&mut self, channel_num: u8, on: u16, off: u16) -> Result<(), T::Error> {
        let channel = LEDChannel::new(channel_num).unwrap();
        let data = [
            (on & 0xff) as u8,
            (on >> 8) as u8,
            (off & 0xff) as u8,
            (off >> 8) as u8,
        ];
        return channel.write_channel(self.device, data);
    }

    pub fn set_all_channels(&mut self, on: u16, off: u16) -> Result<(), T::Error> {
        let values = vec![
            (constants::ALL_LED_ON_LOW, (on & 0xff) as u8),
            (constants::ALL_LED_ON_HIGH, (on >> 8) as u8),
            (constants::ALL_LED_OFF_LOW, (off & 0xff) as u8),
            (constants::ALL_LED_OFF_HIGH, (off >> 8) as u8),
        ];

        for value in values {
            let (register, value) = value;
            let result = self.device.smbus_write_byte_data(register, value);
            if result.is_err() {
                return Err(result.unwrap_err());
            }
        }

        Ok(())
    }

}