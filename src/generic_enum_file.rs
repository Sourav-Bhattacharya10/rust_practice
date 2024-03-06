// Manually implementing Display trait for enum
use std::{any::type_name, fmt};

pub enum Options<T> {
    Some(T),
    None
}

impl<T: fmt::Display> fmt::Display for Options<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Options::Some(x) => write!(fmt, "Type: {} and Value: {}", type_name::<T>(), x),
            Options::None => write!(fmt, "None")
        }
    }
}