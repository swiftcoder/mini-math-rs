use crate::{Point, Vector};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix(pub [[f32; 4]; 4]);

impl Matrix {
    pub fn identity() -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn zero() -> Self {
        Self([
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ])
    }

    pub fn look_at(eye: Point, target: Point, up: Vector) -> Self {
        let z_axis = (target - eye).normalized();
        let x_axis = z_axis.cross(&up).normalized();
        let y_axis = x_axis.cross(&z_axis);

        let eye_vec = eye - Point::new(0.0, 0.0, 0.0);

        Self([
            [x_axis.x, y_axis.x, -z_axis.x, 0.0],
            [x_axis.y, y_axis.y, -z_axis.y, 0.0],
            [x_axis.z, y_axis.z, -z_axis.z, 0.0],
            [
                -x_axis.dot(&eye_vec),
                -y_axis.dot(&eye_vec),
                z_axis.dot(&eye_vec),
                1.0,
            ],
        ])
    }

    pub fn perspective(aspect_ratio: f32, fov: f32, znear: f32, zfar: f32) -> Self {
        let f = 1.0 / (fov / 2.0).tan();

        Self([
            [f / aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (zfar + znear) / (znear - zfar), -1.0],
            [0.0, 0.0, (2.0 * zfar * znear) / (znear - zfar), 0.0],
        ])
    }

    pub fn translation(v: Vector) -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [v.x, v.y, v.z, 1.0],
        ])
    }

    pub fn rotation_x(angle: f32) -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, angle.cos(), -angle.sin(), 0.0],
            [0.0, angle.sin(), angle.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_y(angle: f32) -> Self {
        Self([
            [angle.cos(), 0.0, angle.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-angle.sin(), 0.0, angle.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_z(angle: f32) -> Self {
        Self([
            [angle.cos(), -angle.sin(), 0.0, 0.0],
            [angle.sin(), angle.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn transpose(&self) -> Self {
        let mut r = Self::zero();

        for i in 0..4 {
            for j in 0..4 {
                r.0[i][j] = self.0[j][i];
            }
        }

        r
    }

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
}
