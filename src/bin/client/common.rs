#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ClientState {
    WaitingToConnect,
    Connected,
    LoadLevel,
    Idling,
    Acting,
}