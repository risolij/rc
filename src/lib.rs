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
    byte_count: usize,
}

impl ReadFile {
    pub fn new(file: String) -> Self {
        Self {
            file,
            line_count: 0,
            word_count: 0,
            character_count: 0,
            byte_count: 0,
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

    pub fn byte_count(mut self) -> Self {
        self.byte_count = self.file.as_bytes().len();
        self
    }

    pub fn character_count(mut self) -> Self {
        self.character_count = self.file.chars().count();

        self
    }

    pub fn split(mut self, value: usize) -> Self {
        self.file = format!(
            "{}{}",
            self.file
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
            "Line Count: {}\nWord Count: {}\nCharacter Count: {}\nByte Count: {}",
            self.line_count, self.word_count, self.character_count, self.byte_count
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

        let file2 = ReadFile::new(std::fs::read_to_string("flake.nix").unwrap());
        let test2 = file2.split(30).line_count().word_count().character_count();
        let v2 = vec![test2.line_count, test2.word_count, test2.character_count];

        assert_eq!(vec![40, 85, 964], v);
        assert_eq!(vec![30, 70, 746], v2);
    }
}
