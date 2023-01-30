
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ServerState {
    WaitingForPlayerReadiness,
    WaitingForPlayerLoadLevel,
    PlayerTurn,
    NextTurn,
}