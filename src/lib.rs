use std::fmt;
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
    file: String,
    line_count: usize,
    word_count: usize,
    character_count: usize,
}

impl ReadFile {
    pub fn new(file: String) -> Self {
        Self {
            file,
            line_count: 0,
            word_count: 0,
            character_count: 0,
        }
    }

    pub fn word_count(mut self) -> Self {
        self.word_count = self
            .file
            .lines()
            .map(|line| line.split_whitespace().count())
            .sum();

        self
    }

    pub fn line_count(mut self) -> Self {
        self.line_count = self.file.lines().count();

        self
    }

    pub fn character_count(mut self) -> Self {
        self.character_count = self.file.chars().count();

        self
    }
}

impl fmt::Display for ReadFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Line Count: {}\nWord Count: {}\nCharacter Count: {}",
            self.line_count, self.word_count, self.character_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn counts() {
        let file = ReadFile::new(std::fs::read_to_string("flake.nix").unwrap());
        let test = file.line_count().word_count().character_count();
        let v = vec![test.line_count, test.word_count, test.character_count];

        assert_eq!(vec![40, 85, 964], v);
    }
}
