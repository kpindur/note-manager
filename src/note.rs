use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifier(String);

#[derive(Debug, Serialize, Deserialize)]
pub enum Link {
    Internal(Identifier),
    External(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    contents: String,
    id: Identifier,
    links: Vec<Link>,
    tags: Vec<String>,
    metadata: Metadata,
}

impl Note {
    pub fn new(contents: String, id: Identifier, links: Vec<Link>, tags: Vec<String>) -> Self {
        let now = Utc::now();
        return Note {
            contents, id, links, tags,
            metadata: Metadata { created_at: now, updated_at: now },
        };
    }

    pub fn save_to_json(&self, filename:&str) -> std::io::Result<()> {
        let json = serde_json::to_string(self)?;
        let mut file = File::create(filename)?;
        file.write_all(json.as_bytes())?;
        return Ok(());
    }
    
    pub fn load_from_json(filename: &str) -> std::io::Result<Self> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        return Ok(serde_json::from_str(&contents)?);
    }

    pub fn contents(&self) -> &String { return &self.contents; } 
    pub fn id(&self) -> &String { return &self.id.0; }
    pub fn links(&self) -> &Vec<Link> { return &self.links; }
    pub fn tags(&self) -> &Vec<String> { return &self.tags; }
    pub fn metadata(&self) -> &Metadata { return &self.metadata; }
}

