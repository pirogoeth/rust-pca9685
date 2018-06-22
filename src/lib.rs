//! Provides a simple API for interacting with the Adafruit PCA9685.

#![deny(missing_debug_implementations)]

extern crate i2cdev;

#[macro_use]
extern crate log;
extern crate env_logger;

pub mod constants;
pub mod controller;
pub mod errors;
pub mod led_channel;
#[cfg(target_os = "linux")]
pub mod reset;