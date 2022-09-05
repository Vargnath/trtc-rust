use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;
use trtc_rust::camera::Camera;
use trtc_rust::color::Color;
use trtc_rust::light::PointLight;
use trtc_rust::matrix::Matrix4;
use trtc_rust::sphere::Sphere;
use trtc_rust::tuple::Tuple;
use trtc_rust::world::World;

fn main() {
    let mut floor = Sphere::new();
    floor.transform = floor.transform.scale(10.0, 0.01, 10.0);
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = Sphere::new();
    left_wall.transform = left_wall
        .transform
        .scale(10.0, 0.01, 10.0)
        .rotate_x(PI / 2.0)
        .rotate_y(-PI / 4.0)
        .translate(0.0, 0.0, 5.0);
    left_wall.material = floor.material;

    let mut right_wall = Sphere::new();
    right_wall.transform = right_wall
        .transform
        .scale(10.0, 0.01, 10.0)
        .rotate_x(PI / 2.0)
        .rotate_y(PI / 4.0)
        .translate(0.0, 0.0, 5.0);
    right_wall.material = floor.material;

    let mut middle = Sphere::new();
    middle.transform = middle.transform.translate(-0.5, 1.0, 0.5);
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::new();
    right.transform = right
        .transform
        .scale(0.5, 0.5, 0.5)
        .translate(1.5, 0.5, -0.5);
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::new();
    left.transform = left
        .transform
        .scale(0.33, 0.33, 0.33)
        .translate(-1.5, 0.33, -0.75);
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut world = World::new();
    world.objects.push(floor);
    world.objects.push(left_wall);
    world.objects.push(right_wall);
    world.objects.push(middle);
    world.objects.push(right);
    world.objects.push(left);
    world.light = Some(PointLight::new(
        Tuple::new_point(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    let mut camera = Camera::new(100, 50, PI / 3.0);
    // let mut camera = Camera::new(500, 250, PI / 3.0);
    camera.transform = Matrix4::view_transform(
        Tuple::new_point(0.0, 1.5, -5.0),
        Tuple::new_point(0.0, 1.0, 0.0),
        Tuple::new_vector(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(world);

    let mut file = File::create("chapter_08.ppm").unwrap();
    file.write_all(canvas.to_ppm().as_slice()).unwrap();
}
