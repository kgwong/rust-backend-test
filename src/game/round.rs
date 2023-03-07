use std::collections::HashMap;

use uuid::Uuid;

use super::{drawing::Drawing, deck::Deck};

#[derive(Debug, Clone)]
pub struct RoundDataPerPlayer {
    pub drawing_suggestion: String,
    pub drawing: Option<Drawing>
}

#[derive(Debug, Clone)]
pub struct Round {
    pub round_data_per_player: HashMap<Uuid, RoundDataPerPlayer>
}

impl Round {
    pub fn new(client_ids: Vec<Uuid>, suggestion_deck: &mut Deck) -> Round {
        Round {
            round_data_per_player:
                client_ids.into_iter().map(|id|
                    (id, RoundDataPerPlayer{
                        drawing_suggestion: suggestion_deck.draw_card().unwrap(),
                        drawing: None
                    })
                ).collect()
        }
    }

    pub fn get_drawing_suggestion(&self, client_id: &Uuid) -> Option<&String> {
        self.round_data_per_player.get(client_id).map(|data| &data.drawing_suggestion)
    }

    pub fn get_drawing(&self, client_id: &Uuid) -> &Option<Drawing> {
        // TODO: this is fatal, probably don't crash the server tho
        &self.round_data_per_player.get(client_id).unwrap().drawing
    }

    pub fn set_drawing(&mut self, client_id: &Uuid, drawing: Drawing) {
        let player_data = self.round_data_per_player.get_mut(client_id).unwrap();
        player_data.drawing = Some(drawing);
    }

    pub fn is_done(&self) -> bool {
        return self.round_data_per_player.iter()
            .fold(true, |acc, (_, v)| acc && v.drawing.is_some())
    }
}