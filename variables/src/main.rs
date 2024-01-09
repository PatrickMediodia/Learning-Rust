fn main() {
    // Line comment
    
    /*
        Block comment 
    */
    
    //by default, variables are immutable
    let x = 32;
    //x = 34;
    println!("Th value of x is: {}", x);

    /* 
    CONST
        - common practice to have variable names in constants with uppercase and underscore
        - you cannot mutate a constant
        - const must be type annotated
        - cannot be set to a value that is computed at run time
            ex. return value of a function
    */
    const SUBSCRIBER_COUNT: u32 = 100_000;

    /*
    SHADOWING
        - first x variable is shadowed by the second x variable
        - advantages
            1. keeps the mutability on both occasions
            2. you can change the type on runtime
    */
    let x: &str = "six";
    println!("Th value of x is: {}", x);

    /*
        SCALER
            - Integers (by default i32)
                - 8, 16, 32, 64, arch (depending on architecture)
                - signed or unsigned
            - Float
            - Boal
            - Character
        COMPOUND TYPES
            - Tuple
    */

    /* TUPLES */
    let tup: (&str, i32) = ("Let's Get Rusty", 100_000);
    
    let (channel, sub_count) = tup; // destructuring
    let sub_count = tup.1; // dot notation

    /* 
        ARRAYS
            - fixed length
            - you need to change size dynamically, use a vector
            - starts at 0
    */
    let error_codes = [200, 400, 500];
    let not_found = error_codes[1]; // access the value at index 1 
    let byte = [0; 8]; // create an array with 8 values and set them to 0

    let sum = my_function(11, 22);
    println!("The sum is: {}", sum);

    //control flow
    conditionals();
    loops();
}

/* 
FUNCTIONS
    - functions are declared using the fn keyword
    - snake case convention for function names
    - add parameters by name:type, name:type, name:type
    - to declare a return type use the -> return_type syntax
*/

fn my_function(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    //There are two return types
    //The last expression in a function must not have a semi colon

    /* 
    EXPLICIT RETURN
    let sum: i32 = x + y; //x + y is an expression because it returns a value
    return sum
    */

    //IMPLICIT RETURN
    x + y
}

/*
    STATEMENTS - perform an action, but returns no value
    EXPRESSIONS - returns a value
*/

fn conditionals() {
    let number = 5;

    //if expressions can only evaluate booleans
    if number == 5 {
        println!("TRUE");
    } else {
        println!("FALSE");
    }

    //let variables take expressions
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("{}", number); 
}

fn loops() {
    //LOOP
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        } else {
            println!("{}", counter);
        }
    };

    //WHILE
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1 ;
    }
    println!("LIFTOFF!!!");

    //FOR LOOP
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    //inclusive..exclusive
    for num in 1..4 {
        println!("the value is: {}", num);
    }
}