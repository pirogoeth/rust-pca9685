use std::error;
use std::fmt;

#[derive(Clone, Debug)]
pub struct IndexRangeError;

impl IndexRangeError {

    pub fn new() -> IndexRangeError {
        IndexRangeError{ }
    }

}

impl error::Error for IndexRangeError { }

impl fmt::Display for IndexRangeError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "channel index out of range (0..15)")
    }

}

#[derive(Clone, Debug)]
pub struct ValueRangeError {
    min: i32,
    max: i32,
    val: i32,
}

impl ValueRangeError {

    pub fn new(min: i32, max: i32, val: i32) -> ValueRangeError {
        ValueRangeError{
            min: min,
            max: max,
            val: val,
        }
    }

}

impl error::Error for ValueRangeError { }

impl fmt::Display for ValueRangeError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "value {} out of range ({}..{})", self.val, self.min, self.max)
    }

}