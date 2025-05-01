use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
//use crate::errors::TokenizerError;

#[derive(Debug,Serialize,Deserialize)]
pub struct Vocab {
    token_to_id: HashMap<String, usize>,
    id_to_token: HashMap<usize, String>,
}


impl Vocab {

    pub fn new() -> Self {
        Self { 
            token_to_id: HashMap::new(),
             id_to_token: HashMap::new(), 
            }    
    }

    pub fn add_token(&mut self, token: &str) -> usize {
        if let Some(&id) = self.token_to_id.get(token) {
            id
        } else {
            let id = self.token_to_id.len();
            self.token_to_id.insert(token.to_string(), id);
            self.id_to_token.insert(id, token.to_string());
            id
        }
    }

    
    pub fn get_id(&self, token: &str) -> Option<usize> {
        self.token_to_id.get(token).copied()
    }

    
    pub fn get_token(&self, id:usize) -> Option<&String> {
        self.id_to_token.get(&id)
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self)?;

        Ok(())
    }

    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let vocab = serde_json::from_reader(reader)?;
        Ok(vocab)
    }
}


#[cfg(test)]

pub mod tests {
    use super::*;

    #[test]
    fn test_vocab_save_load() {
        let mut vocab = Vocab::new();
        vocab.add_token("hello");
        vocab.add_token("there");

        vocab.save("vocab_test.json").expect("Failed save");

        let loaded_vocab = Vocab::load("vocab_test.json").expect("Failed load");

        assert_eq!(vocab.token_to_id, loaded_vocab.token_to_id);
        assert_eq!(vocab.id_to_token, loaded_vocab.id_to_token);

        std::fs::remove_file("vocab_test.json").expect("failed delete");
        
    }
    

}