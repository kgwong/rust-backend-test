use std::{rc::Rc, fs::File, collections::HashMap};

use log::{info, error};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{player::PlayerClient, api::{
    server_messages::{
        game_update::{GameUpdate, ClientInfo},
        drawing_parameters::DrawingParameters,
        voting_ballot::{BallotItem, VotingBallot}}}, websocket::player};
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
    curr_round: usize,
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
            players: std::vec![Player{client: host_player, ready_state: false}],
            curr_round: 0,
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

        self.players.push(Player { client: resolved_player, ready_state: false });
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
            // TODO figure out how to handle round creation
            self.rounds = std::vec![
                Round::new(self.get_client_ids(), &mut self.drawing_suggestions_deck); self.num_rounds];

            self.start_round(0);
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
        if self.curr_round != round {
            error!("Not Current Round");
            return Err(());
        }

        let round = self.get_current_round_mut();
        if let Some(_) = round.get_drawing(&client_id) {
            error!("Drawing already Exists");
            return Err(()) //TODO drawing already exist
        }

        round.set_drawing(&client_id, drawing);
        if round.is_done() {
            self.send_voting_ballots();
            //TODO round shouldn't be done until the voting phase is done for it
            self.finish_round();
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

    fn get_current_round_mut(&mut self) -> &mut Round {
        self.rounds.get_mut(self.curr_round).unwrap()
    }

    fn get_current_round(&self) -> &Round {
        self.rounds.get(self.curr_round).unwrap()
    }

    fn start_round(&mut self, round_num: usize) {
        self.state = GameState::DrawingPhase;
        self.curr_round = round_num; //TODO: should we havn a non-started state?, or value in an enum?
        self.broadcast_update();
        self.send_drawing_parameters();
    }

    fn finish_round(&mut self) {
        self.curr_round += 1;
        self.state = GameState::VotingPhase;
        self.broadcast_update()
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
        let round = self.get_current_round();
        for p in self.players.iter() {
            p.client.client_addr.do_send(
                DrawingParameters {
                    message_name: "drawing_parameters".to_string(),
                    round: self.curr_round,
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
        let round = self.get_current_round();
        let drawings = round.get_data();

        let full_ballot: HashMap<&Uuid, BallotItem> =
            drawings.iter().map(|(player_id, round_data)| {
                info!("Collecting ballot for client_id: {}", player_id);
                let b = BallotItem {
                    id: Uuid::new_v4(),
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
            round: self.curr_round,
            ballot: ballot,
        })
    }
}