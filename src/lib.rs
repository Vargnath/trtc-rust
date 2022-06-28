pub mod canvas;
pub mod color;
pub mod tuple;

// Due to round-off errors two floats that should be equivalent might be slightly different.
// The following code was suggested by the book, so it should work within the book's scope.

const EPSILON: f64 = 0.00001;

fn float_eq(lhs: f64, rhs: f64) -> bool {
    (lhs - rhs).abs() < EPSILON
}

#[cfg(test)]
mod tests {
    use crate::float_eq;

    #[test]
    fn float_literals_are_equal() {
        assert!(float_eq(1.0, 1.0));
    }

    #[test]
    fn float_literal_and_calculated_float_are_equal() {
        assert!(float_eq(1.0, 2.0 - 1.0));
    }
}
