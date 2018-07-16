extern crate ctrlc;
extern crate i2cdev;
#[macro_use] extern crate quicli;
extern crate rust_pca9685;

use std::num::{ ParseFloatError, ParseIntError };
use std::path::PathBuf;
use std::sync::atomic::{ AtomicBool, Ordering };
use std::sync::Arc;

use i2cdev::core::*;
#[cfg(target_os = "linux")]
use i2cdev::linux::LinuxI2CDevice;
use i2cdev::mock::MockI2CDevice;
use quicli::prelude::*;
use rust_pca9685::{
    channel::servo::{ ServoChannel, ServoSettings },
    constants,
    controller::{
        calculate_prescale_value,
        Controller,
    },
};

fn parse_hex(src: &str) -> std::result::Result<u16, ParseIntError> {
    if src.to_lowercase().starts_with("0x") {
        return u16::from_str_radix(&src[2..], 16);
    }
    u16::from_str_radix(src, 16)
}

fn parse_positive_float(src: &str) -> std::result::Result<f32, ParseFloatError> {
    let val = src.parse::<f32>()?;
    Ok(val.abs())
}

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "i2c-device", short = "d", default_value = "/dev/i2c-0", help = "I2C device node", parse(from_os_str))]
    device: PathBuf,

    #[structopt(long = "i2c-slave-addr", short = "s", help = "I2C slave address (in hex) for PCA9685", parse(try_from_str = "parse_hex"))]
    slave_address: Option<u16>,

    #[structopt(long = "i2c-mock", short = "M", help = "Use the MockI2CDevice instead of LinuxI2CDevice")]
    mock_device: bool,

    #[structopt(long = "pwm-rate", short = "p", default_value = "60", help = "PWM controller oscillation rate")]
    pwm_rate: f32,

    #[structopt(long = "channel", short = "c", default_value = "0", help = "Servo channel to sweep")]
    channel: u8,

    #[structopt(long = "servo-min", short = "m", default_value = "0", help = "Servo pulse minimum value")]
    servo_min: u16,

    #[structopt(long = "servo-max", short = "x", default_value = "4095", help = "Servo pulse maximum value")]
    servo_max: u16,

    #[structopt(long = "step-size", short = "z", default_value = "1.0", help = "Angle step size", parse(try_from_str = "parse_positive_float"))]
    step_size: f32,

    #[structopt(long = "continuous", short = "C", help = "Continuously sweep servo")]
    continuous: bool,

    #[structopt(flatten)]
    verbosity: Verbosity,
}

main!(|args: Cli, log_level: verbosity| {
    let slave_address = args.slave_address.unwrap_or(constants::PCA9685_SLAVE_ADDRESS);
    warn!("using {:#0.2x} as slave address", slave_address);

    if args.mock_device {
        info!("using mock device");

        let mut device = MockI2CDevice::new();
        let mut controller = Controller::new(&mut device);
        run_sweep(args, &mut controller);
    } else {
        warn!("using real i2c device!");

        #[cfg(target_os = "linux")]
        let mut device = LinuxI2CDevice::new(args.device.as_path(), slave_address);
        #[cfg(not(target_os = "linux"))]
        let mut device = MockI2CDevice::new();

        let mut controller = Controller::new(&mut device);
        run_sweep(args, &mut controller);
    }
});

fn run_sweep<'a, T: I2CDevice + 'a>(args: Cli, controller: &mut Controller<T>) {
    // Set up a keyboard interrupt handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Could not set ctrlc handler!");

    let pwm_rate = calculate_prescale_value(args.pwm_rate);
    controller.set_pwm_rate(pwm_rate).unwrap();

    let mut channel = ServoChannel::new_with_settings(
        args.channel,
        ServoSettings::new(args.servo_min, args.servo_max),
    ).unwrap();

    while running.load(Ordering::SeqCst) {
        sweep_from(controller, &mut channel, -90.0, 90.0, args.step_size);
        sweep_from(controller, &mut channel, 90.0, -90.0, -args.step_size);

        if !args.continuous {
            break;
        }
    }

    // Reorient the servo into a neutral position
    let neutral = args.servo_max - ((args.servo_max - args.servo_min) / 2);
    controller.set_channel(&mut channel, 0, neutral).unwrap();
}

fn sweep_from<'a, T: I2CDevice + 'a>(controller: &mut Controller<T>, channel: &mut ServoChannel, start: f32, end: f32, step: f32) {
    let mut position = start;
    while (start < end && position < end) || (start > end && position > end) {
        let pulse = channel.degrees_to_pulse_time(position).unwrap();
        controller.set_channel(channel, 0, pulse).unwrap();

        position = position + step;
    }
}