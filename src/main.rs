extern crate image;
extern crate num_cpus;
extern crate sdl2;

mod geometry;
mod scene;
mod utils;

use geometry::sphere::{Sphere};
use geometry::ray::{Ray};
use geometry::vec3::{Vec3};
use scene::light::{Light};
use scene::camera::{Camera};
use scene::{Scene};

use std::time::{Instant, Duration};

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const HEIGHT: usize = 720;
const WIDTH: usize = 1280;
const FOV: f32 = 3.14 / 3.;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string()).unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut scene = create_scene();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    scene.camera.move_by(&Vec3::new(1., 0., 0.));
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    scene.camera.move_by(&Vec3::new(-1., 0., 0.));
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    scene.camera.move_by(&Vec3::new(0., 1., 0.));
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    scene.camera.move_by(&Vec3::new(0., -1., 0.));
                },
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    scene.camera.move_by(&Vec3::new(0., 0., 1.));
                },
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    scene.camera.move_by(&Vec3::new(0., 0., -1.));
                },
                Event::KeyDown { keycode: Some(Keycode::KpPlus), .. } => {
                    let old_intensity = scene.lights[0].intensity;
                    scene.lights[0].set_intensity(old_intensity + 0.1);
                },
                Event::KeyDown { keycode: Some(Keycode::KpMinus), .. } => {
                    let old_intensity = scene.lights[0].intensity;
                    scene.lights[0].set_intensity(old_intensity - 0.1);
                },
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        
        let start = Instant::now();

        render(&scene, &mut canvas);
        let duration = start.elapsed();

        println!("fps:{0:.2}, time per frame:{1}ms", 1000. / duration.as_millis() as f32, duration.as_millis());
        println!("x:{}, y:{}, z:{}", scene.camera.position.x, scene.camera.position.y, scene.camera.position.z);
    }

    Ok(())
}

fn render(scene: &Scene, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = render_pixel(x as f32, y as f32, scene);
            canvas.set_draw_color(Color::RGB(color.0, color.1, color.2));
            canvas.draw_point(sdl2::rect::Point::new(x as i32, y as i32)).unwrap();
        }
    }

    canvas.present();
}

fn render_pixel(x: f32, y: f32, scene: &Scene) -> (u8, u8, u8) {
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

fn cast_ray(ray: &mut Ray, scene: &Scene) -> (u8, u8, u8) {
    let mut pixel = Vec3::new(185., 185., 185.);

    for sphere in &scene.objects {
        if sphere.ray_intersect(ray) {
            pixel = Vec3::new(sphere.color.0 as f32, sphere.color.1 as f32, sphere.color.2 as f32);
            let diffuse_light_intensity = get_light_intensity(ray, sphere, &scene.lights);
            pixel = pixel.scale(diffuse_light_intensity);

            utils::limit_color((pixel.x, pixel.y, pixel.z));
        }
    }

    return (pixel.x as u8, pixel.y as u8, pixel.z as u8);
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