pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 3
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
    //format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    // needed so that we can access the parents methods outside of the tests module scope
    use super::*;

    // #[test]
    // fn exploration() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    // use the assert!() macro when checking boolean results
    // #[test]
    // fn larger_can_hold_smaller() {
    //     let larger = Rectangle {
    //         width: 8,
    //         height: 7,
    //     };
    //     let smaller = Rectangle {
    //         width: 5,
    //         height: 1,
    //     };

    //     assert!(larger.can_hold(&smaller));
    // }

    // #[test]
    // fn smaller_cannot_hold_larger() {
    //     let larger = Rectangle {
    //         width: 8,
    //         height: 7,
    //     };
    //     let smaller = Rectangle {
    //         width: 5,
    //         height: 1,
    //     };

    //     // we need to negate it since we want it to pass if it returns false
    //     // and the assert function proccesses it as passed it it receives true
    //     assert!(!smaller.can_hold(&larger));
    // }

    // #[test]
    // fn it_adds_two() {
    //     assert_eq!(4, add_two(2));
    // }

    // #[test]
    // fn greeting_contains_name() {
    //     let result = greeting("Carol");
    //     /* 
    //     assert!(
    //       expression to evaluate,
    //       custom message to rturn  
    //     );
    //     */

    //     // makes us see the value that was actually returned vs the one that we needed
    //     assert!(
    //         result.contains("Carol"),
    //         "Greeting did not contain name, value was {}",
    //         result,
    //     );
    // }
    
    #[test]
    #[should_panic] // to explicitly say that this is the test function uses the should_panic header
    fn greater_than_100() {
        Guess::new(200);
    }
}

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// introduces a bug to the function
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width < other.width && self.height > other.height
//     }
// }

// assert_eq/assert_ne makes it so that it takes in arguments that need to be equal
// and prints the values back if it is not equal

// the values compared must have the PartialEq and Debug traits
// most primitive types implement these traits
// but you have the implement these yourself for enums and structs
// use the #[derive(PartialEq, Debug)] annotation

// should_panic sample code
// tests for scenarios that the code should panic
// meaning we should pass invalid values for this to work
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}
