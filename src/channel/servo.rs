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
    fn pulse_value_in_range(self, value: i32) -> Option<errors::ValueRangeError> {
        let min = self.settings.min as i32;
        let max = self.settings.max as i32;

        if value < min || value > max {
            return Some(errors::ValueRangeError::new(
                min,
                max,
                value,
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
    /// should be moved to.
    /// 
    /// Based on Pimoroni's [pantilthat.pantilt module](https://github.com/pimoroni/pantilt-hat/blob/master/library/pantilthat/pantilt.py#L139)
    pub fn pulse_time_to_degrees(self, pulse: u16) -> Result<i32, errors::ValueRangeError> {
        let _ = env_logger::try_init();

        let valid = self.pulse_value_in_range(pulse as i32);
        if valid.is_some() {
            return Err(valid.unwrap());
        }

        debug!("pulse value {} is valid", pulse);

        let (min, max) = self.settings.servo_range();
        let range = (max - min) as f32;
        debug!("servo range is {} -> {}", min, max);
        debug!("value space is {}", range);

        let distance: u16 = pulse - min;
        debug!("pulse distance from min: {}", distance);

        let scale: f32 = distance as f32 / range;
        debug!("pulse distance scaled to range: {}", scale);

        let scaled: f32 = scale * 180.0;
        debug!("scaled to degrees: {}", scaled);

        let angle = distance as f32 / scaled;
        debug!("angle before finalize: {}", angle);

        let angle = angle.round() - 90.0;
        debug!("angle before trunc to i32: {}", angle);

        let angle = angle as i32;
        debug!("finalized angle: {}", angle);

        Ok(angle)
    }

    /// Given an angle, calculate the pulse time in µs that the servo
    /// should be moved to.
    /// 
    /// Based on Pimoroni's [pantilthat.pantilt module](https://github.com/pimoroni/pantilt-hat/blob/master/library/pantilthat/pantilt.py#L139)
    pub fn degrees_to_pulse_time(self, angle: i32) -> Result<u16, errors::ValueRangeError> {
        if angle < -90 || angle > 90 {
            return Err(errors::ValueRangeError::new(-90, 90, angle));
        }

        debug!("angle value {} is valid", angle);

        let (min, max) = self.settings.servo_range();
        let range = (max - min) as f32;
        debug!("servo range is {} -> {}", min, max);
        debug!("value space is {}", range);

        let angle = angle + 90;
        debug!("normalized angle: {}", angle);

        let scale: f32 = range / 180.0;
        debug!("servo range scaled to angular space: {}", scale);

        let scaled: f32 = scale * angle as f32;
        debug!("angle scaled by pulse width: {}", scaled);

        let pulse = min as f32 + scaled;
        debug!("finalized pulse: {}", pulse);

        Ok(pulse as u16)
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
