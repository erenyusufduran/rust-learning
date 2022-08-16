struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(u32, u32, u32);

struct AlwaysEqual; // Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.

fn main() {
    let mut first_user = User {
        email: String::from("erenydurann@gmail.com"),
        username: String::from("eren"),
        active: true,
        sign_in_count: 1,
    };
    first_user.username = String::from("qroxyn"); // the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable
    println!("{}", first_user.username);
    println!("{}", first_user.email);

    let user2 = User {
        email: String::from("newuser@gmail.com"),
        ..first_user //* same effect with taking entire username, active exc. */
                     //* if we use this syntax, we can't use first_user again because we moved rest of the data. */
    };

    let user3 = build_user(String::from("eren@gmail.csd"), String::from("ered"));
    println!("{}", user3.username);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // shorthand like js
        username,
        active: true,
        sign_in_count: 1,
    }
}
