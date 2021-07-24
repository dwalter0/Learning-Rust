use std::fs;
use std::env;

fn main() {
    if env::args().len() != 3{
        println!("Program requires exactly 2 arguments.");
        return;
    }
    
    let file_path = env::args().nth(1).unwrap();
    let name = env::args().nth(2).unwrap();
    
    let contents = fs::read_to_string(file_path).unwrap();

    for line in contents.lines() {
        if line == name {
            println!("The name is on the roster.");
            return;
        }
    }

    println!("The name wasn't found in the roster.")
    
}
