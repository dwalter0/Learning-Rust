use rand::prelude::*;
use std::io;

fn main() {

    loop{
        let won_the_game = run_game();
        if won_the_game {
            println!("You won the game, starting again.");
        }
        else {
            println!("You lost the game, starting again.")
        }
    }

}

fn run_game() -> bool {
    let number = thread_rng().gen_range(1..101);
    let mut guess: i32 = 0;
    let mut attempts : i32 = 0;
    println!("A number has been generated between 1 and 100. Guess the number that has been entered. You have 5 attempts to guess the number.");
   while guess != number {
        let mut buffer = String::new();
        //This will throw an error if an int isn't entered by the user. Error handling has not been covered yet.
        io::stdin().read_line(&mut buffer).expect("Failed to read standard in.");
        guess = buffer.trim().parse().expect("Failed to parse the input value.");
        if guess < number {
            attempts += 1;
            println!("Higher");
        } else if guess > number {
            attempts += 1;
            println!("Lower");
            
        }
        println!("You have made {} attempts out of 5",attempts);

        if attempts == 5 {
            println!("Out of guesses");
            return false;
        }
   }

   println!("Correct! The number is {}.",number);
   return true;
}
