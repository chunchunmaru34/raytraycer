extern crate image;
extern crate num_cpus;

mod geometry;
mod camera;
mod scene;

use camera::{Camera};
use geometry::{Sphere, Ray, Vec3, Light};
use image::{ImageBuffer};
use scene::{Scene};
use std::time::{Instant};

const HEIGHT: usize = 900;
const WIDTH: usize = 1600;
const FOV: f32 = 3.14 / 3.;

fn main() {
    let mut scene = create_scene();
    // loop {
        let start = Instant::now();
        // scene.camera.move_by(&Vec3::new(10.01, 5.01, -0.01));
        render(&scene);
        let duration = start.elapsed();
        println!("{}", duration.as_millis());
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

    let pixel = cast_ray(&mut ray, scene);

    return pixel;
}

fn cast_ray(ray: &mut Ray, scene: &Scene) -> image::Rgb<u8> {
    let mut pixel = Vec3::new(185., 185., 185.);

    for sphere in &scene.objects {
        if sphere.ray_intersect(ray) {
            pixel = Vec3::new(sphere.color.0 as f32, sphere.color.1 as f32, sphere.color.2 as f32);
            let diffuse_light_intensity = get_light_intensity(ray, sphere, &scene.lights);
            pixel = pixel.scale(diffuse_light_intensity);

            return image::Rgb([pixel.x as u8, pixel.y as u8, pixel.z as u8]);
        }
    }

    return image::Rgb([pixel.x as u8, pixel.y as u8, pixel.z as u8]);
}

fn get_light_intensity(ray: &Ray, sphere: &Sphere, lights: &Vec<Light>) -> f32 {
    let hit_point = ray.origin.plus(&ray.direction.scale(ray.t));
    let hit_normal = hit_point.minus(&sphere.center).normalize();

    let mut diffuse_light_intensity = 0.;
    for light in lights {
        let light_direction = light.position.minus(&hit_point).normalize();
        diffuse_light_intensity += light.intensity * f32::max(0., light_direction.dot_product(&hit_normal));
    }

    diffuse_light_intensity
}

fn create_scene() -> Scene {
    let camera = Camera::new(Vec3::new(0., 0., 0.,));

    let coordinates = Vec3::new(-3.0, 0., -16.0);
    let dark_green = (12 as u8, 55 as u8, 44 as u8);
    let test_sphere = Sphere::new(coordinates, 2.0, dark_green);

    let coordinates2 = Vec3::new(-1.0, -1.5, -12.0);
    let metallic_red = (170 as u8, 84 as u8, 84 as u8);
    let test_sphere2 = Sphere::new(coordinates2, 2.0, metallic_red);

    let coordinates3 = Vec3::new(1.5, -0.5, -18.0);
    let test_sphere3 = Sphere::new(coordinates3, 3.0, dark_green);

    let coordinates4 = Vec3::new(7., 5., -18.0);
    let test_sphere4 = Sphere::new(coordinates4, 4.0, metallic_red);

    let light = Light::new(Vec3::new(-20., 20., 20.), 1.5);

    let mut objects = vec![test_sphere, test_sphere2, test_sphere3, test_sphere4]; 
    objects.sort_by(|sphere1, sphere2| {
        let a = sphere1.center.minus(&camera.position).length();
        let b = sphere2.center.minus(&camera.position).length();

        a.partial_cmp(&b).unwrap()
    });

    Scene {
        lights: vec![light],
        camera,
        objects,
    }
}