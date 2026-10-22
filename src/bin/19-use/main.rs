#![allow(dead_code, unused_variables, unused_imports)]

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn some_function() {}
    }
}

// Publicly re-export `hosting` module to make it accessible from outside the crate
pub use crate::front_of_house::hosting; // Absolute path
// use front_of_house::hosting; // Relative path

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

/*
use std::fmt;
use std::io;
fmt::Result;
io::Result;
*/
use std::fmt::Result;
use std::io::Result as IoResult; // Use `as` to rename an import

/*
use std::cmp::Ordering;
use std::io;
use std::io::Write;
*/
use::std::{cmp::Ordering,
    io::{self, Write},
}; // Nested paths to bring multiple items from a module into scope

// use std::collections::*; // Glob operator to bring all public items defined in a path into scope
