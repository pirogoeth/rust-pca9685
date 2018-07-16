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
pub enum Value {
    Int(i32),
    Float(f32),
}

impl fmt::Display for Value {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{:.4}", fl),
        }
    }

} 

#[derive(Clone, Debug)]
pub struct ValueRangeError {
    min: Value,
    max: Value,
    val: Value,
}

impl ValueRangeError {

    pub fn new(min: Value, max: Value, val: Value) -> ValueRangeError {
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