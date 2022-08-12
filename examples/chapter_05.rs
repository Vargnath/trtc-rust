use std::fs::File;
use std::io::Write;
use trtc_rust::canvas::Canvas;
use trtc_rust::color::Color;
use trtc_rust::ray::Ray;
use trtc_rust::shape::Shape;
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
    let color = Color::new(1.0, 0.0, 0.0);

    #[allow(unused_mut)]
    let mut shape = Sphere::new();

    // Shrink it along the y axis
    // shape.transform = shape.transform.scale(1.0, 0.5, 1.0);

    // Shrink it along the x axis
    // shape.transform = shape.transform.scale(0.5, 1.0, 1.0);

    // Shrink it, and rotate it!
    // shape.transform = shape
    //     .transform
    //     .scale(0.5, 1.0, 1.0)
    //     .rotate_z(std::f64::consts::PI / 4.0);

    // Shrink it, and skew it!
    // shape.transform = shape
    //     .transform
    //     .scale(0.5, 1.0, 1.0)
    //     .shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    for y in 0..canvas.height {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas.width {
            let world_x = -half + pixel_size * x as f64;
            let position = Tuple::new_point(world_x, world_y, wall_z);

            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(r);

            if xs.hit().is_some() {
                canvas.write_pixel(x, y, color);
            }
        }
    }

    let mut file = File::create("chapter_05.ppm").unwrap();
    file.write_all(canvas.to_ppm().as_slice()).unwrap();
}
