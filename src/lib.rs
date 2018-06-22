//! Provides a simple API for interacting with the Adafruit PCA9685.
//! This library aims to provide a simple, stable I2C interface to the
//! PCA9685 which supports all of the functionality the
//! [official library](https://github.com/adafruit/Adafruit_Python_PCA9685),
//! but with better documentation, the safety guarantees of Rust, and tests.
//! 
//! This library references the following resources:
//! - [Adafruit Python PCA9685](https://github.com/adafruit/Adafruit_Python_PCA9685)
//! - [PCA9685 Datasheet](https://cdn-shop.adafruit.com/datasheets/PCA9685.pdf)

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