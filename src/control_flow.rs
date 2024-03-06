// if else is same as other languages
// no ternary operator in rust
// for ternary, if condition {} else {}

use core::fmt;
use std::any::type_name;

use crate::generic_enum_file::Options;

// match
pub fn match_test<T: fmt::Display>(option: Options<T>){
    match option {
        Options::Some(x) => println!("Got something with type: {} and value: {}", type_name::<T>(), x),
        Options::None => println!("Got none")
    }
}