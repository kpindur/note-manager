use std::{collections::HashMap, fs::File, path::Path};
use std::io::{self, Read, Write};

use crate::note::Note;

pub struct App {
    pub notes: HashMap<String, Note>,
    pub current_note: Option<String>,
    pub input_mode: InputMode,
}

pub enum InputMode {
    Normal, 
    Editing,
}

impl App {
    pub fn new(file_path: String) -> std::io::Result<Self> {
        let notes: Vec<Note> = if Path::new(&file_path).exists() {
            let mut file = File::open(&file_path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            serde_json::from_str(&contents)?
        } else {
            Vec::new()
        };

        let notes: HashMap<String, Note> = notes.iter().map(|note| return (note.id().clone(), note.clone())).collect();

        return Ok(App { notes, current_note: None, input_mode: InputMode::Normal } );
    }

    pub fn load_notes(&mut self) -> std::io::Result<()> {
        return Ok(());
    }
}
