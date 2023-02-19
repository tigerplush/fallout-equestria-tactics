use std::{collections::HashMap};

use bevy::prelude::*;

use crate::axial_coordinates::AxialCoordinates;



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
