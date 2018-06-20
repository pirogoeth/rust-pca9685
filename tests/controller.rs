extern crate i2cdev;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rust_pca9685;

use i2cdev::mock::MockI2CDevice;

use rust_pca9685::{
    controller::Controller,
    led_channel::LEDChannel,
};


#[test]
// This test references values used in the [PCA9685 datasheet](https://cdn-shop.adafruit.com/datasheets/PCA9685.pdf).MockI2CDevice
// Specifically, values from Example 1 in Section 7.3.3 are used as a base case.
fn test_write_value_to_channel() {
    let _ = env_logger::try_init();

    let (on, off) = (0x199u16, 0x4ccu16);
    let expected: [u8; 4] = [
        (on & 0xff) as u8,
        (on >> 8) as u8,
        (off & 0xff) as u8,
        (off >> 8) as u8,
    ];

    assert_eq!(expected[0], 0x99u8);  // ON_L
    assert_eq!(expected[1], 0x1u8);  // ON_H
    assert_eq!(expected[2], 0xccu8);  // OFF_L
    assert_eq!(expected[3], 0x4u8);  // OFF_H

    let mut device = MockI2CDevice::new();

    {
        let mut ctrl = Controller::new(&mut device);
        ctrl.set_channel(0, on, off).unwrap();
    }

    let channel = LEDChannel::new(0).unwrap();
    let actual = channel.read_channel(&mut device).unwrap();

    assert_eq!(expected, actual);
}