use crate::ray::Ray;
use crate::shape::Shape;
use crate::sphere::Sphere;
use crate::tuple::Tuple;
use crate::EPSILON;
use std::ops::{Deref, Index};
use std::ptr;

pub struct Computations<'a> {
    pub t: f64,
    pub object: &'a Sphere,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
}

#[derive(Debug, Copy, Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Sphere) -> Self {
        Self { t, object }
    }

    pub fn prepare_computations(&self, r: Ray) -> Computations {
        let object = self.object;
        let point = r.position(self.t);
        let eyev = -r.direction;
        let mut normalv = object.normal_at(point);
        let inside = if normalv * eyev < 0.0 {
            normalv = -normalv;
            true
        } else {
            false
        };
        let over_point = point + normalv * EPSILON;
        Computations {
            t: self.t,
            object,
            point,
            eyev,
            normalv,
            inside,
            over_point,
        }
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && ptr::eq(self.object, other.object)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Intersections<'a> {
    inner: Vec<Intersection<'a>>,
    hit: Option<usize>,
}

impl<'a> Intersections<'a> {
    pub fn new(mut intersections: Vec<Intersection<'a>>) -> Self {
        intersections.sort_by(|lhs, rhs| lhs.t.partial_cmp(&rhs.t).unwrap());
        let hit = intersections
            .iter()
            .enumerate()
            .find(|(_, e)| e.t.is_sign_positive())
            .map(|(i, _)| i);
        Self {
            inner: intersections,
            hit,
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn hit(&self) -> Option<&Intersection<'a>> {
        match self.hit {
            Some(hit) => Some(&self[hit]),
            None => None,
        }
    }
}

impl<'a> Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<'a> Deref for Intersections<'a> {
    type Target = [Intersection<'a>];

    fn deref(&self) -> &Self::Target {
        self.inner.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_float_eq;
    use crate::intersections::{Intersection, Intersections};
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::tuple::Tuple;
    use std::ptr;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert!(ptr::eq(i.object, &s));
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);

        let xs = Intersections::new(vec![i1, i2]);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = Intersections::new(vec![i2, i1]);

        let i = xs.hit();

        assert_eq!(i, Some(&i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = Intersections::new(vec![i2, i1]);

        let i = xs.hit();

        assert_eq!(i, Some(&i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = Intersections::new(vec![i2, i1]);

        let i = xs.hit();

        assert_eq!(i, None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_non_negative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = Intersections::new(vec![i1, i2, i3, i4]);

        let i = xs.hit();

        assert_eq!(i, Some(&i4));
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(r);

        assert_float_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Tuple::new_point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Tuple::new_vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::new_vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_when_intersection_occurs_on_the_outside() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(r);

        assert!(!comps.inside);
    }

    #[test]
    fn the_hit_when_an_intersections_occurs_on_the_inside() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let shape = Sphere::new();
        let i = Intersection::new(1.0, &shape);
        let comps = i.prepare_computations(r);

        assert_eq!(comps.point, Tuple::new_point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Tuple::new_vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.normalv, Tuple::new_vector(0.0, 0.0, -1.0));
    }
}
