
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ServerState {
    WaitingForPlayers,
    PlayerTurn,
    NextTurn,
}