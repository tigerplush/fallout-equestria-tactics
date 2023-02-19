use bevy::prelude::{Vec3, Resource};
use fallout_equestria_tactics::axial_coordinates::AxialCoordinates;

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

#[derive(Debug)]
pub struct ColliderClickedEvent(pub Vec3);

#[derive(Resource)]
pub struct PlayerSpawnpoint(pub AxialCoordinates, pub i32);