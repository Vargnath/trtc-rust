use crate::float_eq;

#[derive(Debug)]
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
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        float_eq(self.x, other.x)
            && float_eq(self.y, other.y)
            && float_eq(self.z, other.z)
            && float_eq(self.w, other.w)
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
}
