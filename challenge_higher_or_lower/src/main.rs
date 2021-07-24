use rand::prelude::*;
use std::io;

fn main() {
   let number = thread_rng().gen_range(1..101);
   let mut guess: i32 = 0;
   while guess != number {
        println!("A number has been generated. Guess the number that has been entered.");
        let mut buffer = String::new();
        //This will throw an error if an int hasn't been added. Error handling has not been covered yet.
        io::stdin().read_line(&mut buffer);
        guess = buffer.trim().parse().unwrap();
        if guess < number {
            println!("Higher")
        } else if guess > number {
            println!("Lower")
        }
   }
   println!("Correct! The number is {}.",number)
}
