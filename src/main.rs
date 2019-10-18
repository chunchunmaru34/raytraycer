extern crate image;
extern crate num_cpus;

mod geometry;
mod camera;

use camera::{Camera};
use geometry::{Sphere, Ray, Vec3};
use image::{ImageBuffer};
use std::time::{Instant};

const HEIGHT: usize = 512;
const WIDTH: usize = 512;

fn main() {
    let scene = create_scene();
    let mut camera = Camera::new(Vec3::new(0., 0., 0.,));
    let start = Instant::now();
    loop {
        render(&scene, &camera);
        camera.move_by(&Vec3::new(1.0, -1.0, -1.0));
        println!("{}", camera.position.x);
    }
    let duration = start.elapsed();
    println!("{}", duration.as_millis());
}

fn render(scene: &Vec<Sphere>, camera: &Camera) {
    let mut image_buffer = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    for y in 0..WIDTH {
        for x in 0..WIDTH {
            let pixel = render_pixel(x as f32, y as f32, scene, camera);
            image_buffer.put_pixel(x as u32, y as u32, pixel);
        }
    }
}

fn render_pixel(x: f32, y: f32, scene: &Vec<Sphere>, camera: &Camera) -> image::Rgb<u8> {
    let direction = Vec3::new(x, y , 1.).normalize();
    let origin = Vec3::new(camera.position.x, camera.position.y, camera.position.z);
    // let origin = Vec3::new(0., 0., 0.);
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
    let coordinates = Vec3::new(10.0, 10.0, 12.0);
    let dark_green = (12 as u8, 55 as u8, 44 as u8);
    let test_sphere = Sphere::new(coordinates, 20.0, dark_green);

    let coordinates2 = Vec3::new(200.0, 300.0, 12.0);
    let pink = (225 as u8, 12 as u8, 225 as u8);
    let test_sphere2 = Sphere::new(coordinates2, 12.0, pink);

    // let light = Light::new(Point::new(500., 500., 150.), 20.);
    let scene = vec![test_sphere, test_sphere2];

    return scene;
}