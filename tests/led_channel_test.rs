extern crate i2cdev;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rust_pca9685;

use i2cdev::core::I2CDevice;
use i2cdev::mock::MockI2CDevice;

#[allow(unused_imports)]
use rust_pca9685::{
    constants,
    controller::{
        calculate_prescale_value,
        Controller,
    },
    channel::{
        base::Channel,
        led::LedChannel,
    },
};

#[test]
fn test_ledchan_new_over_max() {
    let result = LedChannel::new(16);
    match result {
        Ok(_) => panic!("expected error, received ok"),
        Err(_) => return,
    }
}

#[test]
fn test_ledchan_read_channel_bytes() {
    let _ = env_logger::try_init();

    let mut device = MockI2CDevice::new();
    let channel = LedChannel::new(0).unwrap();
    let channel_values: [u8; 4] = [0xfe, 0xed, 0xb3, 0x3f];

    // Write `channel_values` to the mock device's register map
    device.regmap.write_regs(channel.base_address() as usize, &channel_values);

    // Retrieve the written bytes from the channel's registers
    let stored = channel.read_channel(&mut device).unwrap();

    assert_eq!(channel_values, stored);
}

#[test]
fn test_ledchan_write_channel_bytes() {
    let _ = env_logger::try_init();

    let mut device = MockI2CDevice::new();
    let channel = LedChannel::new(1).unwrap();
    let channel_values: [u8; 4] = [0xca, 0xfe, 0xba, 0xbe];

    // Write `channel_values` to the mock device using `write_channel`
    channel.write_channel(&mut device, channel_values).unwrap();

    // Retrieve the written bytes from the channel's registers
    let stored = channel.read_channel(&mut device).unwrap();

    assert_eq!(channel_values, stored);
}
