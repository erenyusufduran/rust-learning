fn main() {
    let mut s = String::from("hello");
    // The double colon :: operator allows us to namespace this particular from function under the String type rather than using some sort of name like string_from.

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    /*  if s hardcoded with (let s = "hello") --> then compile time would decrease, when we give
    mut to variables, compiling time will increase because mutuable variable's size might change
    while running the program.    */

    let s1 = String::from("hello");
    let s2 = s1;
    // To ensure memory safety, after the line let s2 = s1, Rust considers s1 as no longer valid.

    let st1 = String::from("hello");
    let (st2, len) = calculate_length(st1);
    // when calculate_length called, st1 removed. So if we want to use st1, we can return with tuples.
    println!("The length of '{}' is {}.", st2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
