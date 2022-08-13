use std::io;

fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = x + 1;
    println!("The value of x is: {x}");
    {
        let x = x * 2; // shadowing the first x, and continue.
        println!("The value of x in the inner scope is: {}", x);
    }

    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60; // constant variables declaring.
    println!("{THREE_HOURS_IN_SECONDS}");

    // We can do this way
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");
    // We can't --> So mut and reassigning is not same.
    // // let mut spaces = "   ";
    // // spaces = spaces.len();

    // DATA TYPES --> Rust is a statically typed language.
    let guess: u32 = "42".parse().expect("Not a number!"); // if we don't add the :u32, rust will display an error.
    println!("{}", guess);

    // Scalar Types
    // i8,u8,16,32,64,128, --> isize, usize depends on the achitecture of the computer. 64bits or 32bits
    // i32 is the default type.

    //let x = 2.0; // f64
    //let y: f32 = 3.0; // f32

    // numericals + - * / %
    // booleans true false

    // variable tup binds to the entire tuple, because a tuple is considered
    //      a single compound element.

    // let tup = (500, 6.4, 1);
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_, y, _) = tup;
    println!("The value of y is: {y}");

    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;

    // array --> accessing the elements a[0], a[1]
    let a = [1, 2, 3, 4, 5, 6];
    let _b: [i32; 5] = [1, 2, 3, 4, 5];
    let _c = [3; 5]; // [3,3,3,3,3]

    // Invalid array element access

    println!("Please enter an array index: ");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number.");
    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
