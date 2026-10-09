fn main(){
    let a = 5;
    let b: u128 = 10;
    let c: usize = 15;

    let d: f32 = 2.5;
    let e: f64 = 3.5;

    let f: bool = true;

    println!("a = {}, b = {}, c = {}, d = {}, e = {}, f = {}", a, b, c, d, e, f);

    let g: char = 'A';
    let h: char = '中';
    println!("g = {}, h = {}", g, h);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}, {}, {}", tup.0, tup.1, tup.2);
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}, {}, {}, {}, {}", arr[0], arr[1], arr[2], arr[3], arr[4]);

    let a: [i32; 5] = [3; 5];
    println!("{}, {}, {}, {}, {}", a[0], a[1], a[2], a[3], a[4]);
}
