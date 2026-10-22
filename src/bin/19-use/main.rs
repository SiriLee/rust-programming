#![allow(dead_code, unused_variables, unused_imports)]

/*
Summary:
- `use` keyword allows bringing paths into scope.
- `use` can be used to bring a module, struct, enum, function, or trait into scope.
- `use` can be used to rename an import with `as`.
*/

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn some_function() {}
    }
}

use crate::front_of_house::hosting;
// use front_of_house::hosting; // Relative path

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

use std::collections::HashMap;

fn main() {
    eat_at_restaurant();
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
