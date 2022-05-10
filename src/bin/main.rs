use rc::Rc;

fn main() -> Result<(), std::io::Error> {
    let file: Rc = Rc::new("flake.nix")
        .with_line_count()
        .with_word_count()
        .with_character_count()
        .with_byte_count()
        .build();


    file.show_contents();

    Ok(())
}
