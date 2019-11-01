extern crate image;
extern crate num_cpus;

mod geometry;
mod camera;

use camera::{Camera};
use geometry::{Sphere, Ray, Vec3};
use image::{ImageBuffer};
use std::time::{Instant};

const HEIGHT: usize = 768;
const WIDTH: usize = 1024;
const FOV: f32 = 3.14 / 3.;

fn main() {
    let scene = create_scene();
    let mut camera = Camera::new(Vec3::new(0., 0., 0.,));
    // loop {
        let start = Instant::now();
        render(&scene, &camera);
        let duration = start.elapsed();
        println!("{}", duration.as_millis());
        camera.move_by(&Vec3::new(1.0, -1.0, -1.0));
    // }
}

fn render(scene: &Vec<Sphere>, camera: &Camera) {
    let img = ImageBuffer::from_fn(WIDTH as u32, HEIGHT as u32, |x, y| {
        render_pixel(x as f32, y as f32, scene, camera)
    });

    img.save("test.png").unwrap();
}

fn render_pixel(x: f32, y: f32, scene: &Vec<Sphere>, camera: &Camera) -> image::Rgb<u8> {
    let direction = Vec3::new(
        (x + 0.5) - WIDTH as f32 / 2.,
        -(y + 0.5) + HEIGHT as f32 / 2.,
        -(HEIGHT as f32) / (2. * (FOV / 2.).tan())
    ).normalize();
    let origin = Vec3::new(camera.position.x, camera.position.y, camera.position.z);
    let mut ray = Ray::new(origin, direction, std::f32::MAX);

    let pixel = cast_ray(&mut ray, scene);

    return pixel;
}

fn cast_ray(ray: &mut Ray, scene: &Vec<Sphere>) -> image::Rgb<u8> {
    let mut pixel = image::Rgb([185, 185, 185]);

    for sphere in scene {
        if sphere.ray_intersect(ray) {
            pixel = image::Rgb([sphere.color.0, sphere.color.1, sphere.color.2])
        }
    }

    return pixel;
}

fn create_scene() -> Vec<Sphere> {
    let coordinates = Vec3::new(-3.0, 0., -16.0);
    let dark_green = (12 as u8, 55 as u8, 44 as u8);
    let test_sphere = Sphere::new(coordinates, 2.0, dark_green);

    let coordinates2 = Vec3::new(20.0, 300.0, 12.0);
    let metallic_red = (170 as u8, 84 as u8, 84 as u8);
    let test_sphere2 = Sphere::new(coordinates2, 12.0, metallic_red);

    // let light = Light::new(Point::new(500., 500., 150.), 20.);
    let scene = vec![test_sphere];

    return scene;
}