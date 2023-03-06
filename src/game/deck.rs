use std::{fs, io::Read};

use serde::Deserialize;

//TODO add type param
#[derive(Debug)]
pub struct Deck<> {
    v: Vec<String>
}

impl<> Deck<> {

    pub fn from(mut file: fs::File) -> Result<Deck<>, std::io::Error> {
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let v = serde_json::from_str(&contents)?;
        Ok(Deck{
            v: v,
        })
    }

    pub fn draw_card(&mut self) -> Option<String> {
        self.v.pop()
    }

}