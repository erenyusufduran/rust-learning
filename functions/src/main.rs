fn main() {
    print_labeled_measurements(5, 'h');

    let x = {
        let y = 4;
        y + 1
    };
    println!("The value of x is :  {x}");

    let y = { x + 1 };

    println!("The value of y is: {y}");

    let num = return_five();
    println!("return_five function is returning: {num}");

    let x_plus_y = sum(5, 7);
    println!("X + Y = {}", x_plus_y)
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurements is: {value}{unit_label}");
}

fn return_five() -> i32 {
    let x = 5;
    x
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
