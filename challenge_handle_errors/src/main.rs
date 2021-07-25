use std::io;
use rand::prelude::*;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("I'm thinking of a number between 1 and 100...");
    println!("Guess the number:");
    loop {
        let mut guess = String::new();
        let read_result = io::stdin().read_line(&mut guess);
        match read_result {
            Ok(_) => {()}
            Err(e) => {
                eprintln!("There was some error reading input from the user. The error was \"{}\". Try again.", e);
                continue;
            }
        }
        let guess_value: i32;
        let guess_parse_result = guess.trim().parse::<i32>();
        match guess_parse_result {
            Ok(ok_guess) => {
                guess_value = ok_guess;
            }
            Err(e) => {
                eprintln!("There was some error with the input. The error was \"{}\". Try again.",e);
                continue;
            }
        }

        if guess_value > secret_number {
            println!("\n{} is too high! Guess lower:", guess);
        } else if guess_value < secret_number {
            println!("\n{} is too low! Guess higher:", guess);
        } else {
            println!("\nYou got it! The secret number was {}.", secret_number);
            break;
        }
    }    
}
