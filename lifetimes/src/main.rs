// Lifetime annotations don’t change how long any of the references live. Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.

/* 
fn main() {
    /* 
    // fails because r lives longer than x
    let r;

    // x after this scope is deallocated and thus after this scope, r does not reference to anything
    {
        let x = 5;
        r = &x;
    }

    // r is now referencing to something that has already been deallocated
    println!("r: {}", r);
    */

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
*/

// valid because string1 lives for the whole duration of the result
// while strng2 is in the same scope as the function call
// meaning the shortest of the two lifetimes is still valid for the function call

// fn main() {
//     let string1 = String::from("long string is long");
//     {
//         let string2 = String::from("xyz");
//         let result = longest(string1.as_str(), string2.as_str());
//         println!("The longest string is {}", result);
//     }
// }


// fails because string2 does not last as long as the variable the return of the function call is assgined to
// results scope is longer than that of string2

// fn main() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {}", result);
// }

// add the 'a syntax to specify the lifetimes of the parameters and the return type
               // input lifetimes         // output lifetimes
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // since it cannot tell if x or y is returned
    // and the compiler is not sure if either x or y can last until the duration of the return of the function
    // that is why it is not sure if either x or y can last the whole lifetime

    // lifetimes will always consider the smaller of the parameters passed
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


// in this case y does not need to be a lifeitme parameter since it does not affect the return value of the function
/*
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
*/

/* 
// does not compile because the scope of the result variable is only within this function
// therefore we cannot return a reference of a variable that is only available within this scope
// furthermore, result is not related to any of the lifetime parameters passed to the function
fn longest<'a>(x: &str, y: &str) -> &'a str {
    // creates a dangling reference
    let result = String::from("really long string");

    // better to return an owned value rather than a reference
    // so that the caller will be the new owner rather than only having the reference
    result.as_str()
}
*/

/* 
// in order for structs to hold references, we need to add lifetime annotations to all of those references
// instances of ImportantExcerpt must not outlive the references within it
// if it did, that would mean it has a dangling reference
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
*/

/* 

Lifetime Rules

Input Rules
1. the compiler assigns a lifetime parameter to each parameter that’s a reference.

Output Rules
2. if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters

3. if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

*/

/*
let s: &'static str = "I have a static lifetime.";

static lifetimes can be used to make variables live for the whole duration of the program
this is done by allociting it to the heep, this makes it last the whole program
be careful and think if you want this particular variable to last the whole program
you might find that you need to fix the reference and ownership rather than making it available for whole duration
see Box() boxed method and released for similar effects

*/

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}