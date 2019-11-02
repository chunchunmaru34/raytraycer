extern crate image;

mod geometry;
mod scene;

use geometry::sphere::{Sphere};
use geometry::ray::{Ray};
use geometry::vec3::{Vec3};
use scene::light::{Light};
use scene::camera::{Camera};
use scene::{Scene};

use image::{ImageBuffer};
use std::time::{Instant};

const HEIGHT: usize = 768;
const WIDTH: usize = 1024;
const FOV: f32 = 3.14 / 3.;

fn main() {
    let mut scene = create_scene();
    // loop {
        let start = Instant::now();
        render(&scene);
        let duration = start.elapsed();
        println!("{}", duration.as_millis());
        scene.camera.move_by(&Vec3::new(1.0, -1.0, -1.0));
    // }
}

fn render(scene: &Scene) {
    let img = ImageBuffer::from_fn(WIDTH as u32, HEIGHT as u32, |x, y| {
        render_pixel(x as f32, y as f32, scene)
    });

    img.save("test.png").unwrap();
}

fn render_pixel(x: f32, y: f32, scene: &Scene) -> image::Rgb<u8> {
    let direction = Vec3::new(
        (x + 0.5) - WIDTH as f32 / 2.,
        -(y + 0.5) + HEIGHT as f32 / 2.,
        -(HEIGHT as f32) / (2. * (FOV / 2.).tan())
    ).normalize();

    let origin = Vec3::new(scene.camera.position.x, scene.camera.position.y, scene.camera.position.z);
    let mut ray = Ray::new(origin, direction, std::f32::MAX);

    let pixel = cast_ray(&mut ray, &scene.objects);

    return pixel;
}

fn cast_ray(ray: &mut Ray, scene_objects: &Vec<Sphere>) -> image::Rgb<u8> {
    let mut pixel = image::Rgb([185, 185, 185]);

    for sphere in scene_objects {
        if sphere.ray_intersect(ray) {
            pixel = image::Rgb([sphere.color.0, sphere.color.1, sphere.color.2])
        }
    }

    return pixel;
}

fn create_scene() -> Scene {
    let camera = Camera::new(Vec3::new(0., 0., 0.,));

    let coordinates = Vec3::new(-3.0, 0., -16.0);
    let dark_green = (12 as u8, 55 as u8, 44 as u8);
    let test_sphere = Sphere::new(coordinates, 2.0, dark_green);

    let coordinates2 = Vec3::new(4.0, 4., -12.0);
    let metallic_red = (170 as u8, 84 as u8, 84 as u8);
    let test_sphere2 = Sphere::new(coordinates2, 3.0, metallic_red);

    let light = Light::new(Vec3::new(500., 500., 150.), 20.);

    Scene {
        lights: vec![light],
        camera,
        objects: vec![test_sphere, test_sphere2]
    }
}