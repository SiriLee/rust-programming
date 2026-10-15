fn main() {
    let v = Some(3u8);

    /*
    match v {
        Some(3) => println!("three"),
        _ => println!("not three"),
    }
     */

    if let Some(3) = v {
        println!("three");
    } else {
        println!("not three");
    }
}
