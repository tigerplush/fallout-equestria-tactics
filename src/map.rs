use std::{ops::Add, collections::HashMap};

use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AxialCoordinates {
    pub q: i32,
    pub r: i32,
    pub elevation: i32,
}

impl AxialCoordinates {
    pub const UPPER_LEFT: AxialCoordinates = Self::new(0, -1, 0);
    pub const UPPER_RIGHT: AxialCoordinates = Self::new(1, -1, 0);
    pub const RIGHT: AxialCoordinates = Self::new(1, 0, 0);
    pub const LOWER_RIGHT: AxialCoordinates = Self::new(0, 1, 0);
    pub const LOWER_LEFT: AxialCoordinates = Self::new(-1, 1, 0);
    pub const LEFT: AxialCoordinates = Self::new(-1, 0, 0);

    pub const fn new(q: i32, r: i32, elevation: i32) -> Self {
        Self {
            q,
            r,
            elevation,
        }
    }

    pub const fn directions() -> [AxialCoordinates; 6] {
        [
            Self::UPPER_LEFT,
            Self::UPPER_RIGHT,
            Self::RIGHT,
            Self::LOWER_RIGHT,
            Self::LOWER_LEFT,
            Self::LEFT,
        ]
    }

    pub fn neighbors(self) -> [AxialCoordinates; 6] {
        [
            self + Self::UPPER_LEFT,
            self + Self::UPPER_RIGHT,
            self + Self::RIGHT,
            self + Self::LOWER_RIGHT,
            self + Self::LOWER_LEFT,
            self + Self::LEFT,
        ]
    }

    pub fn from_world(translation: Vec3) -> Self {
        let elevation = translation.y.round() as i32;
        Self::new(0, 0, elevation)
    }

    pub fn to_world(&self) -> Vec3 {
        Vec3::splat(0.0)
    }
}

impl Add<AxialCoordinates> for AxialCoordinates {
    type Output = Self;
    fn add(self, rhs: AxialCoordinates) -> Self::Output {
        Self::Output {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
            elevation: self.elevation + rhs.elevation
        }
    }
}

enum TileType {
    Passable(f32),
    Impassable,
}

struct Tile {
    coordinates: AxialCoordinates,
    tile_type: TileType,
}

#[derive(Resource)]
pub struct Map {
     tiles: HashMap<AxialCoordinates, Tile>,
     width: i32,
     depth: i32,
}

impl Map {
    pub fn generate(width: i32, depth: i32) -> Self {
        let mut tiles = HashMap::new();

        for w in -width..width {
            for d in -depth..depth {
                let tile = Tile {
                    coordinates: AxialCoordinates::new(w, d,0),
                    tile_type: TileType::Passable(1.0),
                };
                tiles.insert(tile.coordinates, tile);
            }
        }

        Self {
            tiles,
            width,
            depth,
        }
    }
}