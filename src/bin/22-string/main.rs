#![allow(dead_code, unused_variables, unused_mut)]

fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is {}", s);

    let mut s = String::from("foo");
    let s1 = String::from("bar");
    s.push_str(&s1); // deref coercion: &String to &str
    println!("s is {}", s);

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // fn add(mut self, other: &str) -> String
    // s1 is moved to self and can no longer be used
    // s2 is borrowed as &str(deref coercion) and can still be used
    println!("s3 is {}", s3);
    // s1: moved, s2: borrowed, s3: new String

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3; // s1 is moved, s2 and s3 are borrowed
    let s = format!("{}-{}-{}", s1, s2, s3); // format! macro does not take ownership of s1, s2, s3
    println!("s is {}", s);
}
