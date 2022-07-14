use crate::intersections::{Intersection, Intersections};
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::tuple::Tuple;

#[derive(Debug, Default, Copy, Clone)]
pub struct Sphere {
    pub transform: Matrix4,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            transform: Matrix4::identity(),
        }
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let ray = ray.transform(self.transform.inverse());
        let sphere_to_ray = ray.origin - Tuple::new_point(0.0, 0.0, 0.0);
        let a = ray.direction * ray.direction;
        let b = 2.0 * (ray.direction * sphere_to_ray);
        let c = (sphere_to_ray * sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            Intersections::new(Vec::new())
        } else {
            Intersections::new(vec![
                Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), self),
                Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), self),
            ])
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_float_eq;
    use crate::matrix::Matrix4;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::tuple::Tuple;
    use std::ptr;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_float_eq!(xs[0].t, 4.0);
        assert_float_eq!(xs[1].t, 6.0);
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
        assert_float_eq!(xs[0].t, 5.0);
        assert_float_eq!(xs[1].t, 5.0);
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
        assert_float_eq!(xs[0].t, -1.0);
        assert_float_eq!(xs[1].t, 1.0);
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
        assert_float_eq!(xs[0].t, -6.0);
        assert_float_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(ptr::eq(xs[0].object, &s));
        assert!(ptr::eq(xs[1].object, &s));
    }

    #[test]
    fn a_spheres_default_transformation() {
        let s = Sphere::new();

        assert_eq!(s.transform, Matrix4::identity());
    }

    #[test]
    fn changing_a_spheres_transformation() {
        let mut s = Sphere::new();
        let t = Matrix4::translation(2.0, 3.0, 4.0);
        s.transform = t;

        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let mut s = Sphere::new();
        s.transform = Matrix4::scaling(2.0, 2.0, 2.0);
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_float_eq!(xs[0].t, 3.0);
        assert_float_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let mut s = Sphere::new();
        s.transform = Matrix4::translation(5.0, 0.0, 0.0);
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }
}
