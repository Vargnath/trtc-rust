use crate::ray::Ray;
use crate::tuple::Tuple;

#[derive(Debug, Default, Copy, Clone)]
pub struct Sphere {}

impl Sphere {
    pub fn new() -> Self {
        Self {}
    }

    pub fn intersect(&self, ray: Ray) -> Vec<f64> {
        let sphere_to_ray = ray.origin - Tuple::new_point(0.0, 0.0, 0.0);
        let a = ray.direction * ray.direction;
        let b = 2.0 * (ray.direction * sphere_to_ray);
        let c = (sphere_to_ray * sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            Vec::new()
        } else {
            vec![
                (-b - discriminant.sqrt()) / (2.0 * a),
                (-b + discriminant.sqrt()) / (2.0 * a),
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_float_eq;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::tuple::Tuple;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_float_eq!(xs[0], 4.0);
        assert_float_eq!(xs[1], 6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(
            Tuple::new_point(0.0, 1.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_float_eq!(xs[0], 5.0);
        assert_float_eq!(xs[1], 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(
            Tuple::new_point(0.0, 2.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_float_eq!(xs[0], -1.0);
        assert_float_eq!(xs[1], 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_float_eq!(xs[0], -6.0);
        assert_float_eq!(xs[1], -4.0);
    }
}
