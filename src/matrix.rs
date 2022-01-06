use crate::{NearlyEqual, Point, Vector3, Vector4};

/// A 4x4 matrix, suitable for 3D transformations.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Matrix4(pub [Vector4; 4]);

impl Matrix4 {
    /// A new matrix from a 1D array.
    pub const fn from_1d_array(a: [f32; 16]) -> Self {
        Self([
            Vector4::new(a[0], a[1], a[2], a[3]),
            Vector4::new(a[4], a[5], a[6], a[4]),
            Vector4::new(a[8], a[9], a[10], a[11]),
            Vector4::new(a[12], a[13], a[14], a[15]),
        ])
    }

    /// A new matrix from a 2D array.
    pub const fn from_2d_array(a: [[f32; 4]; 4]) -> Self {
        Self([
            Vector4::new(a[0][0], a[0][1], a[0][2], a[0][3]),
            Vector4::new(a[1][0], a[1][1], a[1][2], a[1][3]),
            Vector4::new(a[2][0], a[2][1], a[2][2], a[2][3]),
            Vector4::new(a[3][0], a[3][1], a[3][2], a[3][3]),
        ])
    }

    /// The identity matrix.
    pub const fn identity() -> Self {
        Self([
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        ])
    }

    /// A matrix composed entirely of zeroes.
    pub const fn zero() -> Self {
        Self([
            Vector4::new(0.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 0.0),
        ])
    }

    /// A look-at matrix suitable for positioning a camera.
    pub fn look_at(eye: Point, target: Point, up: Vector3) -> Self {
        let z_axis = (target - eye).normalized();
        let x_axis = z_axis.cross(up).normalized();
        let y_axis = x_axis.cross(z_axis);

        let eye_vec = eye.into();

        Self([
            Vector4::new(x_axis.x, y_axis.x, -z_axis.x, 0.0),
            Vector4::new(x_axis.y, y_axis.y, -z_axis.y, 0.0),
            Vector4::new(x_axis.z, y_axis.z, -z_axis.z, 0.0),
            Vector4::new(
                -x_axis.dot(eye_vec),
                -y_axis.dot(eye_vec),
                z_axis.dot(eye_vec),
                1.0,
            ),
        ])
    }

    /// A perspective matrix suitable for use as a camera projection.
    pub fn perspective(aspect_ratio: f32, fov_radians: f32, znear: f32, zfar: f32) -> Self {
        let f = 1.0 / (fov_radians / 2.0).tan();

        Self([
            Vector4::new(f / aspect_ratio, 0.0, 0.0, 0.0),
            Vector4::new(0.0, f, 0.0, 0.0),
            Vector4::new(0.0, 0.0, (zfar + znear) / (znear - zfar), -1.0),
            Vector4::new(0.0, 0.0, (2.0 * zfar * znear) / (znear - zfar), 0.0),
        ])
    }

    /// A matrix that translates by the given vector.
    pub fn translation(v: Vector3) -> Self {
        Self([
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(v.x, v.y, v.z, 1.0),
        ])
    }

    /// A matrix that rotates around the x-axis.
    pub fn rotation_x(angle_radians: f32) -> Self {
        Self([
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, angle_radians.cos(), -angle_radians.sin(), 0.0),
            Vector4::new(0.0, angle_radians.sin(), angle_radians.cos(), 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        ])
    }

    /// A matrix that rotates around the y-axis.
    pub fn rotation_y(angle_radians: f32) -> Self {
        Self([
            Vector4::new(angle_radians.cos(), 0.0, angle_radians.sin(), 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(-angle_radians.sin(), 0.0, angle_radians.cos(), 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        ])
    }

    /// A matrix that rotates around the z-axis.
    pub fn rotation_z(angle_radians: f32) -> Self {
        Self([
            Vector4::new(angle_radians.cos(), -angle_radians.sin(), 0.0, 0.0),
            Vector4::new(angle_radians.sin(), angle_radians.cos(), 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        ])
    }

    /// A matrix that scales uniformly in all dimensions.
    pub fn uniform_scale(scale: f32) -> Self {
        Self([
            Vector4::new(scale, 0.0, 0.0, 0.0),
            Vector4::new(0.0, scale, 0.0, 0.0),
            Vector4::new(0.0, 0.0, scale, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        ])
    }

    /// Obtain the specified row vector of this matrix.
    pub fn row(&self, i: usize) -> Vector4 {
        Vector4::new(self.0[0][i], self.0[1][i], self.0[2][i], self.0[3][i])
    }
    /// Obtain the specified column vector of this matrix.
    pub fn column(&self, i: usize) -> Vector4 {
        self.0[i]
    }

    /// The transpose of this matrix (i.e. this matrix flipped along the diagonal)
    pub fn transpose(&self) -> Self {
        let mut r = Self::zero();

        for i in 0..4 {
            for j in 0..4 {
                r.0[i][j] = self.0[j][i];
            }
        }

        r
    }

    /// The inverse of this matrix.
    pub fn invert(&self) -> Self {
        let mut inv = Matrix4::zero();

        inv.0[0][0] = self.0[1][1] * self.0[2][2] * self.0[3][3]
            - self.0[1][1] * self.0[2][3] * self.0[3][2]
            - self.0[2][1] * self.0[1][2] * self.0[3][3]
            + self.0[2][1] * self.0[1][3] * self.0[3][2]
            + self.0[3][1] * self.0[1][2] * self.0[2][3]
            - self.0[3][1] * self.0[1][3] * self.0[2][2];

        inv.0[1][0] = -self.0[1][0] * self.0[2][2] * self.0[3][3]
            + self.0[1][0] * self.0[2][3] * self.0[3][2]
            + self.0[2][0] * self.0[1][2] * self.0[3][3]
            - self.0[2][0] * self.0[1][3] * self.0[3][2]
            - self.0[3][0] * self.0[1][2] * self.0[2][3]
            + self.0[3][0] * self.0[1][3] * self.0[2][2];

        inv.0[2][0] = self.0[1][0] * self.0[2][1] * self.0[3][3]
            - self.0[1][0] * self.0[2][3] * self.0[3][1]
            - self.0[2][0] * self.0[1][1] * self.0[3][3]
            + self.0[2][0] * self.0[1][3] * self.0[3][1]
            + self.0[3][0] * self.0[1][1] * self.0[2][3]
            - self.0[3][0] * self.0[1][3] * self.0[2][1];

        inv.0[3][0] = -self.0[1][0] * self.0[2][1] * self.0[3][2]
            + self.0[1][0] * self.0[2][2] * self.0[3][1]
            + self.0[2][0] * self.0[1][1] * self.0[3][2]
            - self.0[2][0] * self.0[1][2] * self.0[3][1]
            - self.0[3][0] * self.0[1][1] * self.0[2][2]
            + self.0[3][0] * self.0[1][2] * self.0[2][1];

        inv.0[0][1] = -self.0[0][1] * self.0[2][2] * self.0[3][3]
            + self.0[0][1] * self.0[2][3] * self.0[3][2]
            + self.0[2][1] * self.0[0][2] * self.0[3][3]
            - self.0[2][1] * self.0[0][3] * self.0[3][2]
            - self.0[3][1] * self.0[0][2] * self.0[2][3]
            + self.0[3][1] * self.0[0][3] * self.0[2][2];

        inv.0[1][1] = self.0[0][0] * self.0[2][2] * self.0[3][3]
            - self.0[0][0] * self.0[2][3] * self.0[3][2]
            - self.0[2][0] * self.0[0][2] * self.0[3][3]
            + self.0[2][0] * self.0[0][3] * self.0[3][2]
            + self.0[3][0] * self.0[0][2] * self.0[2][3]
            - self.0[3][0] * self.0[0][3] * self.0[2][2];

        inv.0[2][1] = -self.0[0][0] * self.0[2][1] * self.0[3][3]
            + self.0[0][0] * self.0[2][3] * self.0[3][1]
            + self.0[2][0] * self.0[0][1] * self.0[3][3]
            - self.0[2][0] * self.0[0][3] * self.0[3][1]
            - self.0[3][0] * self.0[0][1] * self.0[2][3]
            + self.0[3][0] * self.0[0][3] * self.0[2][1];

        inv.0[3][1] = self.0[0][0] * self.0[2][1] * self.0[3][2]
            - self.0[0][0] * self.0[2][2] * self.0[3][1]
            - self.0[2][0] * self.0[0][1] * self.0[3][2]
            + self.0[2][0] * self.0[0][2] * self.0[3][1]
            + self.0[3][0] * self.0[0][1] * self.0[2][2]
            - self.0[3][0] * self.0[0][2] * self.0[2][1];

        inv.0[0][2] = self.0[0][1] * self.0[1][2] * self.0[3][3]
            - self.0[0][1] * self.0[1][3] * self.0[3][2]
            - self.0[1][1] * self.0[0][2] * self.0[3][3]
            + self.0[1][1] * self.0[0][3] * self.0[3][2]
            + self.0[3][1] * self.0[0][2] * self.0[1][3]
            - self.0[3][1] * self.0[0][3] * self.0[1][2];

        inv.0[1][2] = -self.0[0][0] * self.0[1][2] * self.0[3][3]
            + self.0[0][0] * self.0[1][3] * self.0[3][2]
            + self.0[1][0] * self.0[0][2] * self.0[3][3]
            - self.0[1][0] * self.0[0][3] * self.0[3][2]
            - self.0[3][0] * self.0[0][2] * self.0[1][3]
            + self.0[3][0] * self.0[0][3] * self.0[1][2];

        inv.0[2][2] = self.0[0][0] * self.0[1][1] * self.0[3][3]
            - self.0[0][0] * self.0[1][3] * self.0[3][1]
            - self.0[1][0] * self.0[0][1] * self.0[3][3]
            + self.0[1][0] * self.0[0][3] * self.0[3][1]
            + self.0[3][0] * self.0[0][1] * self.0[1][3]
            - self.0[3][0] * self.0[0][3] * self.0[1][1];

        inv.0[3][2] = -self.0[0][0] * self.0[1][1] * self.0[3][2]
            + self.0[0][0] * self.0[1][2] * self.0[3][1]
            + self.0[1][0] * self.0[0][1] * self.0[3][2]
            - self.0[1][0] * self.0[0][2] * self.0[3][1]
            - self.0[3][0] * self.0[0][1] * self.0[1][2]
            + self.0[3][0] * self.0[0][2] * self.0[1][1];

        inv.0[0][3] = -self.0[0][1] * self.0[1][2] * self.0[2][3]
            + self.0[0][1] * self.0[1][3] * self.0[2][2]
            + self.0[1][1] * self.0[0][2] * self.0[2][3]
            - self.0[1][1] * self.0[0][3] * self.0[2][2]
            - self.0[2][1] * self.0[0][2] * self.0[1][3]
            + self.0[2][1] * self.0[0][3] * self.0[1][2];

        inv.0[1][3] = self.0[0][0] * self.0[1][2] * self.0[2][3]
            - self.0[0][0] * self.0[1][3] * self.0[2][2]
            - self.0[1][0] * self.0[0][2] * self.0[2][3]
            + self.0[1][0] * self.0[0][3] * self.0[2][2]
            + self.0[2][0] * self.0[0][2] * self.0[1][3]
            - self.0[2][0] * self.0[0][3] * self.0[1][2];

        inv.0[2][3] = -self.0[0][0] * self.0[1][1] * self.0[2][3]
            + self.0[0][0] * self.0[1][3] * self.0[2][1]
            + self.0[1][0] * self.0[0][1] * self.0[2][3]
            - self.0[1][0] * self.0[0][3] * self.0[2][1]
            - self.0[2][0] * self.0[0][1] * self.0[1][3]
            + self.0[2][0] * self.0[0][3] * self.0[1][1];

        inv.0[3][3] = self.0[0][0] * self.0[1][1] * self.0[2][2]
            - self.0[0][0] * self.0[1][2] * self.0[2][1]
            - self.0[1][0] * self.0[0][1] * self.0[2][2]
            + self.0[1][0] * self.0[0][2] * self.0[2][1]
            + self.0[2][0] * self.0[0][1] * self.0[1][2]
            - self.0[2][0] * self.0[0][2] * self.0[1][1];

        let mut det = self.0[0][0] * inv.0[0][0]
            + self.0[0][1] * inv.0[1][0]
            + self.0[0][2] * inv.0[2][0]
            + self.0[0][3] * inv.0[3][0];
        det = 1.0 / det;

        for i in 0..4 {
            for j in 0..4 {
                inv.0[i][j] *= det;
            }
        }

        inv
    }

    pub fn as_slice(&self) -> &[f32] {
        unsafe {
            std::slice::from_raw_parts(
                &self.0[0][0],
                std::mem::size_of::<Self>() / std::mem::size_of::<f32>(),
            )
        }
    }
}

impl NearlyEqual for &Matrix4 {
    fn nearly_equals(self, rhs: Self) -> bool {
        for i in 0..4 {
            if !self.0[i].nearly_equals(&rhs.0[i]) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn identity() {
        let m = Matrix4::identity();
        let p = Point::new(1.0, 2.0, 3.0);

        assert_eq!(p, m * p);
        assert_eq!(m, m.transpose());
        assert_eq!(m, m.invert());
    }

    #[test]
    fn invert() {
        let m = Matrix4::from_2d_array([
            [3.0, 2.0, 1.0, 1.0],
            [2.0, 3.0, 2.0, 2.0],
            [1.0, 2.0, 3.0, 3.0],
            [0.0, 1.0, 1.0, 0.0],
        ]);

        assert_eq!(m.invert() * m, Matrix4::identity());

        let n = Matrix4([
            Vector4 {
                x: 0.9742785,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            Vector4 {
                x: 0.0,
                y: 1.7320507,
                z: 0.0,
                w: 0.0,
            },
            Vector4 {
                x: 0.0,
                y: 0.0,
                z: -1.0002,
                w: -1.0,
            },
            Vector4 {
                x: 0.0,
                y: 0.0,
                z: -2.0002,
                w: 0.0,
            },
        ]);
        let inverse = Matrix4([
            Vector4 {
                x: 1.0264006,
                y: -0.0,
                z: -0.0,
                w: -0.0,
            },
            Vector4 {
                x: -0.0,
                y: 0.5773504,
                z: -0.0,
                w: -0.0,
            },
            Vector4 {
                x: -0.0,
                y: -0.0,
                z: -0.0,
                w: -0.49995005,
            },
            Vector4 {
                x: -0.0,
                y: -0.0,
                z: -1.0000001,
                w: 0.50005007,
            },
        ]);
        assert_eq!(n.invert(), inverse);
    }

    #[test]
    fn translate() {
        let m = Matrix4::translation(Vector3::new(10.0, 1.0, 0.0));
        assert_eq!(m * Point::zero(), Point::new(10.0, 1.0, 0.0));

        let n = Matrix4::translation(Vector3::new(-2.0, -5.0, 0.0));
        assert_eq!(n * Point::zero(), Point::new(-2.0, -5.0, 0.0));

        let t = m * n;
        assert_eq!(t * Point::zero(), Point::new(8.0, -4.0, 0.0));
    }

    #[test]
    fn slice() {
        let a = [
            3.0, 2.0, 1.0, 1.0, 2.0, 3.0, 2.0, 2.0, 1.0, 2.0, 3.0, 3.0, 0.0, 1.0, 1.0, 0.0,
        ];
        let m = Matrix4::from_1d_array(a);

        assert_eq!(m.as_slice(), &a);
    }
}
