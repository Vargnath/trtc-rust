use crate::color::Color;
use crate::intersections::{Computations, Intersection, Intersections};
use crate::light::PointLight;
use crate::material::Material;
use crate::matrix::Matrix4;
use crate::plane::Plane;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::sphere::Sphere;
use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WorldShape {
    Sphere(Sphere),
    Plane(Plane),
}

impl From<Sphere> for WorldShape {
    fn from(sphere: Sphere) -> Self {
        Self::Sphere(sphere)
    }
}

impl From<Plane> for WorldShape {
    fn from(plane: Plane) -> Self {
        Self::Plane(plane)
    }
}

impl Shape for WorldShape {
    fn material(&self) -> &Material {
        match self {
            WorldShape::Sphere(sphere) => sphere.material(),
            WorldShape::Plane(plane) => plane.material(),
        }
    }

    fn material_mut(&mut self) -> &mut Material {
        match self {
            WorldShape::Sphere(sphere) => sphere.material_mut(),
            WorldShape::Plane(plane) => plane.material_mut(),
        }
    }

    fn transform(&self) -> &Matrix4 {
        match self {
            WorldShape::Sphere(sphere) => sphere.transform(),
            WorldShape::Plane(plane) => plane.transform(),
        }
    }

    fn transform_mut(&mut self) -> &mut Matrix4 {
        match self {
            WorldShape::Sphere(sphere) => sphere.transform_mut(),
            WorldShape::Plane(plane) => plane.transform_mut(),
        }
    }

    fn local_intersect(&self, local_ray: Ray) -> Intersections<Self> {
        Intersections::new(
            match self {
                WorldShape::Sphere(sphere) => sphere
                    .local_intersect(local_ray)
                    .iter()
                    .map(|x| x.t)
                    .collect::<Vec<_>>(),
                WorldShape::Plane(plane) => plane
                    .local_intersect(local_ray)
                    .iter()
                    .map(|x| x.t)
                    .collect::<Vec<_>>(),
            }
            .into_iter()
            .map(|x| Intersection::<Self>::new(x, self))
            .collect::<Vec<_>>(),
        )
    }

    fn local_normal_at(&self, local_point: Tuple) -> Tuple {
        match self {
            WorldShape::Sphere(sphere) => sphere.local_normal_at(local_point),
            WorldShape::Plane(plane) => plane.local_normal_at(local_point),
        }
    }
}

#[derive(Debug, Clone)]
pub struct World<S: Shape = WorldShape> {
    pub objects: Vec<S>,
    pub light: Option<PointLight>,
}

impl<S: Shape> World<S> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            light: None,
        }
    }

    pub fn intersect_world(&self, r: Ray) -> Intersections<S> {
        let mut xs = Vec::new();
        for object in self.objects.iter() {
            xs.extend_from_slice(object.intersect(r).as_ref());
        }
        xs.sort_by(|lhs, rhs| lhs.t.partial_cmp(&rhs.t).unwrap());
        Intersections::new(xs)
    }

    pub fn shade_hit(&self, comps: Computations<S>) -> Color {
        let shadowed = self.is_shadowed(comps.over_point);
        comps.object.material().lighting(
            self.light.unwrap(),
            comps.point,
            comps.eyev,
            comps.normalv,
            shadowed,
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

impl<S: Shape> Default for World<S> {
    fn default() -> Self {
        World::new()
    }
}

#[doc(hidden)]
pub fn default_world() -> World<Sphere> {
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
    use crate::color::Color;
    use crate::intersections::Intersection;
    use crate::light::PointLight;
    use crate::matrix::Matrix4;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::tuple::Tuple;
    use crate::world::{default_world, World};
    use crate::{assert_float_eq, EPSILON};

    #[test]
    fn creating_a_world() {
        let w: World = World::new();

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

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = World::new();
        w.light = Some(PointLight::new(
            Tuple::new_point(0.0, 0.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        let s1 = Sphere::new();
        w.objects.push(s1);
        let mut s2 = Sphere::new();
        s2.transform = Matrix4::translation(0.0, 0.0, 10.0);
        w.objects.push(s2);
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let i = Intersection::new(4.0, &s2);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);

        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let mut shape = Sphere::new();
        shape.transform = Matrix4::translation(0.0, 0.0, 1.0);
        let i = Intersection::new(5.0, &shape);
        let comps = i.prepare_computations(r);

        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}
