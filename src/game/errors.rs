
#[derive(Debug)]
pub enum JoinGameError{
    ClientIsAlreadyInAGame,
    RoomDoesNotExist,
    GameFull,
    GameAlreadyStarted,
}

#[derive(Debug)]
pub enum StartGameError {
    ClientIsNotInAGame,
    ClientIsNotTheHost,
    GameAlreadyStarted,
    MinimumPlayersNotReached,
}

#[derive(Debug)]
pub enum PlayAgainError{
    ClientIsNotInAGame,
    ClientIsNotTheHost,
    GameIsNotOver,
}

#[derive(Debug)]
pub enum SetPlayerReadyError{
    ClientIsNotInAGame,
}

#[derive(Debug)]
pub enum CreateGameError{
    ClientIsAlreadyInAGame,
}

#[derive(Debug)]
pub enum UpdateGameSettingsError{
    ClientIsNotInAGame,
    ClientIsNotTheHost,
    GameAlreadyStarted,
    InvalidNumRounds,
    InvalidDrawingTimeLimit,
    InvalidVotingTimeLimit,
    DeckDoesNotExist,
    SettingRemovesAllDecks,
}

#[derive(Debug)]
pub enum SubmitDrawingError{
    ClientIsNotInAGame,
    DrawingSubmittedForWrongRound,
    DrawingWasAlreadySubmitted,
}

#[derive(Debug)]
pub enum SubmitVoteError{
    ClientIsNotInAGame,
    GameHasNotStarted,
    MaximumVotesExceeded,
    ClientVotedForSelf,
    InvalidDrawingId,
}