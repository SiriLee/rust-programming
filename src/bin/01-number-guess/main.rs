use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);
}
