#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let ins = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}", ins);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Five: {:?}, Six: {:?}, None: {:?} <---", five, six, none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // or other => erelkdflsakd(other) // _ =>
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}

fn value_in_cents(coin: Coin) -> u8 {
    //* With if, the expression needs to return a Boolean value, but here, it can return any type.
    //* The type of coin in this example is the Coin enum that we defined on the first line.
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State querter from {:?}!", state);
            25
        }
    }
}

// matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
