use std::io;
use std::fmt;

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
}

impl ReadFile {
    pub fn new(file: String) -> Self {
        Self {
            file,
            line_count: 0,
            word_count: 0,
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
}

impl fmt::Display for ReadFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Line Count: {}\nWord Count: {}",
            self.line_count, self.word_count
        )
    }
}
