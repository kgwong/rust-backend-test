use std::{rc::Rc, fs::File, collections::HashMap};

use log::{info, error, debug, warn};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{client_connection::ClientConnection, api::{
    server_messages::{
        lobby_update::{LobbyUpdate},
        drawing_parameters::DrawingParameters,
        voting_ballot::{BallotItem, VotingBallot}}}};
use super::{player_view::{Player, PlayerState}, drawing::{Drawing}, round::Round, deck::Deck, imprint_selector};

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

    last_player_number: usize,
    host_id: Uuid,
    players: HashMap<Uuid, Player>,

    curr_round: Option<usize>, // 1-indexed
    rounds: Vec<Round>,
    num_rounds: usize,

    drawing_suggestions_deck: Deck<>,

}

// Public API
impl Game{

    pub fn new(
        room_code: String,
        host_player_client_connection: Rc<ClientConnection>,
        host_player_name: String
    ) -> Self {
        Game {
            room_code: room_code,
            state: GameState::WaitingForPlayers,
            last_player_number: 0,
            host_id: host_player_client_connection.id.clone(),
            players: HashMap::from([(
                host_player_client_connection.id,
                Player::new(host_player_client_connection, host_player_name, 0)
            )]),
            curr_round: None,
            rounds: std::vec![],
            num_rounds: 5,
            drawing_suggestions_deck:
                Deck::from(File::open("./drawing_suggestions.json").expect("file")).expect("expect"),
        }
    }

    pub fn add_player(
        &mut self,
        client_connection: Rc<ClientConnection>,
        proposed_name: &str
    ) -> Result<(), JoinGameError> {
        if self.players.len() == MAX_PLAYERS {
            return Err(JoinGameError);
        }
        if self.state != GameState::WaitingForPlayers {
            return Err(JoinGameError)
        }

        self.last_player_number += 1;
        let player = Player::new(client_connection, self.resolve_name(proposed_name), self.last_player_number);
        self.players.insert(player.client.id, player);

        info!("CurrentPlayers: {:?}", self.players);
        self.broadcast_update();
        return Ok(());
    }

    /***
     * Completely removes a player if the game isn't in progress. If
     * the game has started, set their state to disconnected so
     * that they may reconnect
     */
    pub fn disconnect_player(&mut self, client_id: &Uuid) {
        match self.state {
            GameState::WaitingForPlayers | GameState::Results => {
                if let Some(player) = self.players.remove(client_id) {
                    info!("Removing {} from game", player.name);
                    if self.is_host(client_id) && !self.all_players_disconnected(){
                        self.host_id = self.get_eldest_player_id().clone();
                    }
                } else {
                    warn!("Player {} does not exist in game", client_id);
                }
            }
            GameState::DrawingPhase | GameState::VotingPhase => {
                info!("TODO set disconnect state")
            },
        }
        self.broadcast_update();
    }

    pub fn all_players_disconnected(&self) -> bool {
        for (id, player) in self.players.iter() {
            return false
        }
        true
    }

    pub fn start_game(&mut self, client_id: Uuid) -> Result<(), StartGameError> {
        if self.is_host(&client_id) {
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

    pub fn set_player_ready(&mut self, client_id: &Uuid, ready_state: bool) -> Result<(), ()> {
        if let Some(player) = self.players.get_mut(client_id){
            let state = match ready_state { true => PlayerState::Ready, false => PlayerState::NotReady };
            player.state = state;
            self.broadcast_update();
            Ok(())
        } else {
            Err(()) // TODO
        }
    }

    pub fn submit_drawing(&mut self, client_id: &Uuid, drawing: Drawing, round: usize) -> Result<(), ()> {
        if self.curr_round != Some(round) {
            error!("Not Current Round: curr_round: {}, round {}", self.curr_round.unwrap(), round);
            return Err(());
        }

        {
            let round = self.get_current_round_mut().ok_or(())?;
            if let Some(_) = round.get_drawing(&client_id) {
                error!("Drawing already Exists");
                return Err(()) //TODO drawing already exist
            }

            round.set_drawing(&client_id, Rc::new(drawing));
        }

        self.set_player_state(&client_id, PlayerState::DrawingDone);

        let round = self.get_current_round().ok_or(())?;
        if round.is_done_drawing() {
            self.send_voting_ballots();
            self.state = GameState::VotingPhase;
            for player in self.players.values_mut() {
                player.state = PlayerState::Voting;
            }
            self.broadcast_update()
        }
        Ok(())
    }

    pub fn submit_vote(&mut self, client_id: &Uuid, votes: HashMap<Uuid, i32>) -> Result<(), ()>{
        {
            let round = self.get_current_round_mut().ok_or(())?;
            round.submit_vote(&client_id, votes);
        }
        self.set_player_state(&client_id, PlayerState::VotingDone);

        let round = self.get_current_round().ok_or(())?;
        if !round.is_done_voting() {
            return Ok(())
        }

        let scores = round.get_scores();
        self.add_to_score(&scores);

        // this is the last round, go to results
        if self.curr_round == Some(self.num_rounds) {
            self.state = GameState::Results;
            for player in self.players.values_mut(){
                player.state = PlayerState::NotReady;
            }
            self.broadcast_update();
        } else {
            self.start_next_round();
        }
        Ok(())
    }


}

impl Game{
    fn is_host(&self, client_id: &Uuid) -> bool {
        *client_id == self.host_id
    }


    fn get_eldest_player_id(&self) -> &Uuid {
        let (_, eldest_player) = self.players.iter().min_by(
            |(_, p1), (_, p2)| p1.number.cmp(&p2.number)
            ).expect("should always have players");
        &eldest_player.client.id
    }

    /**
     *  Returns a new name in the form of `name(1)` if it's a duplicate of an existing name
     */
    fn resolve_name(&self, proposed_name: &str) -> String {
        let trimmed_name = proposed_name.trim();
        let mut best_name = trimmed_name.to_string();
        let mut count = 1;
        while self.players.values().any(|p| p.name == best_name) {
            best_name = format!("{}({})", trimmed_name, count);
            count += 1;
        }
        best_name
    }

    fn get_current_round_mut(&mut self) -> Option<&mut Round> {
        self.rounds.last_mut()
    }

    fn get_current_round(&self) -> Option<&Round> {
        self.rounds.last()
    }

    fn start_next_round(&mut self) {
        let mut imprint_map: HashMap<Uuid, Option<Rc<Drawing>>> = HashMap::new();
        if let Some(round) = self.get_current_round() {
            imprint_map = round.get_data().iter()
                .map(|(player_id, data)| {
                    let drawing: &Drawing = data.drawing.as_ref().unwrap();
                    (player_id.clone(), Some(Rc::new(imprint_selector::random(drawing, 3))))
                })
                .collect();
        }

        self.curr_round = Some(self.curr_round.map_or(1, |v| v + 1));
        let player_ids: Vec<&Uuid> = self.players.keys().collect();
        self.rounds.push(
            Round::new(
                player_ids,
                &mut self.drawing_suggestions_deck,
                &imprint_map,
            ));

        self.state = GameState::DrawingPhase;
        for player in self.players.values_mut() {
            player.state = PlayerState::Drawing
        }

        self.broadcast_update();
        self.send_drawing_parameters();
    }

    fn add_to_score(&mut self, scores: &HashMap<Uuid, i32>) {
        for (id, player) in self.players.iter_mut() {
            if let Some(score) = scores.get(id) {
                player.score += score
            }
        }
    }

    fn set_player_state(&mut self, client_id: &Uuid, state: PlayerState) {
        debug!("Setting PlayerState {} {:?}", client_id, state);
        self.players.get_mut(client_id).expect("player should exist").state = state;
        self.broadcast_update()
    }

}

// Messaging
impl Game{
    pub fn broadcast_update(&self) {
        info!("Broadcasting update to all players");
        for player in self.players.values() {
            self.send_game_view_to_player(&player.client);
        }
    }

    pub fn send_drawing_parameters(&self) {
        let round = self.get_current_round().unwrap();
        for p in self.players.values() {
            p.client.actor_addr.do_send(
                DrawingParameters {
                    message_name: "drawing_parameters".to_string(),
                    round: self.curr_round.unwrap(),
                    drawing_suggestion:
                        round.get_drawing_suggestion(&p.client.id).unwrap().clone(),
                    imprint: round.get_imprint(&p.client.id).map(|i| (*i).clone()),
                }
            )
        }

    }

    fn send_game_view_to_player(&self, client_connection: &ClientConnection) {
        client_connection.actor_addr.do_send(
            LobbyUpdate {
                message_name: "lobby_update".to_string(),
                room_code: self.room_code.clone(),
                state: self.state.clone(),
                round: self.curr_round,
                num_rounds: self.num_rounds,
                players: self.players.iter().map(
                    |(id, p)|
                        p.to_view(self.is_host(id), *id == client_connection.id)
                ).collect(),
            }
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
                    drawing: round_data.drawing.as_ref().map(|d| (**d).clone()).expect("Drawing should exist"),
                };
                (player_id, b)
            }).collect();

        for p in self.players.values() {
            self.send_voting_ballots_to_player(&p.client, &full_ballot);
        }
    }

    fn send_voting_ballots_to_player(
        &self,
        client_connection: &ClientConnection,
        full_ballot: &HashMap<&Uuid, BallotItem>) {
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
        client_connection.actor_addr.do_send(VotingBallot {
            message_name: "voting_ballot".to_string(),
            round: self.curr_round.unwrap(),
            ballot: ballot,
        })
    }
}