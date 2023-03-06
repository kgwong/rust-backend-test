use std::collections::HashMap;

use uuid::Uuid;

use super::drawing::Drawing;

#[derive(Debug, Clone)]
pub struct Round {
    pub player_drawings: HashMap<Uuid, Option<Drawing>>
}

impl Round {
    pub fn new(client_ids: Vec<Uuid>) -> Round {
        Round {
            player_drawings:
                client_ids.into_iter().map(|id| (id, None)).collect()
        }
    }

    pub fn get_drawing(&self, client_id: &Uuid) -> &Option<Drawing> {
        // TODO: this is fatal, probably don't crash the server tho
        self.player_drawings.get(client_id).unwrap()
    }

    pub fn set_drawing(&mut self, client_id: &Uuid, drawing: Drawing) {
        self.player_drawings.insert(*client_id, Some(drawing));
    }

    pub fn is_done(&self) -> bool {
        return self.player_drawings.iter()
            .fold(true, |acc, (_, v)| acc && v.is_some())
    }
}