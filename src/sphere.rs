use crate::intersections::{Intersection, Intersections};
use crate::material::Material;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::tuple::Tuple;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Sphere {
    pub transform: Matrix4,
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            transform: Matrix4::identity(),
            material: Material::new(),
        }
    }
}

impl Shape for Sphere {
    fn material(&self) -> &Material {
        &self.material
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn transform(&self) -> &Matrix4 {
        &self.transform
    }

    fn transform_mut(&mut self) -> &mut Matrix4 {
        &mut self.transform
    }

    fn local_intersect(&self, local_ray: Ray) -> Intersections<Self> {
        let sphere_to_ray = local_ray.origin - Tuple::new_point(0.0, 0.0, 0.0);
        let a = local_ray.direction * local_ray.direction;
        let b = 2.0 * (local_ray.direction * sphere_to_ray);
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

    fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        local_point - Tuple::new_point(0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_float_eq;
    use crate::material::Material;
    use crate::matrix::Matrix4;
    use crate::ray::Ray;
    use crate::shape::Shape;
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
        let xs = s.local_intersect(r);
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
        let xs = s.local_intersect(r);
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
        let xs = s.local_intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();
        let xs = s.local_intersect(r);
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
        let xs = s.local_intersect(r);
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
        let xs = s.local_intersect(r);

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
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();
        let n = s.local_normal_at(Tuple::new_point(1.0, 0.0, 0.0));
        let expected = Tuple::new_vector(1.0, 0.0, 0.0);

        assert_eq!(n, expected);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::new();
        let n = s.local_normal_at(Tuple::new_point(0.0, 1.0, 0.0));
        let expected = Tuple::new_vector(0.0, 1.0, 0.0);

        assert_eq!(n, expected);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::new();
        let n = s.local_normal_at(Tuple::new_point(0.0, 0.0, 1.0));
        let expected = Tuple::new_vector(0.0, 0.0, 1.0);

        assert_eq!(n, expected);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_non_axial_point() {
        let s = Sphere::new();
        let n = s.local_normal_at(Tuple::new_point(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));
        let expected = Tuple::new_vector(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        );

        assert_eq!(n, expected);
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Sphere::new();
        let n = s.local_normal_at(Tuple::new_point(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));

        assert_eq!(n, n.normalize());
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let s = Sphere::new();
        let m = s.material;

        assert_eq!(m, Material::new());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let mut s = Sphere::new();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m;

        assert_eq!(s.material, m);
    }
}
