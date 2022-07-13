use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::tuple::Tuple;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = Tuple::new_point(1.0, 2.0, 3.0);
        let direction = Tuple::new_vector(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = Ray::new(
            Tuple::new_point(2.0, 3.0, 4.0),
            Tuple::new_vector(1.0, 0.0, 0.0),
        );

        let expected = Tuple::new_point(2.0, 3.0, 4.0);
        assert_eq!(r.position(0.0), expected);

        let expected = Tuple::new_point(3.0, 3.0, 4.0);
        assert_eq!(r.position(1.0), expected);

        let expected = Tuple::new_point(1.0, 3.0, 4.0);
        assert_eq!(r.position(-1.0), expected);

        let expected = Tuple::new_point(4.5, 3.0, 4.0);
        assert_eq!(r.position(2.5), expected);
    }
}
