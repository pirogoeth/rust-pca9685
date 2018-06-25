use env_logger;
use i2cdev::core::*;
use std::{thread, time};

use ::constants;
use ::channel::base::Channel;

/// Calculates a value to insert into PRE_SCALE register where
/// `update_rate` is the output modulation frequency in Hertz.
/// 
/// Implemented from the [PCA9685 datasheet](https://cdn-shop.adafruit.com/datasheets/PCA9685.pdf#G4466866).
/// 
/// # Examples
/// 
/// ```
/// # extern crate rust_pca9685;
/// # use rust_pca9685::controller::calculate_prescale_value;
/// #
/// let update_rate = 200.0; // update rate in hz
/// let prescale_value = calculate_prescale_value(update_rate);
/// assert_eq!(0x1e, prescale_value);
/// ```
pub fn calculate_prescale_value(update_rate: f32) -> u8 {
    let base_val = constants::OSCILLATION_FREQ / (constants::STEP_SIZE * update_rate);
    return (base_val.round() - 1.0) as u8;
}

#[derive(Debug)]
pub struct Controller<'a, T: I2CDevice + 'a> {
    device: &'a mut T,
}

impl<'a, T: I2CDevice + 'a> Controller<'a, T> {

    pub fn new(dev: &'a mut T) -> Controller<'a, T> {
        let mut c = Controller{ device: dev };
        {
            c.set_up().unwrap();
        }

        return c;
    }

    /// Performs some initial set up on the PCA9685.
    /// - Set `OUTDRV` on `MODE_2`
    /// - Set `ALLCALL` on `MODE_1`
    /// - Sleep waiting on oscillator
    /// - Read `MODE_1` back
    /// - Unset the `SLEEP` bit on `MODE_1` to wake up controller
    /// - Sleep waiting on oscillator
    /// 
    /// This puts the controller `MODE_*` registers into a known state at
    /// the beginning of operation.
    /// 
    /// At the beginning of operation, you can expect:
    /// - `MODE_1` *should* be set to `0x01` (ALL_CALL)
    /// - `MODE_2` *should* be set to `0x04` (OUTDRV)
    fn set_up(&mut self) -> Result<(), T::Error> {
        self.device.smbus_write_byte_data(constants::MODE_2, constants::OUTDRV)?;
        self.device.smbus_write_byte_data(constants::MODE_1, constants::ALL_CALL)?;
        thread::sleep(time::Duration::from_millis(5));

        let mode1 = self.device.smbus_read_byte_data(constants::MODE_1)?;
        let mode1 = mode1 & (!constants::SLEEP);
        self.device.smbus_write_byte_data(constants::MODE_1, mode1)?;
        thread::sleep(time::Duration::from_millis(5));;

        Ok(())
    }

    /// Sets the controller's output modulation rate.
    /// Expects `prescale_value` to be a value that has been calculated
    /// by [rust_pca9685::controller::calculate_prescale_value][calculate_prescale_value].
    /// 
    /// After `set_pwm_rate`, `MODE_1` will also have the `RESTART` bit set so channel
    /// values will persist even if the controller is put to `SLEEP`.
    /// 
    /// [calculate_prescale_value]: fn.calculate_prescale_value.html
    pub fn set_pwm_rate(&mut self, prescale_value: u8) -> Result<(), T::Error> {
        let _ = env_logger::try_init();

        // Save the old controller mode for revert
        let old_mode = self.device.smbus_read_byte_data(constants::MODE_1)?;
        debug!("MODE_1 is {:#04x} before going to sleep", old_mode);

        // Make a new mode value to sleep the controller
        // NOTE: `0x7f` carries the `EXTCLK`, `AI`, `SLEEP`, `SUB1`, `SUB2`, `SUB3`, `ALL_CALL` bits
        // over to the new mode, intentionally eliding over the `RESTART` value
        let new_mode = (old_mode & 0x7f) | constants::SLEEP;

        // Effectively put the controller to sleep
        self.device.smbus_write_byte_data(constants::MODE_1, new_mode)?;
        debug!("wrote {:#04x} to MODE_1", new_mode);

        // Write `prescale_value` to the `PRE_SCALE` register
        debug!("setting output modulation frequency to {} (prescale)", prescale_value);
        self.device.smbus_write_byte_data(constants::PRE_SCALE, prescale_value)?;

        // Restore the old `MODE_1` flags
        self.device.smbus_write_byte_data(constants::MODE_1, old_mode)?;
        debug!("restored {:#04x} to MODE_1", old_mode);

        // Wait for oscillator to stabilize before setting `RESTART`
        thread::sleep(time::Duration::from_millis(5));

        // Set restart bit; forces all channels to remain in their state when the clock is off
        let restart_mode = old_mode | constants::RESTART;
        self.device.smbus_write_byte_data(constants::MODE_1, restart_mode)?;
        debug!("enable restart mode sets MODE_1 {:#04x}", restart_mode);

        Ok(())
    }

    /// Set `channel`'s registers to the on/off values given.
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
    pub fn set_channel<C: Channel>(&mut self, channel: &mut C, on: u16, off: u16) -> Result<(), T::Error> {
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