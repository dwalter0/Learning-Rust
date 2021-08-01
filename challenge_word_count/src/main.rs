use std::fs;
use std::env;
use std::collections::HashMap;

fn main() {
    if env::args().len() != 2 {
        eprintln!("Program requires an argument: <file path>");
        std::process::exit(1);
    }
    let argument_result = env::args().nth(1);
    match argument_result {
        Some(argument) => {
            run_process(argument)
        }
        None => {println!("Something went wrong with the argument.")}
    }
}

fn run_process(argument: String)
{
    let file_contents = read_file(argument);
    let file_contents_cleaned = file_contents.to_lowercase();
        let word_count = hashify_words(&file_contents_cleaned);

    get_top_word(word_count);
}

fn get_top_word(word_count: HashMap<&str,u32>){
    let mut max_val = 0;
    let mut max_key = "";
    for (k, v) in word_count.iter() {
        if *v > max_val {
            max_key = k;
            max_val = *v;
        }
    }
    println!("The top word was \"{}\" and it appeared {} times.",max_key,max_val);
}

 fn read_file(file_path: String) -> String {   
    let read_file_result = fs::read_to_string(file_path);
    match read_file_result {
        Ok(x) => {
            return x;
        }
        Err(e) => {
            eprintln!("Could not read file. The error was \"{}\". Exiting.",e);
            std::process::exit(1);
        }
    }
}

fn hashify_words<'a>(file_contents_cleaned: &'a String) -> HashMap<&'a str,u32> {
    let word_split = file_contents_cleaned.split_whitespace();
    let mut word_count = HashMap::new();
    for word in word_split {
        let word_entry = word_count.entry(word).or_insert(0);
        *word_entry += 1;
    }
    word_count
}


