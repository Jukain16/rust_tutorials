mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    pub mod serving {
        fn take_order() {}
        pub fn serve_order() {}
        fn take_payment() {}
    }
}
pub use crate::front_of_house::hosting;
use crate::front_of_house::hosting::add_to_waitlist;
pub fn eat_at_restaurant() {
                            // Absolute path
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("Rye"); // Relative path
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast)
} 


mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
}
