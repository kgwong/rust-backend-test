use log::info;


#[derive(Debug)]
pub struct JoinGameError;

const MAX_PLAYERS: usize = 8;

#[derive(Debug, Clone)]
enum GameState{
    WaitingForPlayers,
    DrawingPhase,
    VotingPlase,
    Results,
}

#[derive(Debug, Clone)]
pub struct Game{
    state: GameState,
    host_player: String,
    players: std::vec::Vec<String>,
}

impl Game {

    pub fn new(host_player: String) -> Self {
        Game {
            state: GameState::WaitingForPlayers,
            players: std::vec![host_player.clone()],
            host_player: host_player,
        }
    }

    pub fn add_player(&mut self, player: String) -> std::result::Result<(), JoinGameError> {
        if self.players.len() == MAX_PLAYERS {
            return Err(JoinGameError);
        }

        self.players.push(player);
        info!("CurrentPlayers: {:?}", self.players);
        return Ok(());
    }
}