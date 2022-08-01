use crate::color::Color;
use crate::intersections::{Computations, Intersections};
use crate::light::PointLight;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct World {
    pub objects: Vec<Sphere>,
    pub light: Option<PointLight>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            light: None,
        }
    }

    pub fn intersect_world(&self, r: Ray) -> Intersections {
        let mut xs = Vec::new();
        for object in self.objects.iter() {
            xs.extend_from_slice(object.intersect(r).as_ref());
        }
        xs.sort_by(|lhs, rhs| lhs.t.partial_cmp(&rhs.t).unwrap());
        Intersections::new(xs)
    }

    pub fn shade_hit(&self, comps: Computations) -> Color {
        comps.object.material.lighting(
            self.light.unwrap(),
            comps.point,
            comps.eyev,
            comps.normalv,
            false,
        )
    }

    pub fn color_at(&self, r: Ray) -> Color {
        let xs = self.intersect_world(r);
        if let Some(hit) = xs.hit() {
            let comps = hit.prepare_computations(r);
            self.shade_hit(comps)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }

    pub fn is_shadowed(&self, point: Tuple) -> bool {
        let v = self.light.unwrap().position - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(point, direction);
        let intersections = self.intersect_world(r);

        let h = intersections.hit();
        h.map_or(false, |h| h.t < distance)
    }
}

impl Default for World {
    fn default() -> Self {
        World::new()
    }
}

#[doc(hidden)]
pub fn default_world() -> World {
    let light = PointLight::new(
        Tuple::new_point(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    );
    let mut s1 = Sphere::new();
    s1.material.color = Color::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;
    let mut s2 = Sphere::new();
    s2.transform = Matrix4::scaling(0.5, 0.5, 0.5);

    World {
        objects: vec![s1, s2],
        light: Some(light),
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_float_eq;
    use crate::color::Color;
    use crate::intersections::Intersection;
    use crate::light::PointLight;
    use crate::matrix::Matrix4;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::tuple::Tuple;
    use crate::world::{default_world, World};

    #[test]
    fn creating_a_world() {
        let w = World::new();

        assert!(w.objects.is_empty());
        assert_eq!(w.light, None)
    }

    #[test]
    fn the_default_world() {
        let light = PointLight::new(
            Tuple::new_point(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );
        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Sphere::new();
        s2.transform = Matrix4::scaling(0.5, 0.5, 0.5);
        let w = default_world();

        assert_eq!(w.light, Some(light));
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = default_world();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let xs = w.intersect_world(r);

        assert_eq!(xs.len(), 4);
        assert_float_eq!(xs[0].t, 4.0);
        assert_float_eq!(xs[1].t, 4.5);
        assert_float_eq!(xs[2].t, 5.5);
        assert_float_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let w = default_world();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let shape = w.objects[0];
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = default_world();
        w.light = Some(PointLight::new(
            Tuple::new_point(0.0, 0.25, 0.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let shape = w.objects[1];
        let i = Intersection::new(0.5, &shape);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);

        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = default_world();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        let c = w.color_at(r);

        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = default_world();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let c = w.color_at(r);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut w = default_world();
        let inner = {
            let mut outer = &mut w.objects[0];
            outer.material.ambient = 1.0;
            let inner = &mut w.objects[1];
            inner.material.ambient = 1.0;
            *inner
        };
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.75),
            Tuple::new_vector(0.0, 0.0, -1.0),
        );
        let c = w.color_at(r);

        assert_eq!(c, inner.material.color);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = default_world();
        let p = Tuple::new_point(0.0, 10.0, 0.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = default_world();
        let p = Tuple::new_point(10.0, -10.0, 10.0);

        assert!(w.is_shadowed(p));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = default_world();
        let p = Tuple::new_point(-20.0, 20.0, -20.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = default_world();
        let p = Tuple::new_point(-2.0, 2.0, -2.0);

        assert!(!w.is_shadowed(p));
    }
}
