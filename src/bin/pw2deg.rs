#[macro_use] extern crate quicli;
extern crate rust_pca9685;

use quicli::prelude::*;
use rust_pca9685::channel::servo::{ ServoChannel, ServoSettings };

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "servo-min", short = "m", default_value = "0", help = "Servo pulse minimum value")]
    servo_min: u16,
    #[structopt(long = "servo-max", short = "x", default_value = "4095", help = "Servo pulse maximum value")]
    servo_max: u16,
    #[structopt(help = "Pulse length in µs")]
    pulse_length: u16,
    #[structopt(flatten)]
    verbosity: Verbosity,
}

main!(|args: Cli, log_level: verbosity| {
    let channel = ServoChannel::new_with_settings(
        0,
        ServoSettings::new(args.servo_min, args.servo_max),
    ).unwrap();

    let pulse = args.pulse_length;
    let angle = channel.pulse_time_to_degrees(pulse)?;

    println!("pulse {} (µs) -> angle {}°", pulse, angle);
});