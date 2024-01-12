fn main() {
    /* 
        - Generics are great because they reduce duplication
        - Generics does not incur a performance hit
        - It changes the generics to the type they need to be at compile time
    */

    // both option and result enums use generic types
    /* 
    enum Option<T> {
        Some(T),
        None,
    }
    
    enum Result<T> {
        Ok(T),
        Err(E),
    }
    */

    /* 
    // generic function arguments
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = get_largest(number_list);
    println!("The largest number is {}", largest);

    // generic structs
    let p1 = Point { x:5, y:10 };
    let p2 = Point { x:5.0, y:10.0 };
    // let p3 = Point { x:5.0, y:10 };

    // f64 only can use the y method
    p2.y();
    p2.x();

    // any other type cannot use the y method
    // p1.y();
    */

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

/* 
fn get_largest(number_list: Vec<i32>) -> i32 {
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    largest
}
*/

// generics are specificed in angle brackets after the function name
// uses traits to tell the function that it must only take generics that can be ordered and coppied
fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

// generics are specificed in angle brackets after the struct name
// x is of generic type T, and y is of another generic type U
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

/* 
struct Point<T> {
    x: T,
    y: T,
}

// x method will be available to all types
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// y method will only be available to f64 types
impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}
*/

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // returns a generic that mixes the x of self and the y of the point passed to the function
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}