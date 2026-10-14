fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello: {}, world: {}", hello, world);

    let mut s = String::from("hello world");
    let word_index = first_word_index(&s);
    s.clear();
    println!("The first word index is: {}", word_index);

    s = String::from("hello world");
    let word_slice = first_word(&s); // word_slice is a immutable slice of the string `s`
    // s.clear(); // Cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("The first word is: {}", word_slice); // This will print: The first word is: hello

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // slice contains [2, 3]
    println!("Slice of array: [{}, {}]", slice[0], slice[1]);
}

// Use &str as a parameter type for functions that take string slices, 
// allowing them to accept both String and &str types. 
// This is more flexible than using &String, which only accepts references to String values.
fn first_word_index(s: &str) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i; // Return the index of the first space
        }
    }
    s.len() // Return the length of the string if no space is found
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // Return a slice from the start of the string to the index of the space
        }
    }
    &s[..] // Return the whole string if no space is found
}
