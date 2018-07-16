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
        servo::{ ServoChannel, ServoSettings },
    },
};

#[test]
fn test_srvchan_new_over_max() {
    let result = ServoChannel::new(16);
    match result {
        Ok(_) => panic!("expected error, received ok"),
        Err(_) => return,
    }
}

#[test]
fn test_srvchan_new_with_settings() {
    let _ = env_logger::try_init();

    let channel = ServoChannel::new_with_settings(1, ServoSettings::new(510, 2300)).unwrap();

    assert_eq!(510, channel.minimum_value());
    assert_eq!(2300, channel.maximum_value());
}

#[test]
fn test_srvchan_read_channel_bytes() {
    let _ = env_logger::try_init();

    let mut device = MockI2CDevice::new();
    let channel = ServoChannel::new(0).unwrap();
    let channel_values: [u8; 4] = [0xfe, 0xed, 0xb3, 0x3f];

    // Write `channel_values` to the mock device's register map
    device.regmap.write_regs(channel.base_address() as usize, &channel_values);

    // Retrieve the written bytes from the channel's registers
    let stored = channel.read_channel(&mut device).unwrap();

    assert_eq!(channel_values, stored);
}

#[test]
fn test_srvchan_write_channel_bytes() {
    let _ = env_logger::try_init();

    let mut device = MockI2CDevice::new();
    let channel = ServoChannel::new(1).unwrap();
    let channel_values: [u8; 4] = [0xca, 0xfe, 0xba, 0xbe];

    // Write `channel_values` to the mock device using `write_channel`
    channel.write_channel(&mut device, channel_values).unwrap();

    // Retrieve the written bytes from the channel's registers
    let stored = channel.read_channel(&mut device).unwrap();

    assert_eq!(channel_values, stored);
}

#[test]
fn test_srvchan_degree_to_pulse_conversion() {
    let _ = env_logger::try_init();

    let channel = ServoChannel::new(1).unwrap();

    let init = -77.0; // degrees
    debug!("testing degree -> pulse time conv with init={}°", init);

    let pulse = channel.degrees_to_pulse_time(init).unwrap();
    debug!("init={}° -> pulse time {} µs", init, pulse);

    let degrees = channel.pulse_time_to_degrees(pulse).unwrap();
    debug!("pulse time {} µs -> degrees {}°", pulse, degrees);

    assert_eq!(init, degrees);
}

#[test]
fn test_srvchan_pulse_to_degree_conversion() {
    let _ = env_logger::try_init();

    let channel = ServoChannel::new(1).unwrap();
    let servo_min = channel.minimum_value();
    let servo_max = channel.maximum_value();
    let servo_error = (servo_max - servo_min) as f32 / 180.0;

    let init: u16 = 700; // µs
    debug!("testing pulse time -> degree conv with init={} µs", init);

    let degrees: f32 = channel.pulse_time_to_degrees(init).unwrap();
    debug!("init={} µs -> degrees: {}°", init, degrees);

    let pulse: u16 = channel.degrees_to_pulse_time(degrees).unwrap();
    debug!("degrees={}° -> pulse time: {} µs", degrees, pulse);

    let lower_bound = (init as f32) - servo_error;
    let upper_bound = (init as f32) + servo_error;
    let pulse: f32 = pulse as f32;

    assert!(lower_bound <= pulse && upper_bound >= pulse);
}

// #[test]
// fn test_srvchan_write_value_degrees() {
//     let _ = env_logger::try_init();

//     let mut device = MockI2CDevice::new();
//     let channel = ServoChannel::new_with_settings(1, ServoSettings::new(510, 2300)).unwrap();

//     // let angle
// }