#![feature(iter_intersperse)]

use std::{fmt, fs};

pub struct RcBuilder {
    filename: &'static str,
    contents: String,
    line_count: Option<usize>,
    word_count: Option<usize>,
    character_count: Option<usize>,
    byte_count: Option<usize>,
}

impl RcBuilder {
    pub fn with_line_count(&mut self) -> &mut Self {
        self.line_count = Some(self.contents.lines().count());

        self
    }

    pub fn with_word_count(&mut self) -> &mut Self {
        self.word_count = Some(
            self.contents
                .lines()
                .map(|line| line.split_whitespace().count())
                .sum(),
        );

        self
    }

    pub fn with_character_count(&mut self) -> &mut Self {
        self.character_count = Some(self.contents.chars().count());

        self
    }

    pub fn with_byte_count(&mut self) -> &mut Self {
        self.byte_count = Some(self.contents.as_bytes().len());

        self
    }

    pub fn build(&self) -> Rc {
        Rc {
            filename: self.filename,
            contents: self.contents.clone(),
            line_count: self.line_count.unwrap_or_default(),
            word_count: self.word_count.unwrap_or_default(),
            character_count: self.character_count.unwrap_or_default(),
            byte_count: self.byte_count.unwrap_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct Rc {
    filename: &'static str,
    contents: String,
    line_count: usize,
    word_count: usize,
    character_count: usize,
    byte_count: usize,
}

impl Rc {
    pub fn new(filename: &'static str) -> RcBuilder {
        RcBuilder {
            filename,
            contents: fs::read_to_string(filename).unwrap_or_default(),
            line_count: None,
            word_count: None,
            character_count: None,
            byte_count: None,
        }
    }

    pub fn head(&self, lines: usize) -> String {
        self.contents
            .lines()
            .take(lines)
            .intersperse("\n")
            .collect::<String>()
    }

    pub fn tail(&self, lines: usize) -> String {
        self.contents
            .lines()
            .rev()
            .take(lines)
            .intersperse("\n")
            .collect::<String>()
    }

    pub fn show_contents(&self) {
        println!("{}", self.contents);
    }
}

impl fmt::Display for Rc {
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
    fn can_display() {
        let file = Rc::new("flake.nix")
            .with_line_count()
            .with_word_count()
            .with_character_count()
            .with_byte_count()
            .build();

        assert_eq!(format!("{}", file), "File: flake.nix\nLine Count: 37\nWord Count: 82\nCharacter Count: 926\nByte Count: 926");
    }

    #[test]
    fn has_proper_line_count() {
        let file = Rc::new("flake.nix").with_line_count().build();

        assert_eq!(37, file.line_count);
    }
}
