fn main() {
    let number = 5;

    if number % 4 == 0 {
        println!("Divisible by 4");
    } else if number % 3 == 0 {
        println!("Divisible by 3");
    } else if number % 2 == 0 {
        println!("Divisible by 2");
    } else {
        println!("Number can't be divisible by 2,3 or 4.")
    }

    // using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        };
    };
    println!("The result is: {result}");

    println!("-----------------------------------");

    let mut count = 0;
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    println!("-----------------------------------");

    let mut number_ = 3;
    while number_ != 0 {
        println!("{number_}");
        number_ -= 1;
    }
    println!("LIFTOFF!!!");

    println!("-----------------------------------");
    // NOT REQUIRED WAY..
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", arr[index]);
        index += 1;
    }
    // BEST PRACTICE IS
    for element in arr {
        println!("The value is: {element}");
    }

    println!("-----------------------------------");

    for numberr in (1..4).rev() {
        println!("{numberr}!");
    }
    println!("LISTOFF!!!");
}
