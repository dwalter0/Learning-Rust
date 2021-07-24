use std::env;
use std::fs;
use std::io::prelude::*;

fn main() {
    if env::args().len() != 3 {
        println!("Program requires exactly 2 arguments.");
        return;
    }
    let file_path = env::args().nth(1).unwrap();
    let name = env::args().nth(2).unwrap();

    if !find_name(&name, &file_path) {
        println!(
            "The name \"{}\" wasn't found in the roster. Adding it to the file.",
            name
        );

        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(&file_path)
            .unwrap();
        //No error handling below as this has not been covered yet.
        file.write(b"\n");
        file.write(name.as_bytes());

        let _unused_bool = find_name(&name, &file_path);
    }
}

fn find_name(name: &String, file_path: &String) -> bool {
    let contents = fs::read_to_string(&file_path).unwrap();
    for (i, line) in contents.lines().enumerate() {
        if line == name {
            println!(
                "The name \"{}\" is on the roster. It was found on line {}",
                name,
                i + 1
            );
            return true;
        }
    }
    return false;
}
