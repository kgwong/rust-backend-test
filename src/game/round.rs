use std::{collections::HashMap, rc::Rc, cell::RefCell};

use log::info;
use uuid::Uuid;

use super::{drawing::Drawing, deck::Deck, imprint_mapper, player_view::Player, errors::SubmitVoteError};

const MAX_VOTES_PER_ROUND: i32 = 3;

// TODO: this struct doesn't really make sense
#[derive(Debug, Clone)]
pub struct RoundDataPerPlayer {
    pub drawing_id: Uuid,
    pub drawing_suggestion: String,
    pub imprint: Option<Rc<Drawing>>,
    pub drawing: Option<Rc<Drawing>>,
    pub has_voted: bool,
    pub votes: i32,
}

#[derive(Debug, Clone)]
pub struct Round {
    round_data_per_player: HashMap<Uuid, RoundDataPerPlayer>,
    players: HashMap<Uuid, Rc<RefCell<Player>>>,
}

impl Round {
    pub fn new(
        players: HashMap<Uuid, Rc<RefCell<Player>>>,
        suggestion_deck: &mut Deck,
        imprint_map: &HashMap<Uuid, Option<Rc<Drawing>>>,

    ) -> Round {
        let selected_imprints = imprint_mapper::random(imprint_map);
        Round {
            round_data_per_player:
                players.keys().map(|id|
                    (id.clone(), RoundDataPerPlayer{
                        drawing_id: Uuid::new_v4(),
                        drawing_suggestion: suggestion_deck.draw_card().unwrap(),
                        imprint: selected_imprints.get(&id).and_then(|x| x.clone()),
                        drawing: None,
                        has_voted: false,
                        votes: 0,
                    })
                ).collect(),
            players,
        }
    }

    pub fn get_data(&self) -> &HashMap<Uuid, RoundDataPerPlayer> {
        &self.round_data_per_player
    }

    pub fn get_drawing_suggestion(&self, client_id: &Uuid) -> Option<&String> {
        self.round_data_per_player.get(client_id).map(|data| &data.drawing_suggestion)
    }

    pub fn get_imprint(&self, client_id: &Uuid) -> Option<Rc<Drawing>> {
        self.round_data_per_player.get(client_id).map(|i| i.imprint.clone()).flatten()
    }

    pub fn get_drawing(&self, client_id: &Uuid) -> Option<Rc<Drawing>> {
        self.round_data_per_player.get(client_id).map(|s| s.drawing.clone()).flatten()
    }

    pub fn set_drawing(&mut self, client_id: &Uuid, drawing: Rc<Drawing>) {
        info!("set_drawing client_id: {}", client_id);
        let player_data = self.round_data_per_player.get_mut(client_id).expect("Player should exist");
        player_data.drawing = Some(drawing);
    }

    pub fn submit_vote(&mut self, client_id: &Uuid, votes: HashMap<Uuid, i32>)
    -> Result<(), SubmitVoteError> {
        if votes.values().sum::<i32>() > MAX_VOTES_PER_ROUND {
            return Err(SubmitVoteError::MaximumVotesExceeded)
        }
        let client_drawing = self.round_data_per_player.get(client_id).expect("player should exist").drawing_id;
        if let Some(v) = votes.get(&client_drawing) {
            if *v > 0 {
                return Err(SubmitVoteError::ClientVotedForSelf)
            }
        }
        let drawing_ids: Vec<_> = self.round_data_per_player.values().map(|v| v.drawing_id).collect();
        if votes.keys().any(|id| !drawing_ids.contains(id)) {
            return Err(SubmitVoteError::InvalidDrawingId) // TODO include the drawing id in the error
        }

        for data in self.round_data_per_player.values_mut() {
            data.votes += votes.get(&data.drawing_id).unwrap();
        }
        let player_data = self.round_data_per_player.get_mut(client_id).unwrap();
        player_data.has_voted = true;
        Ok(())
    }

    /**
     * Returns true if all connected players have submitting a drawing
     */
    pub fn is_done_drawing(&self) -> bool {
        self.round_data_per_player.iter()
            .filter(
                |(player_id, _)| {
                    let s = self.players.get(*player_id).expect("player list should be same");
                    !s.borrow().is_disconnected
                })
            .fold(true, |acc, (_, v)| acc && v.drawing.is_some())
    }

    /**
     * Returns true if all connected players have submitting a vote
     */
    pub fn is_done_voting(&self) -> bool {
        self.round_data_per_player.iter()
            .filter(
                |(player_id, _)| {
                    let s = self.players.get(*player_id).expect("player list should be same");
                    !s.borrow().is_disconnected
                })
            .fold(true, |acc, (_, v)| acc && v.has_voted)
    }

    //TODO type the Uuids
    pub fn get_scores(&self) -> HashMap<Uuid, i32> {
        self.round_data_per_player.iter().map(|(player_id, data)|
            (player_id.clone(), data.votes)).collect()
    }
}