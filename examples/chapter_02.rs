use std::fs::File;
use std::io::Write;
use trtc_rust::canvas::Canvas;
use trtc_rust::color::Color;
use trtc_rust::tuple::Tuple;

#[derive(Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(env: &Environment, proj: Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile { position, velocity }
}

fn main() {
    // Projectile starts one unit above the origin.
    // Velocity is normalized to 1 unit/tick.
    let mut p = Projectile {
        position: Tuple::new_point(0.0, 1.0, 0.0),
        velocity: Tuple::new_vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };
    // Gravity -0.1 unit/tick, and wind is -0.01 unit/tick.
    let e = Environment {
        gravity: Tuple::new_vector(0.0, -0.1, 0.0),
        wind: Tuple::new_point(-0.01, 0.0, 0.0),
    };

    let mut c = Canvas::new(900, 550);
    let green = Color::new(0.0, 1.0, 0.0);
    while p.position.y > 0.0 {
        p = tick(&e, p);
        if p.position.x.is_sign_positive() && p.position.y.is_sign_positive() {
            let x = p.position.x as usize;
            let y = c.height - p.position.y as usize;
            if x < c.width && y < c.height {
                c.write_pixel(x, y, green);
            }
        }
    }
    let mut file = File::create("chapter_02.ppm").unwrap();
    file.write_all(c.to_ppm().as_slice()).unwrap();
}
