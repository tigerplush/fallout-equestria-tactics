
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ServerState {
    Init,
    Lobby,
    WaitingForPlayerLoadLevel,
    SpawnPhase,
    PlayerTurn,
    NextTurn,
}
