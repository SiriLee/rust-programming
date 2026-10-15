#![allow(dead_code, unused_variables)]

fn main() {
    // Option is a type that represents a value that may or may not be present.
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap(); // unwrap() is used to get the value inside the Option.
    println!("Sum: {}", sum);
}
