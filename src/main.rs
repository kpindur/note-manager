#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

use std::error::Error;

mod note;
use crate::note::Note;

fn main() -> Result<(), Box<dyn Error>> {
    let loaded_note: Note = Note::load_from_json("sample.json")?;

    println!("Loaded note:");
    println!("Contents: {}", loaded_note.contents());
    println!("Identifier: {:?}", loaded_note.id());
    println!("Tags: {:?}", loaded_note.tags());
    println!("Created at: {}", loaded_note.metadata().created_at);

    return Ok(());
}
