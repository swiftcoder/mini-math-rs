#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.x * rhs.z - self.z * rhs.x,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.dot(self)
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalized(&self) -> Self {
        let d = 1.0 / self.magnitude();
        *self * d
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    macro_rules! assert_nearly_eq {
        ($a:expr, $b:expr) => {
            assert!(f32::abs($a - $b) <= std::f32::EPSILON);
        };
    }

    #[test]
    fn products() {
        let a = Vector::new(3.0, -5.0, 4.0);
        let b = Vector::new(2.0, 6.0, 5.0);

        assert_nearly_eq!(a.dot(&b), -4.0);
        assert_eq!(a.cross(&b), Vector::new(-49.0, 7.0, 28.0));
    }
}
