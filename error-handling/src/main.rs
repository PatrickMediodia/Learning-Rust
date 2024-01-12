use std::fs::{self, File};
use std::io::ErrorKind; // lets us match on the type of error we get
use std::io::Read; 
use std::io;

fn main() {
    // quits the program and shows a message
    // a backtrace will list all of the function calls leading up to the panic
    // panic!("crash and burn");

    // shows backtracing
    //a();

    // result enum
    // similar to the option enum where it results in some value or none
    // result enum represents sucess and error case
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    /* 
    // open returns a result type
    // it may fail as it might not have the permissions or the file does not exist
    let f = File::open("hello.txt");

    // shadowing f and using match to handle the Result enum
    let f = match f {
        Ok(file) => file, // if file is returned, bind it to f

        //panic!("Problem opening the file: {:?}", error), // if error, throw a panic
        
        Err(error) => match error.kind() { // returns an enum with the kind of error that we got
            
            // creating the file can fail so we need another match statement
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },

            // since match should be exhaustive
            // if error other than not found, panic and return the message
            other_error => panic!("Problem opening the file: {:?}", other_error)
        }
    };
    */

    /* 
    // using closures
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    */

    // we can simplify the match expression by calling unwrapped
    // if success, return it to f
    // if error, panic
    // let f = File::open("hello.txt").unwrap()  
    //let f = File::open("hello.txt").expect("Failed to open hello.txt"); // sets the message to be passed to the panic macro

    // error propagation
    // when an method errors, you usually want to propagate that error back to the caller and then handle it there

}

fn read_username_from_file() -> Result<String, io::Error> {
    // using the ? operator is similar to unwrap and expect
    // in that if it succeeds it returns the value to f
    // and if it fails, it will return the error to the method caller
    //let mut f = File::open("hello.txt")?;

    /*
    // the ? above simplifies this
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    }
    */

    /* 
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
    */

    /*
    // the ? above simplifies this
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
    */

    /* 
    // chaining method calls
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?; 
    Ok(s)
    */

    // final version putting everything together
    // the fs module has the read_to_string function which returns a result enum
    fs::read_to_string("hello.txt")

    // NOTE: You cannot use the ? operator in the main function since it does not return anything
    // it can only be used in a function that returns a result or an option

    // DEFAULT: Use the result enum and error propagation
    // if you know an action will always be correct, you can use the unwrap function
}

/*
fn a() {
    b();
}

fn b() {
    c(22);
}

fn c(num: i32) {
    if num == 22 {
        panic!("Don't pass in 22!");
    }
}
*/

pub struct Guess {
    value: i32,
}

impl Guess {
    // acts like a constructor
    // makes it so that the guess value is sure to be between 1 and 100
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    // the only way to get the guess value
    // external sources cannot manipulate it
    pub fn value(&self) -> i32 {
        self.value
    }
}