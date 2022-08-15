fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("Hello");
    println!("'{s}'");
    change(&mut s); // we cannot borrow s as mutable more than once at a time.
    println!("'{s}'");

    /*
    let mut s2 = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    /let r3 = &mut s; // BIG PROBLEM -->  This will going to error, Even we have immutable references, cannot have mutuble reference

    println!("{}, {}, and", r1, r2 /*r3*/);
    */

    let nonreference_from_dangle = dangle();
    println!("{nonreference_from_dangle}");
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.
fn change(s: &mut String) {
    s.push_str(", world!");
}

fn dangle() -> String {
    let s = String::from("hello there");
    s // if we return a reference, then program will give an error, because Ownership is moved out, and nothing is deallocated.
}
