use rc::{Rc, RcBuilder};

fn main() -> Result<(), std::io::Error> {
    let file = RcBuilder::new("flake.nix")
        .with_line_count()
        .with_word_count()
        .with_character_count()
        .with_byte_count()
        .build();

    println!("{}", file);

    Ok(())
}
