#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/*
    All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl.
        We can define associated functions that don’t have self as their first parameter (and thus are not methods),
            because they don’t need an instance of the type to work with.
*/
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        //* methods with parameters *//
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
// We can split the impl blocks.
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
} // There is no reason to seperate this methods, but we'll see soon.

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    if rect1.width() {
        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    }

    let sq = Rectangle::square(30);
    dbg!(sq);
}
