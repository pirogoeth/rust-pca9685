use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ChannelRangeError;

impl error::Error for ChannelRangeError {
    fn description(&self) -> &str {
        "channel number out of range (0..15)"
    }
}

impl fmt::Display for ChannelRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "channel number out of range (0..15)")
    }
}