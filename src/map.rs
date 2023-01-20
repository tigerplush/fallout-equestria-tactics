use std::{ops::Add, collections::HashMap};

use bevy::prelude::*;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct AxialCoordinates {
    pub q: i32,
    pub r: i32,
}

impl AxialCoordinates {
    pub const UPPER_LEFT: AxialCoordinates = Self::new(0, -1);
    pub const UPPER_RIGHT: AxialCoordinates = Self::new(1, -1);
    pub const RIGHT: AxialCoordinates = Self::new(1, 0);
    pub const LOWER_RIGHT: AxialCoordinates = Self::new(0, 1);
    pub const LOWER_LEFT: AxialCoordinates = Self::new(-1, 1);
    pub const LEFT: AxialCoordinates = Self::new(-1, 0);

    pub const fn new(q: i32, r: i32) -> Self {
        Self {
            q,
            r,
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
}

impl Add<AxialCoordinates> for AxialCoordinates {
    type Output = Self;
    fn add(self, rhs: AxialCoordinates) -> Self::Output {
        Self::Output {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
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
                    coordinates: AxialCoordinates::new(w, d),
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