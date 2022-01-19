use rc::{ReadFile, ReadFileError};
use std::env;
use std::fs;

fn main() -> Result<(), ReadFileError> {
    let args = argument_handling();
    let file = create_readfile(args);

    println!(
        "{}",
        file.split(30).line_count().word_count().character_count().byte_count()
    );

    Ok(())
}

fn create_readfile(args: Vec<String>) -> ReadFile {
    match fs::read_to_string(args[1].clone()) {
        Ok(file) => ReadFile::new(file),
        Err(e) => {
            println!("uh oh: {}", e);
            std::process::exit(1);
        }
    }
}

fn argument_handling() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    is_enough_arguments(args.len());

    args
}

fn is_enough_arguments(number: usize) {
    if number < 2 {
        println!("uh oh");
        println!("================================");
        println!("Please enter more arguments");
        std::process::exit(1);
    }
}
