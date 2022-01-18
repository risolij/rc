use std::env;
use std::io;


fn main() -> Result<(), FileError> {
    let args: Vec<String> = env::args().collect();
    is_enough_arguments(args.len());

    let file = File::new(args[1].clone());


    println!("{:?}", file);
    Ok(())
}

fn is_enough_arguments(number: usize) {
    if number < 2 {
        panic!("Please enter more arguments");
    }
}


#[derive(Debug)]
enum FileError {
    FileParse(io::Error),
}

impl From<io::Error> for FileError {
    fn from(e: io::Error) -> FileError {
        FileError::FileParse(e)
    }
}

#[derive(Debug)]
struct File {
    file: String,
}

impl File {
    fn new(file: String) -> Self {
        Self {
            file,
        }
    }
}
