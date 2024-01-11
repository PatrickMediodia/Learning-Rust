// To learn more about modules, see the link below
// https://www.youtube.com/watch?v=969j0qnJGi8&ab_channel=Let%27sGetRusty

// define the front_of_house module here but get contents from a different file but same contents in the module
// we can define the contents of the parent module, create an .rs file with the same name
mod front_of_house;

/* 
pub fn eat_at_restaurant() {
    // paths are separated my double colons

    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}
*/

fn serve_order() {}

// modules can have other modules inside of them
// also structs, enums, traits, etc.

// by default, a child module and everything inside of it is private from the perspective of the module
// child modules can see everything in their parent modules
// hide implementation details by default
mod back_of_house {
    /* 
    fn fix_incorrect_order() {
        cook_order();
        // super allows you to reference the parent module
        // in this case, back_of_house is a child to the parent module lib-crate
        super::serve_order();
    }

    fn cook_order() {}
    */

    // even if our struct is now public, its fields are still private
    pub struct Breakfast {
        // need to explicitly say toast is public
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

    // enums are private by default
    // but the variants inside them are public by default
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// absolute path
// use the pub keyword to make external code reference hosting
pub use crate::front_of_house::hosting;
// relative path
use self::front_of_house::hosting::add_to_waitlist;

// use {} to import many modules within a package
use rand::{Rng, CryptoRng};

use std::io::{self, Write};

// glob operator
use std::io::*;

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    front_of_house::hosting::add_to_waitlist();

    // the use keyword is useful to not require specifying the full path everytime
    // not that readable since you would think it is a local path
    add_to_waitlist();

    // shows that the add_to_waitlist method is from another module
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // use the full path if bringing structs, enums, etc. into scope

    let rand_num = rand::thread_rng().gen_range(1..101);
}

use std::fmt;
//use std::io;

fn function1() -> fmt::Result {
    Ok(())
}
fn function2() -> io::Result<()>{
    Ok(())
}