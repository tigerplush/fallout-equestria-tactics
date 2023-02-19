use std::ops::Add;

use serde::{Deserialize, Serialize};

use bevy::prelude::*;

use crate::{cube_coordinates::CubeCoordinates, common::HEX_SIZE};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AxialCoordinates {
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
        Self { q, r }
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

    pub fn from_world(translation: Vec3) -> (Self, i32) {
        let q = (3.0_f32.sqrt() / 3.0 * translation.x - 1.0/3.0 * translation.z) / HEX_SIZE;
        let r = (2.0/3.0 * translation.z) / HEX_SIZE;
        let elevation = translation.y.round() as i32;
        (Self::axial_round(q, r), elevation)
    }

    pub fn to_world(&self, elevation: i32) -> Vec3 {
        let x = HEX_SIZE * (3.0_f32.sqrt() * self.q as f32 + 3.0_f32.sqrt()/2.0 * self.r as f32);
        let z = HEX_SIZE * (3.0/2.0 * self.r as f32);
        Vec3::new(
            x,
            elevation as f32,
            z
        )
    }

    fn axial_round(q: f32, r: f32) -> Self {
        CubeCoordinates::from((q, r)).round().into()
    }

    pub fn distance(&self, rhs: &AxialCoordinates) -> i32 {
        let this = CubeCoordinates::from(self);
        let other = CubeCoordinates::from(rhs);
        this.distance(&other)
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

impl From<CubeCoordinates<i32>> for AxialCoordinates {
    fn from(value: CubeCoordinates<i32>) -> Self {
        Self::new(
            value.q,
            value.r
        )
    }
}

#[test]
fn test_conversion() {
    let world = Vec3::new(0.0, 0.0, 0.0);
    assert_eq!(AxialCoordinates::from_world(world), (AxialCoordinates::new(0, 0), 0));
}