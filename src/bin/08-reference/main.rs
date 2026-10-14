fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Pass a reference to s1, so ownership is not moved
    println!("The length of '{}' is {}.", s1, len); // This will print: The length of 'hello' is 5.

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("s2 after change: {}", s2); // This will print: s2 after change: hello, world

    // Summary: Only one mutable reference or any number of immutable references are allowed at a time, 
    // but not both. This prevents data races at compile time.
    let mut s = String::from("hello");
    let s1: &mut String = &mut s; // Mutable reference to s
    println!("s1: {}", s1);

    // let s2 = &mut s; 
    // println!("s1: {}, s2: {}", s1, s2); 
    // Cannot borrow `s` as mutable more than once at a time
    
    // let s2 = &s;
    // println!("s1: {}, s2: {}", s1, s2);
    // Cannot borrow `s` as immutable because it is already borrowed as mutable
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
