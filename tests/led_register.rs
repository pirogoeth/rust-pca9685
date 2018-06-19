extern crate i2cdev;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rust_pca9685;

use i2cdev::mock::MockI2CDevice;
use std::fmt;

use rust_pca9685::{
    constants::{
        BASE_LED_ON_LOW,
        BASE_LED_ON_HIGH,
        BASE_LED_OFF_LOW,
        BASE_LED_OFF_HIGH,
    },
    led_register::{ LEDRegister },
};

#[derive(Clone, Copy, Debug)]
struct ChannelRegisterQuad(u8, u8, u8, u8);
impl ChannelRegisterQuad {

    pub fn on_addrs(&self) -> (u8, u8) {
        (self.0, self.1)
    }

    pub fn off_addrs(&self) -> (u8, u8) {
        (self.2, self.3)
    }

}

impl fmt::Display for ChannelRegisterQuad {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "( ON_L: {:#x}, ON_H: {:#x}, OFF_L: {:#x}, OFF_H: {:#x} )",
            self.0,
            self.1,
            self.2,
            self.3,
        )
    }

}

const CHANNEL_REGISTERS: [ChannelRegisterQuad; 16] = [
    ChannelRegisterQuad(0x06, 0x07, 0x08, 0x09),  // chan 0
    ChannelRegisterQuad(0x0a, 0x0b, 0x0c, 0x0d),  // chan 1
    ChannelRegisterQuad(0x0e, 0x0f, 0x10, 0x11),  // chan 2
    ChannelRegisterQuad(0x12, 0x13, 0x14, 0x15),  // chan 3
    ChannelRegisterQuad(0x16, 0x17, 0x18, 0x19),  // chan 4
    ChannelRegisterQuad(0x1a, 0x1b, 0x1c, 0x1d),  // chan 5
    ChannelRegisterQuad(0x1e, 0x1f, 0x20, 0x21),  // chan 6
    ChannelRegisterQuad(0x22, 0x23, 0x24, 0x25),  // chan 7
    ChannelRegisterQuad(0x26, 0x27, 0x28, 0x29),  // chan 8
    ChannelRegisterQuad(0x2a, 0x2b, 0x2c, 0x2d),  // chan 9
    ChannelRegisterQuad(0x2e, 0x2f, 0x30, 0x31),  // chan 10
    ChannelRegisterQuad(0x32, 0x33, 0x34, 0x35),  // chan 11
    ChannelRegisterQuad(0x36, 0x37, 0x38, 0x39),  // chan 12
    ChannelRegisterQuad(0x3a, 0x3b, 0x3c, 0x3d),  // chan 13
    ChannelRegisterQuad(0x3e, 0x3f, 0x40, 0x41),  // chan 14
    ChannelRegisterQuad(0x42, 0x43, 0x44, 0x45),  // chan 15
];

#[test]
fn test_new_lr_over_max() {
    let result = LEDRegister::new(16);
    match result {
        Ok(_) => panic!("expected error, received ok"),
        Err(_) => return,
    }
}

#[test]
fn test_calculated_addrs_for_channel() {
    let _ = env_logger::try_init();

    for i in 0..16 as u8 {
        debug!("calculate register addrs for channel {}", i);

        let result = LEDRegister::new(i).unwrap();
        let expected = ChannelRegisterQuad(
            BASE_LED_ON_LOW + (4 * i),
            BASE_LED_ON_HIGH + (4 * i),
            BASE_LED_OFF_LOW + (4 * i),
            BASE_LED_OFF_HIGH + (4 * i),
        );

        debug!("expected register addrs for channel {} are {}", i, expected);
        debug!("calculated register addrs for channel {} are {}", i, result);

        assert_eq!(expected.on_addrs(), result.on_addrs());
        assert_eq!(expected.off_addrs(), result.off_addrs());
    }
}

#[test]
fn test_calculated_addrs_for_channel_with_regmap() {
    let _ = env_logger::try_init();

    for i in 0..16 {
        let registers = CHANNEL_REGISTERS[i];
        let result = LEDRegister::new(i as u8).unwrap();

        debug!("channel {} should have addresses {}", i, registers);

        assert_eq!(registers.on_addrs(), result.on_addrs());
        assert_eq!(registers.off_addrs(), result.off_addrs());
    }
}

#[test]
fn test_lr_read_channel_bytes() {
    let _ = env_logger::try_init();

    let mut device = MockI2CDevice::new();
    let channel = LEDRegister::new(0).unwrap();
    let channel_values: [u8; 4] = [0xfe, 0xed, 0xb3, 0x3f];

    // Write `channel_values` to the mock device's register map
    device.regmap.write_regs(channel.base_address() as usize, &channel_values);

    // Retrieve the written bytes from the channel's registers
    let stored = channel.read_channel(&mut device).unwrap();

    assert_eq!(channel_values, stored);
}

#[test]
fn test_lr_write_channel_bytes() {
    let _ = env_logger::try_init();

    let mut device = MockI2CDevice::new();
    let channel = LEDRegister::new(1).unwrap();
    let channel_values: [u8; 4] = [0xca, 0xfe, 0xba, 0xbe];

    // Write `channel_values` to the mock device using `write_channel`
    channel.write_channel(&mut device, channel_values).unwrap();

    // Retrieve the written bytes from the channel's registers
    let stored = channel.read_channel(&mut device).unwrap();

    assert_eq!(channel_values, stored);
}