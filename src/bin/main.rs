use rc::Rc;

fn main() -> Result<(), std::io::Error> {
    let file: Rc = Rc::new("flake.nix")
        .with_line_count()
        .with_word_count()
        .with_character_count()
        .with_byte_count()
        .build();



    println!("{}", file.head(10));
    println!("{}", file.tail(10));

    Ok(())
}
