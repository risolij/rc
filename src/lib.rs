use std::fmt;
use std::fs;
use std::io;

#[derive(Debug)]
pub enum ReadFileError {
    ReadFileParse(io::Error),
}

impl From<io::Error> for ReadFileError {
    fn from(e: io::Error) -> ReadFileError {
        ReadFileError::ReadFileParse(e)
    }
}

#[derive(Debug)]
pub struct ReadFile {
    filename: &'static str,
    contents: String,
    line_count: usize,
    word_count: usize,
    character_count: usize,
    byte_count: usize,
}

impl ReadFile {
    pub fn new(filename: &'static str) -> Self {
        Self {
            filename,
            contents: ReadFile::contents(filename.to_string()),
            line_count: 0,
            word_count: 0,
            character_count: 0,
            byte_count: 0,
        }
    }

    pub fn contents(file: String) -> String {
        match fs::read_to_string(file) {
            Ok(file) => file,
            Err(e) => {
                println!("uh oh: {}", e);
                std::process::exit(1);
            }
        }
    }

    pub fn word_count(mut self) -> Self {
        self.word_count = self
            .contents
            .lines()
            .map(|line| line.split_whitespace().count())
            .sum();

        self
    }
    pub fn line_count(mut self) -> Self {
        self.line_count = self.contents.lines().count();

        self
    }

    pub fn byte_count(mut self) -> Self {
        self.byte_count = self.contents.as_bytes().len();
        self
    }

    pub fn character_count(mut self) -> Self {
        self.character_count = self.contents.chars().count();

        self
    }

    pub fn split(mut self, value: usize) -> Self {
        self.contents = format!(
            "{}{}",
            self.contents
                .lines()
                .take(value)
                .collect::<Vec<&str>>()
                .join("\n"),
            '\n'
        );
        self
    }
}

impl fmt::Display for ReadFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "File: {}\nLine Count: {}\nWord Count: {}\nCharacter Count: {}\nByte Count: {}",
            self.filename, self.line_count, self.word_count, self.character_count, self.byte_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn counts() {
        let file = ReadFile::new("flake.nix");
        let test = file.line_count().word_count().character_count();
        let v = vec![test.line_count, test.word_count, test.character_count];

        let file2 = ReadFile::new("flake.nix");
        let test2 = file2.split(30).line_count().word_count().character_count();
        let v2 = vec![test2.line_count, test2.word_count, test2.character_count];

        assert_eq!(vec![40, 85, 964], v);
        assert_eq!(vec![30, 70, 746], v2);
    }
}
