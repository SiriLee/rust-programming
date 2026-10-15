#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Area: {}", area(&rect));

    println!("{:?}", rect); // Debug trait allows us to print the struct using {:?}
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
