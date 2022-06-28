use crate::float_eq;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    w: f64,
}

impl Tuple {
    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn is_point(&self) -> bool {
        float_eq(self.w, 1.0)
    }

    pub fn is_vector(&self) -> bool {
        float_eq(self.w, 0.0)
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2))
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: self.w / magnitude,
        }
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self::new_vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        float_eq(self.x, other.x)
            && float_eq(self.y, other.y)
            && float_eq(self.z, other.z)
            && float_eq(self.w, other.w)
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let zero = Tuple::new_vector(0.0, 0.0, 0.0);
        zero - self
    }
}

impl Mul for Tuple {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;

    #[test]
    fn tuple_with_w_equals_1_is_a_point() {
        let a = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn tuple_with_w_equals_0_is_a_vector() {
        let a = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn new_point_creates_tuple_with_w_equals_1() {
        let a = Tuple {
            x: 4.0,
            y: -4.0,
            z: 3.0,
            w: 1.0,
        };
        let p = Tuple::new_point(4.0, -4.0, 3.0);
        assert!(a.is_point());
        assert_eq!(a, p);
    }

    #[test]
    fn new_vector_creates_tuple_with_w_equals_0() {
        let a = Tuple {
            x: 4.0,
            y: -4.0,
            z: 3.0,
            w: 0.0,
        };
        let p = Tuple::new_vector(4.0, -4.0, 3.0);
        assert!(a.is_vector());
        assert_eq!(a, p);
    }

    #[test]
    fn adding_two_tuples_results_in_correct_tuple() {
        let a1 = Tuple {
            x: 3.0,
            y: -2.0,
            z: 5.0,
            w: 1.0,
        };
        let a2 = Tuple {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: 0.0,
        };
        let expected = Tuple {
            x: 1.0,
            y: 1.0,
            z: 6.0,
            w: 1.0,
        };
        assert_eq!(a1 + a2, expected);
    }

    #[test]
    fn subtracting_two_points_results_in_correct_vector() {
        let p1 = Tuple::new_point(3.0, 2.0, 1.0);
        let p2 = Tuple::new_point(5.0, 6.0, 7.0);
        let expected = Tuple::new_vector(-2.0, -4.0, -6.0);
        assert_eq!(p1 - p2, expected);
    }

    #[test]
    fn subtracting_a_vector_from_a_point_results_in_correct_point() {
        let p = Tuple::new_point(3.0, 2.0, 1.0);
        let v = Tuple::new_vector(5.0, 6.0, 7.0);
        let expected = Tuple::new_point(-2.0, -4.0, -6.0);
        assert_eq!(p - v, expected);
    }

    #[test]
    fn subtracting_two_vectors_results_in_correct_vector() {
        let v1 = Tuple::new_vector(3.0, 2.0, 1.0);
        let v2 = Tuple::new_vector(5.0, 6.0, 7.0);
        let expected = Tuple::new_vector(-2.0, -4.0, -6.0);
        assert_eq!(v1 - v2, expected);
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector_results_in_correct_vector() {
        let zero = Tuple::new_vector(0.0, 0.0, 0.0);
        let v = Tuple::new_vector(1.0, -2.0, 3.0);
        let expected = Tuple::new_vector(-1.0, 2.0, -3.0);
        assert_eq!(zero - v, expected);
    }

    #[test]
    fn negating_a_tuple_results_in_correct_tuple() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let expected = Tuple {
            x: -1.0,
            y: 2.0,
            z: -3.0,
            w: 4.0,
        };
        assert_eq!(-a, expected);
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar_results_in_correct_tuple() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let expected = Tuple {
            x: 3.5,
            y: -7.0,
            z: 10.5,
            w: -14.0,
        };
        assert_eq!(a * 3.5, expected);
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction_results_in_correct_tuple() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let expected = Tuple {
            x: 0.5,
            y: -1.0,
            z: 1.5,
            w: -2.0,
        };
        assert_eq!(a * 0.5, expected);
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar_results_in_correct_tuple() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        let expected = Tuple {
            x: 0.5,
            y: -1.0,
            z: 1.5,
            w: -2.0,
        };
        assert_eq!(a / 2.0, expected);
    }

    #[test]
    fn magnitude_of_vector_1_0_0_is_1() {
        let v = Tuple::new_vector(1.0, 0.0, 0.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_0_1_0_is_1() {
        let v = Tuple::new_vector(0.0, 1.0, 0.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_0_0_1_is_1() {
        let v = Tuple::new_vector(0.0, 0.0, 1.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_results_is_correct() {
        let v = Tuple::new_vector(1.0, 2.0, 3.0);

        assert_eq!(v.magnitude(), f64::sqrt(14.0));
    }

    #[test]
    fn magnitude_of_all_negative_vector_results_is_correct() {
        let v = Tuple::new_vector(-1.0, -2.0, -3.0);

        assert_eq!(v.magnitude(), f64::sqrt(14.0));
    }

    #[test]
    fn normalizing_vector_4_0_0_gives_vector_1_0_0() {
        let v = Tuple::new_vector(4.0, 0.0, 0.0);
        let expected = Tuple::new_vector(1.0, 0.0, 0.0);

        assert_eq!(v.normalize(), expected);
    }

    #[test]
    fn normalizing_vector_4_0_0_gives_correct_vector() {
        let v = Tuple::new_vector(1.0, 2.0, 3.0);

        let sqrt_14 = f64::sqrt(14.0);
        let expected = Tuple::new_vector(1.0 / sqrt_14, 2.0 / sqrt_14, 3.0 / sqrt_14);

        assert_eq!(v.normalize(), expected);
    }

    #[test]
    fn magnitude_of_normalized_vector_is_1() {
        let v = Tuple::new_vector(1.0, 2.0, 3.0);

        assert_eq!(v.normalize().magnitude(), 1.0);
    }

    #[test]
    fn scalar_product_of_two_vectors_is_correct() {
        let a = Tuple::new_vector(1.0, 2.0, 3.0);
        let b = Tuple::new_vector(2.0, 3.0, 4.0);

        assert_eq!(a * b, 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors_is_correct() {
        let a = Tuple::new_vector(1.0, 2.0, 3.0);
        let b = Tuple::new_vector(2.0, 3.0, 4.0);

        let expected = Tuple::new_vector(-1.0, 2.0, -1.0);
        assert_eq!(a.cross(&b), expected);

        let expected = Tuple::new_vector(1.0, -2.0, 1.0);
        assert_eq!(b.cross(&a), expected);
    }
}
