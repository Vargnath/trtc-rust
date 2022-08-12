pub mod camera;
pub mod canvas;
pub mod color;
pub mod intersections;
pub mod light;
pub mod material;
pub mod matrix;
pub mod ray;
pub mod shape;
pub mod sphere;
pub mod tuple;
pub mod world;

// Due to round-off errors two floats that should be equivalent might be slightly different.
// The following code was suggested by the book, so it should work within the book's scope.

const EPSILON: f64 = 0.00001;

fn float_eq(lhs: f64, rhs: f64) -> bool {
    (lhs - rhs).abs() < EPSILON
}

#[macro_export]
macro_rules! assert_float_eq {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !$crate::float_eq(*left_val, *right_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(
                        r#"assertion failed: `float_eq(left, right)`
  left: `{:?}`,
 right: `{:?}`"#,
                        &*left_val, &*right_val
                    );
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn float_literals_are_equal() {
        assert_float_eq!(1.0, 1.0);
    }

    #[test]
    fn float_literal_and_calculated_float_are_equal() {
        assert_float_eq!(1.0, 2.0 - 1.0);
    }
}
