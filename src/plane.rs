use crate::intersections::{Intersection, Intersections};
use crate::material::Material;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::tuple::Tuple;
use crate::EPSILON;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Plane {
    pub transform: Matrix4,
    pub material: Material,
}

impl Plane {
    pub fn new() -> Self {
        Self {
            transform: Matrix4::identity(),
            material: Material::new(),
        }
    }
}

impl Shape for Plane {
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
        if local_ray.direction.y.abs() < EPSILON {
            return Intersections::new(Vec::new());
        }
        let t = -local_ray.origin.y / local_ray.direction.y;
        Intersections::new(vec![Intersection::new(t, self)])
    }

    fn local_normal_at(&self, _local_point: Tuple) -> Tuple {
        Tuple::new_vector(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_float_eq;
    use crate::plane::Plane;
    use crate::ray::Ray;
    use crate::shape::Shape;
    use crate::tuple::Tuple;
    use std::ptr;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::new();
        let n1 = p.local_normal_at(Tuple::new_point(0.0, 0.0, 1.0));
        let n2 = p.local_normal_at(Tuple::new_point(10.0, 0.0, -1.0));
        let n3 = p.local_normal_at(Tuple::new_point(-5.0, 0.0, 150.0));

        assert_eq!(n1, Tuple::new_vector(0.0, 1.0, 0.0));
        assert_eq!(n2, Tuple::new_vector(0.0, 1.0, 0.0));
        assert_eq!(n3, Tuple::new_vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = Plane::new();
        let r = Ray::new(
            Tuple::new_point(0.0, 10.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let xs = p.local_intersect(r);

        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = Plane::new();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let xs = p.local_intersect(r);

        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Plane::new();
        let r = Ray::new(
            Tuple::new_point(0.0, 1.0, 0.0),
            Tuple::new_vector(0.0, -1.0, 0.0),
        );
        let xs = p.local_intersect(r);

        assert_eq!(xs.len(), 1);
        assert_float_eq!(xs[0].t, 1.0);
        assert!(ptr::eq(xs[0].object, &p));
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Plane::new();
        let r = Ray::new(
            Tuple::new_point(0.0, -1.0, 0.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        let xs = p.local_intersect(r);

        assert_eq!(xs.len(), 1);
        assert_float_eq!(xs[0].t, 1.0);
        assert!(ptr::eq(xs[0].object, &p));
    }
}
