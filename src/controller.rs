use i2cdev::core::*;

use ::constants;
use ::led_channel::LEDChannel;

pub struct Controller<'a, T: I2CDevice + 'a> {
    device: &'a mut T,
}

impl<'a, T: I2CDevice + 'a> Controller<'a, T> {

    pub fn new(dev: &'a mut T) -> Controller<'a, T> {
        Controller{
            device: dev,
        }
    }

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