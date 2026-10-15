struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

/*
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}
// Missing lifetime specifier for the string slices in the struct definition would lead to a compilation error.
*/

struct Color(i32, i32, i32); // Tuple struct

fn main() {
    let user1 = User {
        username: String::from("Alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("Username: {}", user1.username);

    let mut user2 = User {
        username: String::from("Bob"),
        email: String::from("bob@example.com"),
        sign_in_count: 1,
        active: true,
    };
    user2.email = String::from("bob_new@example.com");
    println!("Updated Email: {}", user2.email);

    let user3 = build_user(String::from("Charlie"), String::from("charlie@example.com"));
    println!("Username: {}, Active: {}", user3.username, user3.active);

    let user4 = User {
        username: String::from("Dave"),
        email: String::from("dave@example.com"),
        ..user1 // This uses the remaining fields from user1
    };
    println!("Username: {}, sign_in_count: {}", user4.username, user4.sign_in_count);

    let black = Color(0, 0, 0);
    println!("Color: ({}, {}, {})", black.0, black.1, black.2);
}

fn build_user(username: String, email: String) -> User {
    User {
        username, // This is shorthand for username: username
        email,
        sign_in_count: 1,
        active: true,
    }
}
