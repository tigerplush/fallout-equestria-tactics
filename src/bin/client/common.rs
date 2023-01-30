#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ClientState {
    WaitingToConnect,
    Connected,
    LoadingLevel,
    LevelLoaded,
    Idling,
    Acting,
}