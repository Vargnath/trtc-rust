use crate::intersections::Intersections;
use crate::material::Material;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::tuple::Tuple;

pub trait Shape: std::fmt::Debug {
    fn transform(&self) -> Matrix4;

    fn material(&self) -> Material;

    fn local_intersect(&self, local_ray: Ray) -> Intersections;

    fn local_normal_at(&self, local_point: Tuple) -> Tuple;

    fn intersect(&self, ray: Ray) -> Intersections {
        let local_ray = ray.transform(self.transform().inverse());
        self.local_intersect(local_ray)
    }

    fn normal_at(&self, point: Tuple) -> Tuple {
        let local_point = self.transform().inverse() * point;
        let local_normal = self.local_normal_at(local_point);
        let mut world_normal = self.transform().inverse().transpose() * local_normal;
        world_normal.w = 0.0;

        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use crate::intersections::Intersections;
    use crate::material::Material;
    use crate::matrix::Matrix4;
    use crate::ray::Ray;
    use crate::shape::Shape;
    use crate::tuple::Tuple;
    use std::cell::Cell;
    use std::f64::consts::PI;

    #[derive(Debug)]
    struct TestShape {
        transform: Matrix4,
        material: Material,
        saved_ray: Cell<Option<Ray>>,
    }

    impl TestShape {
        pub fn new() -> Self {
            TestShape {
                transform: Matrix4::identity(),
                material: Material::new(),
                saved_ray: Cell::new(None),
            }
        }
    }

    impl Shape for TestShape {
        fn transform(&self) -> Matrix4 {
            self.transform
        }

        fn material(&self) -> Material {
            self.material
        }

        fn local_intersect(&self, ray: Ray) -> Intersections {
            self.saved_ray.set(Some(ray));

            Intersections::new(Vec::new())
        }

        fn local_normal_at(&self, local_point: Tuple) -> Tuple {
            Tuple::new_vector(local_point.x, local_point.y, local_point.z)
        }
    }

    fn test_shape() -> TestShape {
        TestShape::new()
    }

    #[test]
    fn the_default_transformation() {
        let s = test_shape();

        assert_eq!(s.transform(), Matrix4::identity());
    }

    #[test]
    fn assigning_a_transformation() {
        let mut s = test_shape();
        s.transform = Matrix4::translation(2.0, 3.0, 4.0);

        assert_eq!(s.transform(), Matrix4::translation(2.0, 3.0, 4.0));
    }

    #[test]
    fn the_default_material() {
        let s = test_shape();

        assert_eq!(s.material(), Material::new());
    }

    #[test]
    fn assigning_a_material() {
        let mut s = test_shape();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m;

        assert_eq!(s.material(), m);
    }

    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let mut s = test_shape();
        s.transform = Matrix4::scaling(2.0, 2.0, 2.0);
        let _ = s.intersect(r);

        let saved_ray = s.saved_ray.get();
        assert!(saved_ray.is_some());
        assert_eq!(saved_ray.unwrap().origin, Tuple::new_point(0.0, 0.0, -2.5));
        assert_eq!(
            saved_ray.unwrap().direction,
            Tuple::new_vector(0.0, 0.0, 0.5)
        );
    }

    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let mut s = test_shape();
        s.transform = Matrix4::translation(5.0, 0.0, 0.0);
        let _ = s.intersect(r);

        let saved_ray = s.saved_ray.get();
        assert!(saved_ray.is_some());
        assert_eq!(saved_ray.unwrap().origin, Tuple::new_point(-5.0, 0.0, -5.0));
        assert_eq!(
            saved_ray.unwrap().direction,
            Tuple::new_vector(0.0, 0.0, 1.0)
        );
    }

    #[test]
    fn computing_the_normal_on_a_translate_shape() {
        let mut s = test_shape();
        s.transform = Matrix4::translation(0.0, 1.0, 0.0);
        let n = s.normal_at(Tuple::new_point(0.0, 1.70711, -0.70711));

        assert_eq!(n, Tuple::new_vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let mut s = test_shape();
        s.transform = Matrix4::scaling(1.0, 0.5, 1.0) * Matrix4::rotation_z(PI / 5.0);
        let n = s.normal_at(Tuple::new_point(
            0.0,
            f64::sqrt(2.0) / 2.0,
            -f64::sqrt(2.0) / 2.0,
        ));

        assert_eq!(n, Tuple::new_vector(0.0, 0.97014, -0.24254));
    }
}
