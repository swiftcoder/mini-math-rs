use crate::{Matrix4, Point, Vector2, Vector3, Vector4};

impl std::ops::Mul<&Matrix4> for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: &Matrix4) -> Self {
        self * *rhs
    }
}

impl std::ops::Mul for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = Self::zero();

        for i in 0..4 {
            for j in 0..4 {
                result.0[i][j] = rhs.0[i][0] * self.0[0][j]
                    + rhs.0[i][1] * self.0[1][j]
                    + rhs.0[i][2] * self.0[2][j]
                    + rhs.0[i][3] * self.0[3][j];
            }
        }

        result
    }
}

impl std::ops::Mul<Point> for Matrix4 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        Point::new(
            self.0[0][0] * rhs.x + self.0[1][0] * rhs.y + self.0[2][0] * rhs.z + self.0[3][0],
            self.0[0][1] * rhs.x + self.0[1][1] * rhs.y + self.0[2][1] * rhs.z + self.0[3][1],
            self.0[0][2] * rhs.x + self.0[1][2] * rhs.y + self.0[2][2] * rhs.z + self.0[3][2],
        )
    }
}

impl std::ops::Mul<Vector3> for Matrix4 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3::new(
            self.0[0][0] * rhs.x + self.0[1][0] * rhs.y + self.0[2][0] * rhs.z,
            self.0[0][1] * rhs.x + self.0[1][1] * rhs.y + self.0[2][1] * rhs.z,
            self.0[0][2] * rhs.x + self.0[1][2] * rhs.y + self.0[2][2] * rhs.z,
        )
    }
}

impl std::ops::Mul<Vector4> for Matrix4 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Vector4 {
        Vector4::new(
            self.row(0).dot(rhs),
            self.row(1).dot(rhs),
            self.row(2).dot(rhs),
            self.row(3).dot(rhs),
        )
    }
}

impl std::ops::Mul<Matrix4> for Point {
    type Output = Point;

    fn mul(self, rhs: Matrix4) -> Point {
        Point::new(
            Vector3::from_scalar(self.x).dot(Vector3::from(rhs.column(0))),
            Vector3::from_scalar(self.y).dot(Vector3::from(rhs.column(1))),
            Vector3::from_scalar(self.z).dot(Vector3::from(rhs.column(2))),
        )
    }
}

impl std::ops::Mul<Matrix4> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Matrix4) -> Vector3 {
        Vector3::new(
            Vector3::from_scalar(self.x).dot(Vector3::from(rhs.column(0))),
            Vector3::from_scalar(self.y).dot(Vector3::from(rhs.column(1))),
            Vector3::from_scalar(self.z).dot(Vector3::from(rhs.column(2))),
        )
    }
}

impl std::ops::Mul<Matrix4> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: Matrix4) -> Vector4 {
        Vector4::new(
            Vector4::from_scalar(self.x).dot(rhs.column(0)),
            Vector4::from_scalar(self.y).dot(rhs.column(1)),
            Vector4::from_scalar(self.z).dot(rhs.column(2)),
            Vector4::from_scalar(self.w).dot(rhs.column(3)),
        )
    }
}

macro_rules! vector_op {
    (impl $trait:ident<$other_type: ty> for $type:ty {
        fn $op_fn:ident -> $result_type:ty, $op:tt { $($field:ident),+ }
    }) => {
        impl std::ops::$trait<$other_type> for $type {
            type Output = $result_type;

            fn $op_fn(self, rhs: $other_type) -> $result_type {
                <$result_type>::new($(self.$field $op rhs.$field),+)
            }
        }
    };
}

macro_rules! vector_assign_op {
    (impl $trait:ident<$other_type: ty> for $type:ty {
        fn $op_fn:ident, $op:tt { $($field:ident),+ }
    }) => {
        impl std::ops::$trait<$other_type> for $type {
            fn $op_fn(&mut self, rhs: $other_type) {
                $(self.$field $op rhs.$field);+
            }
        }
    };
}

vector_op!(impl Add<Vector2> for Vector2 { fn add -> Vector2, + {x, y} });
vector_op!(impl Sub<Vector2> for Vector2 { fn sub -> Vector2, - {x, y} });
vector_op!(impl Mul<Vector2> for Vector2 { fn mul -> Vector2, * {x, y} });
vector_op!(impl Div<Vector2> for Vector2 { fn div -> Vector2, / {x, y} });
vector_assign_op!(impl AddAssign<Vector2> for Vector2 { fn add_assign, += {x, y} });
vector_assign_op!(impl SubAssign<Vector2> for Vector2 { fn sub_assign, -= {x, y} });
vector_assign_op!(impl MulAssign<Vector2> for Vector2 { fn mul_assign, *= {x, y} });
vector_assign_op!(impl DivAssign<Vector2> for Vector2 { fn div_assign, /= {x, y} });

vector_op!(impl Add<Vector3> for Vector3 { fn add -> Vector3, + {x, y, z} });
vector_op!(impl Sub<Vector3> for Vector3 { fn sub -> Vector3, - {x, y, z} });
vector_op!(impl Mul<Vector3> for Vector3 { fn mul -> Vector3, * {x, y, z} });
vector_op!(impl Div<Vector3> for Vector3 { fn div -> Vector3, / {x, y, z} });
vector_assign_op!(impl AddAssign<Vector3> for Vector3 { fn add_assign, += {x, y, z} });
vector_assign_op!(impl SubAssign<Vector3> for Vector3 { fn sub_assign, -= {x, y, z} });
vector_assign_op!(impl MulAssign<Vector3> for Vector3 { fn mul_assign, *= {x, y, z} });
vector_assign_op!(impl DivAssign<Vector3> for Vector3 { fn div_assign, /= {x, y, z} });

vector_op!(impl Add<Vector4> for Vector4 { fn add -> Vector4, + {x, y, z, w} });
vector_op!(impl Sub<Vector4> for Vector4 { fn sub -> Vector4, - {x, y, z, w} });
vector_op!(impl Mul<Vector4> for Vector4 { fn mul -> Vector4, * {x, y, z, w} });
vector_op!(impl Div<Vector4> for Vector4 { fn div -> Vector4, / {x, y, z, w} });
vector_assign_op!(impl AddAssign<Vector4> for Vector4 { fn add_assign, += {x, y, z, w} });
vector_assign_op!(impl SubAssign<Vector4> for Vector4 { fn sub_assign, -= {x, y, z, w} });
vector_assign_op!(impl MulAssign<Vector4> for Vector4 { fn mul_assign, *= {x, y, z, w} });
vector_assign_op!(impl DivAssign<Vector4> for Vector4 { fn div_assign, /= {x, y, z, w} });

vector_op!(impl Add<Vector3> for Point { fn add -> Point, + {x, y, z} });
vector_op!(impl Sub<Vector3> for Point { fn sub -> Point, - {x, y, z} });
vector_op!(impl Sub<Point> for Point { fn sub -> Vector3, - {x, y, z} });
vector_assign_op!(impl AddAssign<Vector3> for Point { fn add_assign, += {x, y, z} });
vector_assign_op!(impl SubAssign<Vector3> for Point { fn sub_assign, -= {x, y, z} });
