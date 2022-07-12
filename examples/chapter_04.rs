use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;
use trtc_rust::canvas::Canvas;
use trtc_rust::color::Color;
use trtc_rust::matrix::Matrix4;
use trtc_rust::tuple::Tuple;

const CANVAS_WIDTH: usize = 200;
const CANVAS_HEIGHT: usize = 200;
const NUM_HOURS: usize = 12;

fn main() {
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    let origin = Tuple::new_point(canvas.width as f64 / 2.0, 0.0, canvas.height as f64 / 2.0);
    let radius = CANVAS_WIDTH as f64 * 3.0 / 8.0;

    (0..NUM_HOURS)
        .into_iter()
        .map(|hour| {
            Matrix4::scaling(radius, 1.0, radius)
                .rotate_y(2.0 * PI * (hour as f64 / NUM_HOURS as f64))
                .translate(origin.x, origin.y, origin.z)
                * Tuple::new_point(0.0, 0.0, 1.0)
        })
        .for_each(|hour| {
            canvas.write_pixel(
                hour.x.round() as usize,
                CANVAS_HEIGHT - hour.z.round() as usize,
                Color::new(1.0, 1.0, 1.0),
            );
        });

    let mut file = File::create("chapter_04.ppm").unwrap();
    file.write_all(canvas.to_ppm().as_slice()).unwrap();
}
