// structs and enums are the building blocks for creating new types in rust

// grouping related data with different types
// allows naming our structure and data within it
// referencing the data by name and not by index location

/* 
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // creation of new struct instances
    // when making a struct mutable, use the mut keyword
    // makes the whole struct mutable, you cannot just make one field mutable
    let mut user1 = User {
        email: String::from("patrick@gmail.com"),
        username: String::from("pat"),
        active: true,
        sign_in_count: 1,
    };

    // access values using the dot notation
    let name = user1.username;
    user1.username = String::from("new username"); // must set the whole struct to mutable on instantiation 

    // creating a user from a method
    let user2 = build_user(
        String::from("user2@gmail.com"), 
        String::from("user2")
    );

    // creation of new instances while reusing data from another instance
    let user3 = User {
        email: String::from("user3@gmail.com"), 
        username: String::from("user3"),
        ..user2
    };

    // tuple structs
    // useful in order to give your tuple a name and be of different type from other tuples
    // color and point both have the same field types, but have their own types
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
}

fn build_user(email: String, username: String) -> User {
    /* 
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
    */

    // if same name of function arguments and struct field, you can simplify
    // field init shorthand syntax
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
*/

/*
// initial implementation, problem is having the two values seprate from each other
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels,",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
*/

/* 
// using tuples, better because only passing one variable but it is not named
fn main() {
    let rect = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels,",
        area(rect)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// houses the functions and methods associated with a struct    
impl Rectangle {
    // methods are tied to an instance of a struct
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        /* 
        if self.area() > other.area() {
            return true;
        }
        false
        */
        self.width > other.width && self.height > other.height
    }
}

// you can create many implementation blocks for a single struct
// associated functions are not tied to an instance of our struct
impl Rectangle {
    // methods get passed self
    // associated methods do not get passed self
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50,
    };

    let rect3 = Rectangle::square(25);

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    println!("rect: {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels,",
        //rectarea(&rect)

        rect.area()
        // clear that the area function is associated with the Rectangle struct
        // rust uses the dot (.) notation for both objects and references as it has automatic referencing and dereferencing
    );
}

/* 
// passing a reference to the rectangle since we don't want to take ownership
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
*/