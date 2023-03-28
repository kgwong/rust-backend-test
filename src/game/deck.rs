use std::{fs, io::Read};

use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Deck<CardT> {
    v: Vec<CardT>
}

impl Deck<String> {
    pub fn from(mut file: fs::File) -> Result<Self, std::io::Error> {
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let v = serde_json::from_str(&contents)?;
        Ok(Deck{
            v: v,
        })
    }
}

impl<CardT> Deck<CardT> {
    pub fn from_decks(decks: Vec<Self>) -> Self {
        Deck {
            v: decks.into_iter().map(|d| d.v).flatten().collect(),
        }
    }

    pub fn add_card(&mut self, card: CardT) {
        self.v.push(card);
    }

    pub fn draw_card(&mut self) -> Option<CardT> {
        self.v.pop()
    }

    pub fn shuffle(&mut self) {
        self.v.shuffle(&mut rand::thread_rng());
    }

}