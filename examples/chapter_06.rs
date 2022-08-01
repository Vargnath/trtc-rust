use std::fs::File;
use std::io::Write;
use trtc_rust::canvas::Canvas;
use trtc_rust::color::Color;
use trtc_rust::light::PointLight;
use trtc_rust::ray::Ray;
use trtc_rust::sphere::Sphere;
use trtc_rust::tuple::Tuple;

fn main() {
    let ray_origin = Tuple::new_point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100usize;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    let mut shape = Sphere::new();
    shape.material.color = Color::new(1.0, 0.2, 1.0);

    let light_position = Tuple::new_point(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_position, light_color);

    for y in 0..canvas.height {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas.width {
            let world_x = -half + pixel_size * x as f64;
            let position = Tuple::new_point(world_x, world_y, wall_z);

            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(r);

            if let Some(hit) = xs.hit() {
                let point = r.position(hit.t);
                let normal = hit.object.normal_at(point);
                let eye = -r.direction;
                let color = hit
                    .object
                    .material
                    .lighting(light, point, eye, normal, false);
                canvas.write_pixel(x, y, color);
            }
        }
    }

    let mut file = File::create("chapter_06.ppm").unwrap();
    file.write_all(canvas.to_ppm().as_slice()).unwrap();
}
