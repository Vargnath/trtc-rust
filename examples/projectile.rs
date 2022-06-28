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
        velocity: Tuple::new_vector(1.0, 1.0, 0.0).normalize(),
    };
    // Gravity -0.1 unit/tick, and wind is -0.01 unit/tick.
    let e = Environment {
        gravity: Tuple::new_vector(0.0, -0.1, 0.0),
        wind: Tuple::new_point(-0.01, 0.0, 0.0),
    };

    let mut count = 0;
    while p.position.y > 0.0 {
        p = tick(&e, p);
        count += 1;
        println!("{:?}", p);
    }
    println!("The Projectile hit the ground after {} ticks", count);
}
