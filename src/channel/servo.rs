use env_logger;
use std::fmt;

use ::channel::{
    base::Channel,
    errors,
};


#[derive(Clone, Copy, Debug)]
pub struct ServoSettings {
    min: u16,
    max: u16,
}

impl ServoSettings {

    pub fn new(min: u16, max: u16) -> ServoSettings {
        ServoSettings{
            min: min,
            max: max,
        }
    }

    /// Returns a `ServoSettings` struct with default values.
    /// Note that these are not necessarily safe defaults -- it greatly depends on the servo
    /// that is being controlled! For example, the Tower Pro SG90 
    pub fn defaults() -> ServoSettings {
        ServoSettings{
            min: 0,
            max: 4095,
        }
    }

    pub fn servo_range(self) -> (u16, u16) {
        (self.min, self.max)
    }

}

#[derive(Clone, Copy, Debug)]
pub struct ServoChannel {
    channel_num: u8,
    settings: ServoSettings,
}

impl ServoChannel {

    /// Creates a `ServoChannel` at the specified channel index.
    /// Uses the `ServoChannel::defaults()` method to get a default set of servo settings.
    pub fn new(channel_num: u8) -> Result<ServoChannel, errors::IndexRangeError> {
        if channel_num > 15 {
            return Err(errors::IndexRangeError);
        }

        Ok(
            ServoChannel{
                channel_num: channel_num,
                settings: ServoSettings::defaults(),
            }
        )
    }

    /// Creates a `ServoChannel` at the specified channel index.
    /// Uses a custom `ServoSettings` struct in lieu of calling `ServoChannel::defaults()`
    pub fn new_with_settings(channel_num: u8, settings: ServoSettings) -> Result<ServoChannel, errors::IndexRangeError> {
        let mut chan = ServoChannel::new(channel_num)?;
        chan.settings = settings;
        Ok(chan)
    }

    /// Checks if a value is in the configured range of the servo, based on `ServoSettings`
    fn pulse_value_in_range(self, value: f32) -> Option<errors::ValueRangeError> {
        let min = self.settings.min as i32;
        let max = self.settings.max as i32;

        if value < min as f32 || value > max as f32 {
            return Some(errors::ValueRangeError::new(
                errors::Value::Int(min),
                errors::Value::Int(max),
                errors::Value::Float(value),
            ));
        }

        None
    }

    /// Returns the minimum value that is allowable for this `ServoChannel`.
    pub fn minimum_value(self) -> u16 {
        return self.settings.min;
    }

    /// Returns the maximum value that is allowable for this `ServoChannel`.
    pub fn maximum_value(self) -> u16 {
        return self.settings.max;
    }

    /// Given a pulse time (µs), calculate the angle in degrees that the servo
    /// should be moved to. The angle space this is normalized to is (-90...90).
    /// 
    /// Based on Pimoroni's [pantilthat.pantilt module](https://github.com/pimoroni/pantilt-hat/blob/master/library/pantilthat/pantilt.py#L139)
    pub fn pulse_time_to_degrees(self, pulse: u16) -> Result<f32, errors::ValueRangeError> {
        let _ = env_logger::try_init();

        let valid = self.pulse_value_in_range(pulse as f32);
        if valid.is_some() {
            return Err(valid.unwrap());
        }

        debug!("pulse value {} is valid", pulse);

        let (min, max) = self.settings.servo_range();
        debug!("servo range is {} -> {}", min, max);

        let servo_range = (max - min) as f32;
        debug!("servo differential is {}", servo_range);

        let pulse_diff = (pulse - min) as f32;
        debug!("pulse differential is {}", pulse_diff);

        let angle: f32 = pulse_diff / servo_range;
        debug!("prescaled angle is {:.2}", angle);

        let angle: f32 = angle * 180.0;
        debug!("scaled angle is {:.2}", angle);

        let angle: f32 = angle.round();
        debug!("rounded angle is {:.2}", angle);

        let angle: f32 = angle - 90.0;
        debug!("normalized angle is {:.2}", angle);

        Ok(angle)
    }

    /// Given an angle, calculate the pulse time in µs that the servo
    /// should be moved to. Expects the angle to be normalized on (-90...90)
    /// 
    /// Based on Pimoroni's [pantilthat.pantilt module](https://github.com/pimoroni/pantilt-hat/blob/master/library/pantilthat/pantilt.py#L139)
    pub fn degrees_to_pulse_time(self, angle: f32) -> Result<u16, errors::ValueRangeError> {
        if angle < -90.0 || angle > 90.0 {
            return Err(errors::ValueRangeError::new(
                errors::Value::Int(-90),
                errors::Value::Int(90),
                errors::Value::Float(angle),
            ));
        }

        debug!("angle value {:.2} is valid", angle);

        let (min, max) = self.settings.servo_range();
        debug!("servo range is {} -> {}", min, max);

        let servo_range = (max - min) as f32;
        debug!("servo differential is {:.2}", servo_range);

        let angle: f32 = angle + 90.0;
        debug!("denormalized angle: {:.2}", angle);

        let scale: f32 = servo_range / 180.0;
        debug!("range scale is: {:.2}", scale);

        let scaled: f32 = (angle as f32) * scale;
        debug!("scaled angle to servo differential: {:.2}", scaled);

        let pulse: u16 = min + (scaled as u16);
        debug!("finalized pulse: {}", pulse);

        Ok(pulse)
    }

}

impl Channel for ServoChannel {

    /// Returns the channel index for this `ServoChannel`
    fn channel_num(&self) -> u8 {
        return self.channel_num;
    }

}

impl fmt::Display for ServoChannel {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ServoChannel<ON_L: {:#x}, ON_H: {:#x}, OFF_L: {:#x}, OFF_H: {:#x}>",
            self.on_low(),
            self.on_high(),
            self.off_low(),
            self.off_high(),
        )
    }

}
