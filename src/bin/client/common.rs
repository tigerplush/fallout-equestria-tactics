#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ClientState {
    Init,
    Lobby,
    WaitingToConnect,
    Connected,
    LoadingLevel,
    LevelLoaded,
    Idling,
    Acting,
}
