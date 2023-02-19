#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ClientState {
    Init,
    Lobby,
    LoadingLevel,
    LevelLoaded,
    SpawnPhase,
    Idling,
    Acting,
}
