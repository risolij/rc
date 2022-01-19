use rc::{ReadFile, ReadFileError};
use std::env;
use std::fs;

fn main() -> Result<(), ReadFileError> {
    let file = argument_handling();

    println!("{}", 
        file
        .split(30)
        .line_count()
        .word_count()
        .character_count());

    Ok(())
}

fn argument_handling() -> ReadFile {
    let args: Vec<String> = env::args().collect();
    is_enough_arguments(args.len());

    match fs::read_to_string(args[1].clone()) {
        Ok(file) => ReadFile::new(file),
        Err(_) => {
            println!("uh oh");
            std::process::exit(1);
        }
    }
}

fn is_enough_arguments(number: usize) {
    if number < 2 {
        println!("uh oh");
        println!("================================");
        println!("Please enter more arguments");
        std::process::exit(1);
    }
}
