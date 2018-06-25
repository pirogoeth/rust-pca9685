//! Provides abstractions for a channel on the PCA9685 controller.
//! Shared functionality is represented in the `Channel` trait,
//! while specialized behaviour is implemented in the individual
//! `ServoChannel` and `LedChannel` impls.

pub mod base;
pub mod led;
pub mod servo;
