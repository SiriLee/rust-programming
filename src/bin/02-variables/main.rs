const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("Max points: {}", MAX_POINTS);

    let x = 5;
    // x = 6; // error: cannot assign twice to immutable variable `x`
    let x = x + 1; // shadowing
    println!("x = {}", x);

    let spaces = "   ";
    let spaces = spaces.len(); // shadowing
    println!("spaces = {}", spaces);
}