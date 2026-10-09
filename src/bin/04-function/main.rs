fn plus_five(x: i32) -> i32 {
    x + 5 // return the last expression. if "x + 5;" return ()
}

fn main() {
    another_function(5, 6); // argument
    let x = plus_five(6);
    println!("The value of x is: {}", x);
}

fn another_function(x: i32, y: i32) {
    // parameter
    println!("the value of x is: {}", x);
    println!("the value of y is: {}", y);
} 
