fn main() {
    let number = 10;
    check_number(number);

    let condition = true;
    // If-else is an expression in Rust, which means it returns a value.
    let result = if condition {
        "Condition is true"
    } else {
        "Condition is false"
    };
    println!("{}", result);
}

fn check_number(num: i32) {
    if num > 0 {
        println!("The number is positive.");
    } else if num < 0 {
        println!("The number is negative.");
    } else {
        println!("The number is zero.");
    }
}
