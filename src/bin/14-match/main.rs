#![allow(dead_code, unused_variables)]

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("Value in cents: {}", value);

    let coin = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(coin);
    println!("Value in cents: {}", value);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Six: {:?}", six);
    println!("None: {:?}", none);

    let v = 0u8;
    match v {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // The underscore `_` is a catch-all pattern that matches any value.
    }
}
