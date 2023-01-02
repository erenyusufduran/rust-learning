// cargo new restaurant_modules --lib

#[allow(dead_code)]
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {}

#[allow(dead_code)]
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
    // Enums aren’t very useful unless their variants are public;
    // it would be annoying to have to annotate all enum variants with pub in every case, so the default for enum variants is to be public.
    // Structs are often useful without their fields being public, so struct fields follow the general rule of everything being private by default unless annotated with pub.
    pub struct Breakfast {
        pub toast: String,      // toast public
        seasonal_fruit: String, // private
    } // So user can reach Breakfast struct and toast, but chef will write seasonal_fruit.

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

#[allow(unused_variables)]
pub fn eat_at_restaurant() {
    // // Absolute Path
    // crate::front_of_house::hosting::add_to_waitlist();

    // // Relative Path
    // front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please!", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
