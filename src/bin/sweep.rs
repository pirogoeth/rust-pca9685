extern crate i2cdev;
#[macro_use] extern crate quicli;
extern crate rust_pca9685;

use i2cdev::core::I2CDevice;
use i2cdev::mock::MockI2CDevice;
use quicli::prelude::*;
use rust_pca9685::{
    channel::servo::{ ServoChannel, ServoSettings },
    controller::{
        calculate_prescale_value,
        Controller,
    },
};

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "pwm-rate", short = "p", default_value = "60", help = "PWM controller oscillation rate")]
    pwm_rate: f32,
    #[structopt(long = "channel", short = "c", default_value = "0", help = "Servo channel to sweep")]
    channel: u8,
    #[structopt(long = "servo-min", short = "m", default_value = "0", help = "Servo pulse minimum value")]
    servo_min: u16,
    #[structopt(long = "servo-max", short = "x", default_value = "4095", help = "Servo pulse maximum value")]
    servo_max: u16,
    #[structopt(long = "continuous", short = "C", help = "Continuously sweep servo")]
    continuous: bool,
    #[structopt(flatten)]
    verbosity: Verbosity,
}

main!(|args: Cli, log_level: verbosity| {
    let mut device = MockI2CDevice::new();
    let pwm_rate = calculate_prescale_value(args.pwm_rate);

    let mut controller = Controller::new(&mut device);
    controller.set_pwm_rate(pwm_rate).unwrap();

    let mut channel = ServoChannel::new_with_settings(
        args.channel,
        ServoSettings::new(args.servo_min, args.servo_max),
    ).unwrap();

    loop {
        sweep_from(&mut controller, &mut channel, -90, 90, 1);
        sweep_from(&mut controller, &mut channel, 90, -90, 1);

        if !args.continuous {
            break;
        }
    }
});

fn sweep_from<'a, T: I2CDevice + 'a>(controller: &mut Controller<T>, channel: &mut ServoChannel, start: i32, end: i32, step: i32) {
    let mut position = start;
    while position < end {
        let pulse = channel.degrees_to_pulse_time(position).unwrap();
        controller.set_channel(channel, 0, pulse).unwrap();

        position = position + step;
    }
}