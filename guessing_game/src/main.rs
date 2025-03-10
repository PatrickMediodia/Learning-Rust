use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess the number!");

    //range (inclusive..exclusive)
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        //variables are immutable by default
        let mut guess: String = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //parsing could fail that is why expect is needed
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");

        //if you want to handle inputs, use the enum being returned by the parse function
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        //println!("The secret number is: {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()), 
            Ordering::Greater => println!("{}", "Too big!".red()), 
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
