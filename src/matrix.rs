use crate::{Point, Vector};

/// A 4x4 matrix, suitable for 3D transformations.
#[derive(Copy, Clone, Debug, PartialEq, zerocopy::AsBytes, zerocopy::FromBytes)]
#[repr(C)]
pub struct Matrix(pub [[f32; 4]; 4]);

impl Matrix {
    /// The identity matrix.
    pub fn identity() -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// A matrix composed entirely of zeroes.
    pub fn zero() -> Self {
        Self([
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ])
    }

    /// A look-at matrix suitable for positioning a camera.
    pub fn look_at(eye: Point, target: Point, up: Vector) -> Self {
        let z_axis = (target - eye).normalized();
        let x_axis = z_axis.cross(up).normalized();
        let y_axis = x_axis.cross(z_axis);

        let eye_vec = eye.into();

        Self([
            [x_axis.x, y_axis.x, -z_axis.x, 0.0],
            [x_axis.y, y_axis.y, -z_axis.y, 0.0],
            [x_axis.z, y_axis.z, -z_axis.z, 0.0],
            [
                -x_axis.dot(eye_vec),
                -y_axis.dot(eye_vec),
                z_axis.dot(eye_vec),
                1.0,
            ],
        ])
    }

    /// A perspective matrix suitable for use as a camera projection.
    pub fn perspective(aspect_ratio: f32, fov_radians: f32, znear: f32, zfar: f32) -> Self {
        let f = 1.0 / (fov_radians / 2.0).tan();

        Self([
            [f / aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (zfar + znear) / (znear - zfar), -1.0],
            [0.0, 0.0, (2.0 * zfar * znear) / (znear - zfar), 0.0],
        ])
    }

    /// A matrix that translates by the given vector.
    pub fn translation(v: Vector) -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [v.x, v.y, v.z, 1.0],
        ])
    }

    /// A matrix that rotates around the x-axis.
    pub fn rotation_x(angle_radians: f32) -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, angle_radians.cos(), -angle_radians.sin(), 0.0],
            [0.0, angle_radians.sin(), angle_radians.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// A matrix that rotates around the y-axis.
    pub fn rotation_y(angle_radians: f32) -> Self {
        Self([
            [angle_radians.cos(), 0.0, angle_radians.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-angle_radians.sin(), 0.0, angle_radians.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// A matrix that rotates around the z-axis.
    pub fn rotation_z(angle_radians: f32) -> Self {
        Self([
            [angle_radians.cos(), -angle_radians.sin(), 0.0, 0.0],
            [angle_radians.sin(), angle_radians.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// A matrix that scales uniformly in all dimensions.
    pub fn uniform_scale(scale: f32) -> Self {
        Self([
            [scale, 0.0, 0.0, 0.0],
            [0.0, scale, 0.0, 0.0],
            [0.0, 0.0, scale, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
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
        let mut inv = Matrix::zero();

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
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn identity() {
        let m = Matrix::identity();
        let p = Point::new(1.0, 2.0, 3.0);

        assert_eq!(p, m * p);
        assert_eq!(m, m.transpose());
        assert_eq!(m, m.invert());
    }

    #[test]
    fn invert() {
        let m = Matrix([
            [3.0, 2.0, 1.0, 1.0],
            [2.0, 3.0, 2.0, 2.0],
            [1.0, 2.0, 3.0, 3.0],
            [0.0, 1.0, 1.0, 0.0],
        ]);

        assert_eq!(m.invert() * m, Matrix::identity());
    }

    #[test]
    fn translate() {
        let m = Matrix::translation(Vector::new(10.0, 1.0, 0.0));
        assert_eq!(m * Point::zero(), Point::new(10.0, 1.0, 0.0));

        let n = Matrix::translation(Vector::new(-2.0, -5.0, 0.0));
        assert_eq!(n * Point::zero(), Point::new(-2.0, -5.0, 0.0));

        let t = m * n;
        assert_eq!(t * Point::zero(), Point::new(8.0, -4.0, 0.0));
    }
}
