use std::rc::Rc;

use log::info;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{player::PlayerClient, api::game_update::{GameUpdate, ClientInfo}};

// TODO, make this an actual view, and not just copy fields?
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerView {
    pub name: String,
    pub ready_state: bool
}

#[derive(Debug)]
pub struct Player{
    pub client: Rc<PlayerClient>,
    pub ready_state: bool,
}

impl Player{
    pub fn to_view(&self) -> PlayerView {
        PlayerView {
            name: self.client.name.clone(),
            ready_state: self.ready_state
        }
    }
}

#[derive(Debug)]
pub struct JoinGameError;

#[derive(Debug)]
pub struct StartGameError;

const MAX_PLAYERS: usize = 8;
const MAX_ROUNDS: usize = 5;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameState{
    WaitingForPlayers,
    DrawingPhase,
    VotingPlase,
    Results,
}

#[derive(Debug)]
pub struct Game{
    room_code: String,
    state: GameState,
    players: std::vec::Vec<Player>,
    round: usize,
    max_rounds: usize,
}

impl Game {

    pub fn new(room_code: String, host_player: Rc<PlayerClient>) -> Self {
        Game {
            room_code: room_code,
            state: GameState::WaitingForPlayers,
            players: std::vec![Player{client: host_player, ready_state: false}],
            round: 0,
            max_rounds: MAX_ROUNDS,
        }
    }

    pub fn add_player(&mut self, player: Rc<PlayerClient>) -> Result<(), JoinGameError> {
        if self.players.len() == MAX_PLAYERS {
            return Err(JoinGameError);
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
                return Err(StartGameError)
            }
            info!("Host is starting the game");
            self.state = GameState::DrawingPhase;
            self.broadcast_update();
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

    pub fn broadcast_update(&self) {
        info!("Broadcasting update to all players");
        for (i, p) in self.players.iter().enumerate() {
            self.send_game_view_to_player(&p.client, i);
        }
    }

    fn current_game_view(&self, client_info: ClientInfo) -> GameUpdate {
        GameUpdate {
            message_name: "game_update".to_string(),
            room_code: self.room_code.clone(),
            state: self.state.clone(),
            round: self.round,
            players: self.players.iter().map(|p| p.to_view()).collect(),
            client_info: client_info,
        }
    }

    fn send_game_view_to_player(&self, player: &PlayerClient, index: usize) {
        player.client_addr.do_send(
            self.current_game_view(ClientInfo{player_index: index})
        );
    }

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
}