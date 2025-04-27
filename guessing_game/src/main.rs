//Prelude: default scope for every Rust program. Everything else here is outside this prelude

use rand::Rng;
use std::{cmp::Ordering, io}; //standard library (I/O  and Ordering) //standard RNG library

// Guessing game
fn main() {
    // println! is a macro
    println!("Guess the number!");

    // non-mutable random generated int 32 number between 1 and 100 (rangeStart..=rangeEnd)
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //game loop
    loop {
        println!("Please input your guess");

        //New mutable variable bound to a empty string which size is not know at compile time
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) //read line from cmd and put it in guess (through reference)
            .expect("failed to read line"); // throw exception, needed because .read_line return a Result enum that MUST be handled, or else there is a warning at compile time 

        // Shadows the guess variable, giving it a new value taken fron trimming whitespaces
        // and carriage return form the input string, and then trying to convert it in unsigner number (u32)
        let guess: u32 = match guess.trim().parse() {
          Ok(num) => num, // if valid number, return it!
          Err(_) => continue, // ignore invalid inputs. "(_)" is a "catch all" expression 
        };

        // {} is string placeholder
        println!("You guessed {guess}");

        // if guess is == or < or > secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed it right!");
                break; //Exit loop
            }
        }
    }
}
