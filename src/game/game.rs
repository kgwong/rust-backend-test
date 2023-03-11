use std::{rc::Rc, fs::File, collections::HashMap};

use log::{info, error};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{player::PlayerClient, api::{
    server_messages::{
        game_update::{GameUpdate, ClientInfo},
        drawing_parameters::DrawingParameters,
        voting_ballot::{BallotItem, VotingBallot}}}};
use super::{player_view::Player, drawing::Drawing, round::Round, deck::Deck};

#[derive(Debug)]
pub struct JoinGameError;

#[derive(Debug)]
pub struct StartGameError;

const MIN_PLAYERS: usize = 2;
const MAX_PLAYERS: usize = 8;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameState{
    WaitingForPlayers,
    DrawingPhase,
    VotingPhase,
    Results,
}

#[derive(Debug)]
pub struct Game{
    room_code: String,
    state: GameState,
    players: std::vec::Vec<Player>,
    curr_round: Option<usize>, // 1-indexed
    rounds: std::vec::Vec<Round>,
    num_rounds: usize,

    drawing_suggestions_deck: Deck<>,
}

// Public API
impl Game {

    pub fn new(room_code: String, host_player: Rc<PlayerClient>) -> Self {
        Game {
            room_code: room_code,
            state: GameState::WaitingForPlayers,
            players: std::vec![Player::new(host_player)],
            curr_round: None,
            rounds: std::vec![],
            num_rounds: 5,
            drawing_suggestions_deck:
                Deck::from(File::open("./drawing_suggestions.json").expect("file")).expect("expect"),
        }
    }

    pub fn add_player(&mut self, player: Rc<PlayerClient>) -> Result<(), JoinGameError> {
        if self.players.len() == MAX_PLAYERS {
            return Err(JoinGameError);
        }
        if self.state != GameState::WaitingForPlayers {
            return Err(JoinGameError)
        }

        //TODO clean this up
        let resolved_player = Rc::new(PlayerClient{
            client_uuid: player.client_uuid,
            peer_addr: player.peer_addr,
            client_addr: player.client_addr.clone(),
            name: self.resolve_name(&player),
        });

        // TODO: move name out of player client
        self.players.push(Player::new(resolved_player));
        info!("CurrentPlayers: {:?}", self.players);
        self.broadcast_update();
        return Ok(());
    }

    pub fn start_game(&mut self, client_id: Uuid) -> Result<(), StartGameError> {
        if self.players[0].client.client_uuid == client_id {
            if self.state != GameState::WaitingForPlayers {
                return Err(StartGameError);
            }
            if self.players.len() < MIN_PLAYERS {
                return Err(StartGameError);
            }
            info!("Host is starting the game");
            self.drawing_suggestions_deck.shuffle();
            self.start_next_round();
            Ok(())
        } else {
            Err(StartGameError)
        }
    }

    pub fn set_player_ready(&mut self, client_id: Uuid, ready_state: bool) -> Result<(), ()> {
        if let Some(player) = self.players.iter_mut().find(|p| p.client.client_uuid == client_id){
            player.ready_state = ready_state;
            self.broadcast_update();
            Ok(())
        } else {
            Err(()) // TODO
        }
    }

    pub fn submit_drawing(&mut self, client_id: Uuid, drawing: Drawing, round: usize) -> Result<(), ()> {
        if self.curr_round != Some(round) {
            error!("Not Current Round: curr_round: {}, round {}", self.curr_round.unwrap(), round);
            return Err(());
        }

        let round = self.get_current_round_mut().ok_or(())?;
        if let Some(_) = round.get_drawing(&client_id) {
            error!("Drawing already Exists");
            return Err(()) //TODO drawing already exist
        }

        round.set_drawing(&client_id, drawing);
        if round.is_done_drawing() {
            self.send_voting_ballots();
            self.state = GameState::VotingPhase;
            self.broadcast_update()
        }
        Ok(())
    }

    pub fn submit_vote(&mut self, client_id: Uuid, votes: HashMap<Uuid, i32>) -> Result<(), ()>{
        let mut scores = None;

        {
            let round = self.get_current_round_mut().ok_or(())?;
            round.submit_vote(&client_id, votes);
            if round.is_done_voting() {
                scores = Some(round.get_scores());
            } else {
                return Ok(())
            }
        }

        {
            self.add_to_score(&scores.unwrap());

            // this is the last round, go to results
            if self.curr_round == Some(self.num_rounds) {
                self.state = GameState::Results;
                self.broadcast_update();
            } else {
                self.start_next_round();
            }
        }
        Ok(())
    }


}

impl Game {
    /**
     *  Returns a new name in the form of `name(1)` if it's a duplicate of an existing name
     */
    fn resolve_name(&self, player: &PlayerClient) -> String {
        let trimmed_name = player.name.trim();
        let mut proposed_name = trimmed_name.to_string();
        let mut count = 1;
        while self.players.iter().any(|p| p.client.name == proposed_name) {
            proposed_name = format!("{}({})", trimmed_name, count);
            count += 1;
        }
        proposed_name
    }

    // TODO: shouldn't need to copy these
    fn get_client_ids(&self) -> Vec<Uuid> {
        self.players.iter().map(|p| p.client.client_uuid).collect()
    }

    fn get_current_round_mut(&mut self) -> Option<&mut Round> {
        self.rounds.last_mut()
    }

    fn get_current_round(&self) -> Option<&Round> {
        self.rounds.last()
    }

    fn start_next_round(&mut self) {
        self.curr_round = Some(self.curr_round.map_or(1, |v| v + 1));
        self.rounds.push(
            Round::new(self.get_client_ids(), &mut self.drawing_suggestions_deck));

        self.state = GameState::DrawingPhase;
        self.broadcast_update();
        self.send_drawing_parameters();
    }

    fn add_to_score(&mut self, scores: &HashMap<Uuid, i32>) {
        // TODO: this is silly
        for (player_id, points) in scores.iter(){
            let player = self.players.iter_mut().find(|p| p.client.client_uuid == *player_id).unwrap();
            player.score += *points;
        }
    }
}

// Messaging
impl Game {
    pub fn broadcast_update(&self) {
        info!("Broadcasting update to all players");
        for (i, p) in self.players.iter().enumerate() {
            self.send_game_view_to_player(&p.client, i);
        }
    }

    pub fn send_drawing_parameters(&self) {
        let round = self.get_current_round().unwrap();
        for p in self.players.iter() {
            p.client.client_addr.do_send(
                DrawingParameters {
                    message_name: "drawing_parameters".to_string(),
                    round: self.curr_round.unwrap(),
                    drawing_suggestion:
                        round.get_drawing_suggestion(&p.client.client_uuid).unwrap().clone(),
                }
            )
        }

    }

    fn current_game_view(&self, client_info: ClientInfo) -> GameUpdate {
        GameUpdate {
            message_name: "game_update".to_string(),
            room_code: self.room_code.clone(),
            state: self.state.clone(),
            round: self.curr_round,
            num_rounds: self.num_rounds,
            players: self.players.iter().map(|p| p.to_view()).collect(),
            client_info: client_info,
        }
    }

    fn send_game_view_to_player(&self, player: &PlayerClient, index: usize) {
        player.client_addr.do_send(
            self.current_game_view(ClientInfo{player_index: index})
        );
    }

    fn send_voting_ballots(&self) {
        let round = self.get_current_round().unwrap();
        let drawings = round.get_data();

        let full_ballot: HashMap<&Uuid, BallotItem> =
            drawings.iter().map(|(player_id, round_data)| {
                info!("Collecting ballot for client_id: {}", player_id);
                let b = BallotItem {
                    id: round_data.drawing_id.clone(),
                    suggestion: round_data.drawing_suggestion.clone(),
                    drawing: round_data.drawing.clone().expect("drawing should exist"),
                };
                (player_id, b)
            }).collect();

        for p in self.players.iter() {
            self.send_voting_ballots_to_player(&p.client, &full_ballot);
        }
    }

    fn send_voting_ballots_to_player(&self, player: &PlayerClient, full_ballot: &HashMap<&Uuid, BallotItem>) {
        // Send all the ballot items except the players own drawing
        //let ballot: Vec<BallotItem> = full_ballot.into_iter()
        //    .filter(|(player_id, _)| player.client_uuid != **player_id)
        //    .map(|(player_id, ballot_item)| ballot_item)
        //    .collect();

        // Actually, send the whole ballot by
        //let i: Vec<&BallotItem = full_ballot.iter().map(|(i, j)| j).collect();
        let ballot: Vec<BallotItem> = full_ballot.iter()
                .map(|(_, ballot_item)| (*ballot_item).clone())
                .collect();
        player.client_addr.do_send(VotingBallot {
            message_name: "voting_ballot".to_string(),
            round: self.curr_round.unwrap(),
            ballot: ballot,
        })
    }
}