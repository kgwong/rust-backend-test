use std::{rc::Rc, fs::File, collections::HashMap, cell::RefCell};

use log::{info, error, debug, warn};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{client_connection::ClientConnection, api::{
    server_messages::{
        lobby_update::{LobbyUpdate},
        drawing_parameters::DrawingParameters,
        voting_ballot::{BallotItem, VotingBallot, VotableBallotItem}, game_settings_update::GameSettingsUpdate, results::Results}}};
use super::{
    player_view::{Player, PlayerState}, drawing::{Drawing},
    round::Round, deck::Deck, imprint_selector,
    game_settings::{GameSettings, GameMode},
    deck_repository::{self, get_available_deck_names},
    errors::*};


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
    settings: GameSettings,
    state: GameState,

    last_player_host_rank: usize,
    host_id: Uuid,
    players: HashMap<Uuid, Rc<RefCell<Player>>>,

    curr_round: Option<usize>, // 1-indexed
    rounds: Vec<Round>,

    drawing_suggestions_deck: Option<Deck<String>>,
}

// Public API
impl Game{

    pub fn new(
        room_code: String,
        host_player_client_connection: Rc<ClientConnection>,
        host_player_name: String
    ) -> Self {
        let new_game = Game {
            room_code: room_code,
            settings: GameSettings {
                mode: GameMode::Default,
                rounds: 5,
                drawing_phase_time_limit_seconds: None,
                voting_phase_time_limit_seconds: None,
                drawing_decks_included: deck_repository::get_available_deck_names().into_iter()
                                            .map(|d| (d.to_string(), true)).collect(),
            },
            state: GameState::WaitingForPlayers,
            last_player_host_rank: 0,
            host_id: host_player_client_connection.id.clone(),
            players: HashMap::from([(
                host_player_client_connection.id,
                Rc::new(RefCell::new(Player::new(host_player_client_connection, host_player_name, 0)))
            )]),
            curr_round: None,
            rounds: std::vec![],
            drawing_suggestions_deck: None,
        };
        new_game.broadcast_lobby_update();
        new_game.broadcast_settings_update();
        new_game
    }

    pub fn add_player(
        &mut self,
        client_connection: Rc<ClientConnection>,
        proposed_name: &str
    ) -> Result<(), JoinGameError> {
        if self.players.len() == MAX_PLAYERS {
            return Err(JoinGameError::GameFull);
        }
        if self.state != GameState::WaitingForPlayers {
            return Err(JoinGameError::GameAlreadyStarted)
        }

        self.send_settings_update_to_player(&client_connection);

        self.last_player_host_rank += 1;
        let player = Player::new(client_connection, self.resolve_name(proposed_name), self.last_player_host_rank);
        self.players.insert(player.client.id, Rc::new(RefCell::new(player)));

        info!("CurrentPlayers: {:?}", self.players);
        self.broadcast_lobby_update();
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
                    info!("Removing {} from game", player.borrow().name);
                    if !self.all_players_disconnected() {
                        self.update_host()
                    }
                } else {
                    warn!("Player {} does not exist in game", client_id);
                }
            }
            GameState::DrawingPhase | GameState::VotingPhase => {
                if let Some(player) = self.players.get_mut(client_id) {
                    player.borrow_mut().is_disconnected = true;
                    if !self.all_players_disconnected() {
                        self.update_host();
                        if self.state == GameState::DrawingPhase {
                            self.go_to_voting_phase_if_drawing_is_done();
                        } else if self.state == GameState::VotingPhase {
                            self.finish_round_if_voting_phase_is_done();
                        }
                    }
                } else {
                    warn!("Player {} does not exist in game", client_id);
                }
            },
        }
        self.broadcast_lobby_update();
    }

    pub fn all_players_disconnected(&self) -> bool {
        for (_, player) in self.players.iter() {
            if player.borrow_mut().is_disconnected == false {
                return false
            }
        }
        true
    }

    pub fn update_settings(&mut self, client_id: &Uuid, game_settings: &GameSettings)
    -> Result<(), UpdateGameSettingsError> {
        if self.state != GameState::WaitingForPlayers {
            return Err(UpdateGameSettingsError::GameAlreadyStarted);
        }
        if !self.is_host(&client_id) {
            return Err(UpdateGameSettingsError::ClientIsNotTheHost);
        }

        if game_settings.rounds > 25 {
            return Err(UpdateGameSettingsError::InvalidNumRounds);
        }
        // TODO: need to allow setting time limit when it's implemented
        if let Some(drawing_limit) = game_settings.drawing_phase_time_limit_seconds {
            if drawing_limit > 300 {
                return Err(UpdateGameSettingsError::InvalidDrawingTimeLimit);
            }
            return Err(UpdateGameSettingsError::InvalidDrawingTimeLimit);
        }
        if let Some(voting_limit) = game_settings.voting_phase_time_limit_seconds {
            if voting_limit > 300 {
                return Err(UpdateGameSettingsError::InvalidVotingTimeLimit);
            }
            return Err(UpdateGameSettingsError::InvalidVotingTimeLimit);
        }

        //verify that this game update doesn't remove all the decks
        let removes_all_decks = self.settings.drawing_decks_included.iter()
            .filter(|(_, i)| **i)
            .all(|(n, _)| {
                game_settings.drawing_decks_included.get(n) == Some(&false)
            });
        if removes_all_decks {
            return Err(UpdateGameSettingsError::SettingRemovesAllDecks)
        }

        for deck in game_settings.drawing_decks_included.keys() {
            if !get_available_deck_names().contains(&deck.as_str()) {
                return Err(UpdateGameSettingsError::DeckDoesNotExist)
            }
        }
        // Apply settings after passing input validation
        self.settings.mode = game_settings.mode;
        self.settings.rounds = game_settings.rounds;
        self.settings.drawing_phase_time_limit_seconds = game_settings.drawing_phase_time_limit_seconds;
        self.settings.voting_phase_time_limit_seconds = game_settings.voting_phase_time_limit_seconds;
        for (deck_name, include) in game_settings.drawing_decks_included.iter() {
            self.settings.drawing_decks_included.insert(deck_name.to_string(), *include);
        }
        self.broadcast_settings_update();
        Ok(())
    }

    pub fn start_game(&mut self, client_id: &Uuid) -> Result<(), StartGameError> {
        if !self.is_host(client_id) {
            return Err(StartGameError::ClientIsNotTheHost)
        }
        if self.state != GameState::WaitingForPlayers {
            return Err(StartGameError::GameAlreadyStarted);
        }
        if self.players.len() < MIN_PLAYERS {
            return Err(StartGameError::MinimumPlayersNotReached);
        }
        info!("Host is starting the game");

        self.drawing_suggestions_deck = Some(self.init_deck());
        self.start_next_round();
        Ok(())
    }

    pub fn play_again(&mut self, client_id: &Uuid) -> Result<(), PlayAgainError> {
        if !self.is_host(client_id) {
            return Err(PlayAgainError::ClientIsNotTheHost)
        }
        if self.state != GameState::Results {
            return Err(PlayAgainError::GameIsNotOver);
        }
        self.players.retain(|_, p| !p.borrow().is_disconnected);

        self.state = GameState::WaitingForPlayers;
        self.curr_round = None;
        self.rounds = std::vec![];
        for p in self.players.values_mut() {
            p.borrow_mut().score = 0;
        }
        self.broadcast_lobby_update();
        Ok(())
    }

    pub fn set_player_ready(&mut self, client_id: &Uuid, ready_state: bool) {
        let player = self.players.get_mut(client_id).expect("player should exist");
        let state = match ready_state { true => PlayerState::Ready, false => PlayerState::NotReady };
        player.borrow_mut().state = state;
        self.broadcast_lobby_update();
    }

    pub fn submit_drawing(&mut self, client_id: &Uuid, drawing: Drawing, round: usize)
    -> Result<(), SubmitDrawingError> {
        if self.curr_round != Some(round) {
            error!("Not Current Round: curr_round: {}, round {}", self.curr_round.unwrap(), round);
            return Err(SubmitDrawingError::DrawingSubmittedForWrongRound);
        }

        {
            let round = self.get_current_round_mut().ok_or(SubmitDrawingError::DrawingSubmittedForWrongRound)?;
            if let Some(_) = round.get_drawing(&client_id) {
                error!("Drawing already Exists");
                return Err(SubmitDrawingError::DrawingWasAlreadySubmitted)
            }

            round.set_drawing(&client_id, Rc::new(drawing));
        }

        self.set_player_state(&client_id, PlayerState::DrawingDone);
        self.go_to_voting_phase_if_drawing_is_done();
        Ok(())
    }

    pub fn submit_vote(&mut self, client_id: &Uuid, votes: HashMap<Uuid, i32>)
    -> Result<(), SubmitVoteError>{
        {
            let round = self.get_current_round_mut().ok_or(SubmitVoteError::GameHasNotStarted)?;
            round.submit_vote(&client_id, votes)?;
        }
        self.set_player_state(&client_id, PlayerState::VotingDone);
        self.finish_round_if_voting_phase_is_done();
        Ok(())
    }


}

impl Game{
    fn is_host(&self, client_id: &Uuid) -> bool {
        *client_id == self.host_id
    }

    fn update_host(&mut self) {
        self.host_id = self.get_player_with_highest_host_rank().clone();
    }

    fn get_player_with_highest_host_rank(&self) -> Uuid {
        let eldest_player = self.players.values()
            .filter(|player| !player.borrow().is_disconnected)
            .min_by(
                |p1, p2| p1.borrow().host_rank.cmp(&p2.borrow().host_rank))
            .expect("should always have players");
        eldest_player.borrow().client.id
    }

    /**
     *  Returns a new name in the form of `name(1)` if it's a duplicate of an existing name
     */
    fn resolve_name(&self, proposed_name: &str) -> String {
        let trimmed_name = proposed_name.trim();
        let mut best_name = trimmed_name.to_string();
        let mut count = 1;
        while self.players.values().any(|p| p.borrow().name == best_name) {
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

    fn set_all_player_states(&mut self, state: PlayerState) {
        for player in self.players.values_mut() {
            player.borrow_mut().state = state;
        }
    }

    fn init_deck(&self) -> Deck<String> {
        let decks: Vec<_> = self.settings.drawing_decks_included.iter()
            .filter(|(_, i)| **i)
            .map(|(n, _)| {
                Deck::from(File::open(format!("./decks/{}.json", n)).expect("file")).expect("expect")
            })
            .collect();
        let mut combined_deck = Deck::from_decks(decks);
        combined_deck.add_card("rabbit".to_string());
        combined_deck.shuffle();
        combined_deck
    }

    fn start_next_round(&mut self) {
        let mut imprint_map: HashMap<Uuid, Option<Rc<Drawing>>> = HashMap::new();
        if let Some(round) = self.get_current_round() {
            imprint_map = round.get_data().iter()
                .map(|(player_id, data)| {
                    let imprint = imprint_selector::random(data.drawing.clone(), data.imprint.clone(), 3);
                    (player_id.clone(), imprint)
                })
                .collect();
        }

        self.curr_round = Some(self.curr_round.map_or(1, |v| v + 1));
        self.rounds.push(
            Round::new(
                self.players.clone(),
                &mut self.drawing_suggestions_deck.as_mut().expect("Deck should be init after start_game"),
                &imprint_map,
            ));

        self.state = GameState::DrawingPhase;
        self.set_all_player_states(PlayerState::Drawing);
        self.broadcast_lobby_update();
        self.send_drawing_parameters();
    }

    fn go_to_voting_phase_if_drawing_is_done(&mut self) {
        let round = self.get_current_round().expect("round should exist");
        if round.is_done_drawing() {
            self.send_voting_ballots();
            self.state = GameState::VotingPhase;
            self.set_all_player_states(PlayerState::Voting);
            self.broadcast_lobby_update()
        }
    }

    fn finish_round_if_voting_phase_is_done(&mut self) {
        let round = self.get_current_round().expect("round should exist");
        if round.is_done_voting() {
        let scores = round.get_scores();
            self.add_to_score(&scores);

            // this is the last round, go to results
            if self.curr_round == Some(self.settings.rounds) {
                self.state = GameState::Results;
                self.set_all_player_states(PlayerState::NotReady);


                let best_drawing = self.rounds.iter()
                    .map(|r| r.get_data())
                    .flatten()
                    .max_by(|(_, data), (_, data2)| data.votes.cmp(&data2.votes));
                let s = best_drawing.expect("should have drawings").1;
                let r = Results {
                    message_name: "results".to_string(),
                    highest_rated_drawing: s.drawing.as_ref().expect("drawing should exist").to_owned().to_vec(),
                    imprint: s.imprint.as_ref().map(|i| i.as_ref().clone()),
                    num_votes: s.votes,
                    drawing_suggestion: s.drawing_suggestion.clone(),
                };
                // Important: send scores before sending the results
                self.broadcast_lobby_update();
                self.broadcast_results(r);
            } else {
                self.start_next_round();
            }
        }

    }

    fn add_to_score(&mut self, scores: &HashMap<Uuid, i32>) {
        for (id, player) in self.players.iter_mut() {
            if let Some(score) = scores.get(id) {
                player.borrow_mut().score += score
            }
        }
    }

    fn set_player_state(&mut self, client_id: &Uuid, state: PlayerState) {
        debug!("Setting PlayerState {} {:?}", client_id, state);
        self.players.get_mut(client_id).expect("player should exist").borrow_mut().state = state;
        self.broadcast_lobby_update()
    }

}

// Messaging
impl Game{
    pub fn broadcast_lobby_update(&self) {
        info!("Broadcasting lobby update to all players");
        for player in self.players.values() {
            self.send_lobby_update_to_player(&player.borrow().client);
        }
    }

    pub fn broadcast_settings_update(&self) {
        info!("Broadcasting settings update to all players");
        for player in self.players.values() {
            self.send_settings_update_to_player(&player.borrow().client);
        }
    }

    pub fn broadcast_results(&self, results: Results) {
        info!("Broadcasting results to all players");
        for player in self.players.values() {
            self.send_results_to_player(&player.borrow().client, results.clone());
        }
    }

    pub fn send_drawing_parameters(&self) {
        let round = self.get_current_round().unwrap();
        for player in self.players.values() {
            player.borrow().client.actor_addr.do_send(
                DrawingParameters {
                    message_name: "drawing_parameters".to_string(),
                    round: self.curr_round.unwrap(),
                    drawing_suggestion:
                        round.get_drawing_suggestion(&player.borrow().client.id).unwrap().clone(),
                    imprint: round.get_imprint(&player.borrow().client.id).map(|i| (*i).clone()),
                }
            )
        }

    }

    fn send_lobby_update_to_player(&self, client_connection: &ClientConnection) {
        client_connection.actor_addr.do_send(
            LobbyUpdate {
                message_name: "lobby_update".to_string(),
                room_code: self.room_code.clone(),
                state: self.state.clone(),
                round: self.curr_round,
                players: self.players.iter().map(
                    |(id, player)|
                        player.borrow().to_view(self.is_host(id), *id == client_connection.id)
                ).collect(),
            }
        );
    }

    fn send_settings_update_to_player(&self, client_connection: &ClientConnection) {
        client_connection.actor_addr.do_send(
            GameSettingsUpdate {
                message_name: "game_settings_update".to_string(),
                settings: self.settings.clone(),
            }
        );
    }

    fn send_results_to_player(&self, client_connection: &ClientConnection, results: Results) {
        client_connection.actor_addr.do_send(results);
    }

    fn send_voting_ballots(&self) {
        let round = self.get_current_round().unwrap();
        let data = round.get_data();
        let full_ballot: HashMap<&Uuid, BallotItem> =
            data.iter().map(|(player_id, round_data)| {
                let b = BallotItem {
                    id: round_data.drawing_id.clone(),
                    suggestion: round_data.drawing_suggestion.clone(),
                    drawing: round_data.drawing.as_ref().map(|d| (**d).clone()).unwrap_or_default(),
                    imprint: round_data.imprint.as_ref().map(|i| (**i).clone()).unwrap_or_default(),
                };
                (player_id, b)
            }).collect();

        for player in self.players.values() {
            self.send_voting_ballots_to_player(&player.borrow().client, &full_ballot);
        }
    }

    fn send_voting_ballots_to_player(
        &self,
        client_connection: &ClientConnection,
        full_ballot: &HashMap<&Uuid, BallotItem>
    ) {
        let ballot: Vec<VotableBallotItem> = full_ballot.iter()
                .map(|(player_id, ballot_item)|
                    VotableBallotItem{
                        ballot_item: (*ballot_item).clone(),
                        is_voting_enabled: **player_id != client_connection.id,
                        } )
                .collect();
        client_connection.actor_addr.do_send(VotingBallot {
            message_name: "voting_ballot".to_string(),
            round: self.curr_round.unwrap(),
            ballot: ballot,
        })
    }
}