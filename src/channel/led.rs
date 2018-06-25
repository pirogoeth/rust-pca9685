use std::fmt;

use ::errors;
use ::channel::base::Channel;

#[derive(Clone, Copy, Debug)]
pub struct LedChannel {
    channel_num: u8,
}

impl LedChannel {

    /// Creates a `LedChannel` at the specified channel register.
    pub fn new(channel_num: u8) -> Result<LedChannel, errors::ChannelRangeError> {
        if channel_num > 15 {
            return Err(errors::ChannelRangeError);
        }

        Ok(
            LedChannel{
                channel_num: channel_num,
            }
        )
    }

}

impl Channel for LedChannel {

    /// Returns the channel index for this `ServoChannel`
    fn channel_num(&self) -> u8 {
        return self.channel_num;
    }

}

impl fmt::Display for LedChannel {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "LedChannel<ON_L: {:#x}, ON_H: {:#x}, OFF_L: {:#x}, OFF_H: {:#x}>",
            self.on_low(),
            self.on_high(),
            self.off_low(),
            self.off_high(),
        )
    }

}

