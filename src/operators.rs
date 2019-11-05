use crate::{Matrix, Point, Vector};

impl std::ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = Self::zero();

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.0[i][j] += rhs.0[i][k] * self.0[k][j];
                }
            }
        }

        result
    }
}

impl std::ops::Mul<Point> for Matrix {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        Point::new(
            self.0[0][0] * rhs.x + self.0[1][0] * rhs.y + self.0[2][0] * rhs.z + self.0[3][0],
            self.0[0][1] * rhs.x + self.0[1][1] * rhs.y + self.0[2][1] * rhs.z + self.0[3][1],
            self.0[0][2] * rhs.x + self.0[1][2] * rhs.y + self.0[2][2] * rhs.z + self.0[3][2],
        )
    }
}

impl std::ops::Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector::new(
            self.0[0][0] * rhs.x + self.0[1][0] * rhs.y + self.0[2][0] * rhs.z,
            self.0[0][1] * rhs.x + self.0[1][1] * rhs.y + self.0[2][1] * rhs.z,
            self.0[0][2] * rhs.x + self.0[1][2] * rhs.y + self.0[2][2] * rhs.z,
        )
    }
}

macro_rules! scalar_op {
    ($trait: ident, $op_fn: ident, $type: ty, $other_type: ty, $result_type: ty, $op: tt) => {
        impl std::ops::$trait<($other_type)> for $type {
            type Output = $result_type;

            fn $op_fn(self, rhs: $other_type) -> $result_type {
                <$result_type>::new(self.x $op rhs, self.y $op rhs, self.z $op rhs)
            }
        }
    };
}

macro_rules! vector_op {
    ($trait: ident, $op_fn: ident, $type: ty, $other_type: ty, $result_type: ty, $op: tt) => {
        impl std::ops::$trait<($other_type)> for $type {
            type Output = $result_type;

            fn $op_fn(self, rhs: $other_type) -> $result_type {
                <$result_type>::new(self.x $op rhs.x, self.y $op rhs.y, self.z $op rhs.z)
            }
        }
    };
}

scalar_op!(Add, add, Point, f32, Point, +);
scalar_op!(Sub, sub, Point, f32, Point, -);
scalar_op!(Mul, mul, Point, f32, Point, *);

scalar_op!(Add, add, Vector, f32, Vector, +);
scalar_op!(Sub, sub, Vector, f32, Vector, -);
scalar_op!(Mul, mul, Vector, f32, Vector, *);

vector_op!(Add, add, Vector, Vector, Vector, +);
vector_op!(Sub, sub, Vector, Vector, Vector, -);

vector_op!(Add, add, Point, Vector, Point, +);
vector_op!(Sub, sub, Point, Vector, Point, -);
vector_op!(Sub, sub, Point, Point, Vector, -);
