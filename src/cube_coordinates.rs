use std::ops::Sub;

use bevy::prelude::shape::Cube;

use crate::axial_coordinates::AxialCoordinates;

pub struct CubeCoordinates<T> {
    pub q: T,
    pub r: T,
    s: T,
}

impl<T> CubeCoordinates<T> {
    const fn new(q: T, r: T, s: T) -> Self {
        Self {
            q,
            r,
            s,
        }
    }
}

impl CubeCoordinates<i32> {
    pub fn distance(&self, rhs: &Self) -> i32 {
        std::cmp::max(std::cmp::max(self.q - rhs.q, self.r - rhs.r), self.s - rhs.s)
    }
}


impl CubeCoordinates<f32>
{
    pub fn round(&self) -> CubeCoordinates<i32> {
        let mut q = self.q.round();
        let mut r = self.r.round();
        let mut s = self.s.round();

        let q_diff = (q - self.q).abs();
        let r_diff = (r - self.r).abs();
        let s_diff = (s - self.s).abs();

        if q_diff > r_diff && q_diff > s_diff {
            q = -r - s;
        }
        else if r_diff > s_diff {
            r = -q - s;
        }
        else {
            s = -q - r;
        }

        CubeCoordinates::new(
            q as i32,
            r as i32,
            s as i32
        )
    }
}

impl From<AxialCoordinates> for CubeCoordinates<i32> {
    fn from(value: AxialCoordinates) -> Self {
        Self::new(
            value.q,
            value.r,
            -value.q - value.r
        )
    }
}

impl From<&AxialCoordinates> for CubeCoordinates<i32> {
    fn from(value: &AxialCoordinates) -> Self {
        Self::new(
            value.q,
            value.r,
            -value.q - value.r
        )
    }
}

impl From<(f32, f32)> for CubeCoordinates<f32> {
    fn from(value: (f32, f32)) -> Self {
        Self::new(
            value.0,
            value.1,
            -value.0 - value.1
        )
    }
}
impl From<(i32, i32)> for CubeCoordinates<i32> {
    fn from(value: (i32, i32)) -> Self {
        Self::new(
            value.0,
            value.1,
            -value.0 - value.1
        )
    }
}

#[test]
fn distance() {
    let a = CubeCoordinates::from((0, 0));
    let b = CubeCoordinates::from((0, 1));
    assert_eq!(a.distance(&b), 1);
}