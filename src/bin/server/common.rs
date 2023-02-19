
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ServerState {
    /// This is the base state the server starts in, it is responsible for loading everything that isn't plugin related.
    Init,
    /// 
    Lobby,
    WaitingForPlayerLoadLevel,
    SpawnPhase,
    PlayerTurn,
    NextTurn,
}
