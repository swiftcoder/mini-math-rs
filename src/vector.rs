use crate::{NearlyEqual, Point};

/// A vector in 3D space.
#[derive(Copy, Clone, Debug, PartialEq, zerocopy::AsBytes, zerocopy::FromBytes)]
#[repr(C)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    /// Create a vector from x, y, z coordinates
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// The additive identity
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// The multiplicative identity
    pub fn one() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    /// Compute the dot product between this vector and another
    pub fn dot(&self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Compute the cross product between this vector and another.
    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    /// The length of this vector squared. Note that this avoids an expensive square root.
    pub fn magnitude_squared(&self) -> f32 {
        self.dot(*self)
    }

    /// The length of this vector. Note that this involves an expensive square root.
    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    /// Normalize this vector to unit length. Note that this involves an expensive square root.
    pub fn normalized(&self) -> Self {
        let d = self.magnitude();
        if d > 0.0 {
            let d = 1.0 / d;
            *self * d
        } else {
            *self
        }
    }
}

impl From<Point> for Vector {
    /// Convert a point into a vector
    fn from(p: Point) -> Self {
        Vector {
            x: p.x,
            y: p.y,
            z: p.z,
        }
    }
}

impl NearlyEqual for &Vector {
    fn nearly_equals(self, rhs: Self) -> bool {
        self.x.nearly_equals(rhs.x) && self.y.nearly_equals(rhs.y) && self.z.nearly_equals(rhs.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn products() {
        let a = Vector::new(3.0, -5.0, 4.0);
        let b = Vector::new(2.0, 6.0, 5.0);

        assert!(a.dot(b).nearly_equals(-4.0));
        assert_eq!(a.cross(b), Vector::new(-49.0, -7.0, 28.0));
    }
}
