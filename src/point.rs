use crate::Vector;

/// A point in 3D space.
#[derive(Copy, Clone, Debug, PartialEq, zerocopy::AsBytes, zerocopy::FromBytes)]
#[repr(C)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    /// Create a new point from x, y, z coordinates
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl From<Vector> for Point {
    /// Convert a vector into a point
    fn from(v: Vector) -> Self {
        Point {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}
