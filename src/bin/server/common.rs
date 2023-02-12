use bevy::prelude::Component;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ServerState {
    WaitingForPlayerReadiness,
    WaitingForPlayerLoadLevel,
    SpawnPhase,
    PlayerTurn,
    NextTurn,
}

#[derive(Component)]
pub struct Player(pub u64);