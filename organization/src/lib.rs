// all structs, enums, functions, and methods are private by default
// use `pub` to make them public

/* 
module system:
- packages - a Cargo feater that lets you build, test, and share crates
- crates - a tree of modules that produces a library or executable
- modules & use - lets you control the organization, scope, and privacy of paths
- paths - a way of naming an item such as a stuct, function, or module
*/

mod front_of_house;
// and its files have the same implmentation in one file here
/* mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
} */

mod back_of_house {
    // making an enum public = all of its variants will be public
    pub enum Appetizer {
        Soup,
        Salad,
    }
    // for structs, need to explicitly make each field public
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
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

// bring paths into scope
// idomatic use path - including the parent module to make
// clear that the function isn't locally defined
use crate::front_of_house::hosting as host; // as keyword

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    front_of_house::hosting::add_to_waitlist();

    // order breakfast with rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change mind about the bread
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // next line won't compile, because seasonal_fruit is private
    //meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    
    host::add_to_waitlist();
}
