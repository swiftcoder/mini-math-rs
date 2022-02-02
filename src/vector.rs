use crate::NearlyEqual;

/// A vector in 2D space.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

/// A vector in 3D space.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// A point in 3D space.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// A homogeneous vector in 3D space.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

macro_rules! implement_operator {
    // Binary operator
    (impl $Op:ident<$S:ident> for $T:ident {
        fn $op:ident($x:ident, $s:ident) -> $Output:ty $body:block
    }) => {
        impl std::ops::$Op<$S> for $T {
            type Output = $Output;

            fn $op($x, $s: $S) -> Self::Output $body
        }
    };
    // Binary assignment operator
    (impl $Op:ident<$S:ident> for $T:ident {
        fn $op:ident(&mut $x:ident, $s:ident) $body:block
    }) => {
        impl std::ops::$Op<$S> for $T {
            fn $op(&mut $x, $s: $S) $body
        }
    };
}

macro_rules! implement_vector {
    ($VectorT:ident { $($field:ident),+ }) => {
        impl $VectorT {
            /// Construct new a vector from individual coordinates
            pub const fn new($($field: f32),+) -> Self {
                Self { $($field),+ }
            }

            /// Construct new a vector where each coordinate is the same
            pub const fn from_scalar(s: f32) -> Self {
                Self { $($field: s),+ }
            }

            /// The additive identity
            pub const fn zero() -> Self {
                Self { $($field: 0.0),+ }
            }

            /// The multiplicative identity
            pub const fn one() -> Self {
                Self { $($field: 1.0),+ }
            }

            /// Compute the dot product between this vector and another
            pub fn dot(&self, rhs: Self) -> f32 {
                [$(self.$field * rhs.$field),+].iter().sum()
            }

            /// Linear interpolation between this vector and another
            pub fn lerp(&self, rhs: Self, factor: f32) -> Self {
                let t = factor.min(1.0).max(0.0);
                Self::new($(self.$field * (1.0 - t) + rhs.$field * t),+)
            }

            pub fn as_slice(&self) -> &[f32] {
                unsafe { std::slice::from_raw_parts(&self.x, std::mem::size_of::<Self>() / std::mem::size_of::<f32>()) }
            }
        }

        impl std::ops::Neg for $VectorT {
            type Output = $VectorT;
            fn neg(self) -> $VectorT { $VectorT::new($(-self.$field),+) }
        }

        implement_operator!(impl Add<f32> for $VectorT {
            fn add(self, t) -> $VectorT { $VectorT::new($(self.$field + t),+) }
        });
        implement_operator!(impl Sub<f32> for $VectorT {
            fn sub(self, t) -> $VectorT { $VectorT::new($(self.$field - t),+) }
        });
        implement_operator!(impl Mul<f32> for $VectorT {
            fn mul(self, t) -> $VectorT { $VectorT::new($(self.$field * t),+) }
        });
        implement_operator!(impl Div<f32> for $VectorT {
            fn div(self, t) -> $VectorT { $VectorT::new($(self.$field / t),+) }
        });

        implement_operator!(impl AddAssign<f32> for $VectorT {
            fn add_assign(&mut self, t) { $(self.$field += t);+ }
        });
        implement_operator!(impl SubAssign<f32> for $VectorT {
            fn sub_assign(&mut self, t) { $(self.$field -= t);+ }
        });
        implement_operator!(impl MulAssign<f32> for $VectorT {
            fn mul_assign(&mut self, t) { $(self.$field *= t);+ }
        });
        implement_operator!(impl DivAssign<f32> for $VectorT {
            fn div_assign(&mut self, t) { $(self.$field /= t);+ }
        });

        implement_operator!(impl Mul<$VectorT> for f32 {
            fn mul(self, t) -> $VectorT { $VectorT::new($(self * t.$field),+) }
        });
        implement_operator!(impl Div<$VectorT> for f32 {
            fn div(self, t) -> $VectorT { $VectorT::new($(self / t.$field),+) }
        });

        impl std::ops::Index<usize> for $VectorT {
            type Output = f32;
            fn index(&self, i: usize) -> &f32 {
                [$(&self.$field),+][i]
            }
        }

        impl std::ops::IndexMut<usize> for $VectorT {
            fn index_mut(&mut self, i: usize) -> &mut f32 {
                [$(&mut self.$field),+][i]
            }
        }

        impl NearlyEqual for &$VectorT {
            fn nearly_equals(self, rhs: Self) -> bool {
                $(self.$field.nearly_equals(rhs.$field))&&+
            }
        }
    }
}

implement_vector!(Vector2 { x, y });
implement_vector!(Vector3 { x, y, z });
implement_vector!(Point { x, y, z });
implement_vector!(Vector4 { x, y, z, w });

impl Vector2 {
    /// Compute a cross product between this vector and another.
    /// This treats both inputs as 3D vectors with a z-component of zero,
    /// performs the normal 3D cross product, and returns only the resulting z-component.
    pub fn cross(&self, rhs: Self) -> f32 {
        self.x * rhs.y - self.y * rhs.x
    }
}

impl Vector3 {
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

impl From<Point> for Vector3 {
    /// Convert a point into a vector
    fn from(p: Point) -> Self {
        Vector3 {
            x: p.x,
            y: p.y,
            z: p.z,
        }
    }
}

impl From<Vector4> for Vector3 {
    /// Convert a point into a vector
    fn from(v: Vector4) -> Self {
        Vector3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<Vector3> for Point {
    /// Convert a vector into a point
    fn from(v: Vector3) -> Self {
        Point {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<Vector4> for Point {
    /// Convert a vector into a point
    fn from(v: Vector4) -> Self {
        Point {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<Vector3> for Vector4 {
    /// Convert a point into a vector
    fn from(v: Vector3) -> Self {
        Vector4 {
            x: v.x,
            y: v.y,
            z: v.z,
            w: 0.0,
        }
    }
}

impl From<Point> for Vector4 {
    /// Convert a point into a vector
    fn from(p: Point) -> Self {
        Vector4 {
            x: p.x,
            y: p.y,
            z: p.z,
            w: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn products() {
        let a = Vector3::new(3.0, -5.0, 4.0);
        let b = Vector3::new(2.0, 6.0, 5.0);

        assert!(a.dot(b).nearly_equals(-4.0));
        assert_eq!(a.cross(b), Vector3::new(-49.0, -7.0, 28.0));
    }

    #[test]
    fn lerp() {
        let a = Vector3::new(1.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 1.0, 0.0);

        assert_eq!(a.lerp(b, 0.75), Vector3::new(0.25, 0.75, 0.0));
    }

    #[test]
    fn slice() {
        let a = Vector3::new(1.0, 2.0, 3.0);

        assert_eq!(a.as_slice(), &[1.0, 2.0, 3.0]);
    }
}
