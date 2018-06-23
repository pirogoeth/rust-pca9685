extern crate i2cdev;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rust_pca9685;

use i2cdev::core::I2CDevice;
use i2cdev::mock::MockI2CDevice;

use rust_pca9685::{
    constants,
    controller::{
        calculate_prescale_value,
        Controller,
    },
    led_channel::LEDChannel,
};

#[test]
fn test_calculate_prescale_values() {
    assert_eq!(0x1e, calculate_prescale_value(200f32));
}

#[test]
fn test_controller_init() {
    let _ = env_logger::try_init();

    let mut device = MockI2CDevice::new();

    // Write the default values to `MODE_*` registers
    device.smbus_write_byte_data(constants::MODE_1, 0x11).unwrap();
    device.smbus_write_byte_data(constants::MODE_2, 0x04).unwrap();

    // Run the controller initialization step
    Controller::new(&mut device);

    let mode1 = device.smbus_read_byte_data(constants::MODE_1).unwrap();
    let mode2 = device.smbus_read_byte_data(constants::MODE_2).unwrap();

    assert_eq!(0x01, mode1);
    assert_eq!(0x04, mode2);
}

#[test]
fn test_controller_set_pwm_rate() {
    let _ = env_logger::try_init();

    let prescale_value = calculate_prescale_value(60f32);

    let mut device = MockI2CDevice::new();

    {
        let mut ctrl = Controller::new(&mut device);
        ctrl.set_pwm_rate(prescale_value).unwrap();
    }

    let mode1 = device.smbus_read_byte_data(constants::MODE_1).unwrap();
    let prescale_reg = device.smbus_read_byte_data(constants::PRE_SCALE).unwrap();
    assert_eq!(constants::ALL_CALL | constants::RESTART, mode1);
    assert_eq!(prescale_value, prescale_reg);
}

#[test]
/// This test references values used in the [PCA9685 datasheet](https://cdn-shop.adafruit.com/datasheets/PCA9685.pdf).MockI2CDevice
/// Specifically, values from Example 1 in Section 7.3.3 are used as a base case.
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