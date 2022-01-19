use rc::{ReadFile, ReadFileError};

fn main() -> Result<(), ReadFileError> {
    let file = ReadFile::new("flake.nix");

    println!(
        "{}",
        file.split(30)
            .line_count()
            .word_count()
            .character_count()
            .byte_count()
    );

    Ok(())
}
