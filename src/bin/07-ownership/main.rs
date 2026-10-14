fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    // println!("{}", s1); // This line would cause a compile-time error because s1 is no longer valid

    let s3 = s2.clone(); // s2 is cloned to s3, both are valid
    println!("s2: {}, s3: {}", s2, s3); // This will print: s2: hello, s3: hello

    let s = String::from("world");
    takes_ownership(s); // s is moved into the function, and is no longer valid
    // println!("{}", s); // This line would cause a compile-time error because s is no longer valid
    let x = 5;
    makes_copy(x); // x is copied into the function, and is still valid
    println!("x: {}", x); // This will print: x: 5

    let s1 = gives_ownership(); // s1 takes ownership of the returned String
    println!("s1: {}", s1); // This will print: s1: hello

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 is moved into the function, and s3 takes ownership of the returned String
    // println!("{}", s2); // This line would cause a compile-time error because s2 is no longer valid
    println!("s3: {}", s3); // This will print: s3: hello

}

fn takes_ownership(some_string: String) {
    println!("Taking ownership of: {}", some_string);
} // some_string goes out of scope and is dropped here

fn makes_copy(some_integer: i32) {
    println!("Making a copy of: {}", some_integer);
} // some_integer goes out of scope, but nothing special happens because it's a copy

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // some_string is returned and moves out to the caller
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string is returned and moves out to the caller
}
