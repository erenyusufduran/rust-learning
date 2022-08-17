enum IpAddrKind {
    //* Because these are the only possibilities for an IP address that our program will come across,
    //*    we can enumerate all possible variants, which is where enumeration gets its name.
    V4, // V4(String)
    V6, // V6(String) --> if we go with this way, we don't need to any struct, we could use it
        // let home = IpAddr::V4(String::from("127.0.0.1"));
        // V4(u8,u8,u8,u8) --> let home = IpAddr::V4(127,0,0,1);
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

//* But if we used the different structs, which each have their own type, we couldn’t as easily define a function to take any of these kinds of messages as we could with the Message */
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32);

impl Message {
    fn call(&self) {
        // method body would be defined.
    }
}

enum Option<T> {
    //* <T> means the Some variant of the Option enum can hold one piece of data of any type,
    //* and that each concrete type that gets used in place of T makes the overall Option<T> type a different type.
    None, // rust doesn't have nulls.
    Some(T),
}

fn main() {
    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    let _new_routing = route(loopback);

    let m = Message::Write(String::from("hello"));
    m.call();

    let _some_number = Some(5);
    let _some_char = Some('e');
    let _absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; --> it's not working, because x's type is i8 and y's Option<i8>.
}

fn route(ip_kind: IpAddr) {}
