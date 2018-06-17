use ::{
    constants,
    errors,
};

#[derive(Debug)]
pub struct LEDRegister {
    // Must be between 0 and 15
    channel_num: u8,
}

impl LEDRegister {

    // Creates a LEDRegister at the specified channel register.
    pub fn new(channel_num: u8) -> Result<LEDRegister, errors::ChannelRangeError> {
        if channel_num > 15 {
            return Err(errors::ChannelRangeError);
        }

        Ok(
            LEDRegister{
                channel_num: channel_num,
            }
        )
    }

    pub fn on_addrs(self) -> (u8, u8) {
        let low_base = constants::BASE_LED_ON_LOW;
        let high_base = constants::BASE_LED_ON_HIGH;

        (low_base + self.channel_num, high_base + self.channel_num)
    }


    pub fn off_addrs(self) -> (u8, u8) {
        let low_base = constants::BASE_LED_OFF_LOW;
        let high_base = constants::BASE_LED_OFF_HIGH;

        (low_base + self.channel_num, high_base + self.channel_num)
    }

}