use crate::canvas::Canvas;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::tuple::Tuple;
use crate::world::World;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    pub transform: Matrix4,
    pub half_width: f64,
    pub half_height: f64,
    pub pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let half_view = f64::tan(field_of_view / 2.0);
        let aspect = hsize as f64 / vsize as f64;
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        let pixel_size = (half_width * 2.0) / hsize as f64;

        Self {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix4::identity(),
            half_width,
            half_height,
            pixel_size,
        }
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;
        let pixel = self.transform.inverse() * Tuple::new_point(world_x, world_y, -1.0);
        let origin = self.transform.inverse() * Tuple::new_point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();
        Ray::new(origin, direction)
    }

    pub fn render<S: Shape>(&self, world: World<S>) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray);
                image.write_pixel(x, y, color);
            }
        }
        image
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_float_eq;
    use crate::camera::Camera;
    use crate::color::Color;
    use crate::matrix::Matrix4;
    use crate::tuple::Tuple;
    use crate::world::default_world;
    use std::f64::consts::PI;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;
        let c = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, PI / 2.0);
        assert_eq!(c.transform, Matrix4::identity());
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);

        assert_float_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);

        assert_float_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Tuple::new_point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Tuple::new_vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);

        assert_eq!(r.origin, Tuple::new_point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Tuple::new_vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform = Matrix4::rotation_y(PI / 4.0) * Matrix4::translation(0.0, -2.0, 5.0);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Tuple::new_point(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            Tuple::new_vector(f64::sqrt(2.0) / 2.0, 0.0, -f64::sqrt(2.0) / 2.0)
        );
    }

    #[test]
    fn rendering_a_world_with_camera() {
        let w = default_world();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = Tuple::new_point(0.0, 0.0, -5.0);
        let to = Tuple::new_point(0.0, 0.0, 0.0);
        let up = Tuple::new_vector(0.0, 1.0, 0.0);
        c.transform = Matrix4::view_transform(from, to, up);
        let image = c.render(w);

        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}
