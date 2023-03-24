use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum GameMode {
    Default,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameSettings{
    pub mode: GameMode,
    pub rounds: usize,
    pub drawing_phase_time_limit_seconds: Option<u32>,
    pub voting_phase_time_limit_seconds: Option<u32>,
    pub drawing_decks_included: HashMap<String, bool>,
}