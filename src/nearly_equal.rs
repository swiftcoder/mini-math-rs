/// Compare floating-point values using an epsilon
pub trait NearlyEqual {
    fn nearly_equals(self, rhs: Self) -> bool;
}

impl NearlyEqual for f32 {
    fn nearly_equals(self, rhs: Self) -> bool {
        (self - rhs).abs() < std::f32::EPSILON
    }
}

impl<T> NearlyEqual for Option<T>
where
    T: NearlyEqual,
{
    fn nearly_equals(self, rhs: Self) -> bool {
        match (self, rhs) {
            (Some(a), Some(b)) => a.nearly_equals(b),
            (None, None) => true,
            _ => false,
        }
    }
}

/// Asserts that two expressions are nearly equal to each other (using [`NearlyEqual`]).
#[macro_export]
macro_rules! assert_nearly_eq {
    ($left:expr, $right:expr) => {{
        if !($left.nearly_equals($right)) {
            panic!(
                "assertion failed: `(left == right)`\nleft: `{:?}`,\nright: `{:?}`",
                $left, $right
            )
        }
    }};
}
