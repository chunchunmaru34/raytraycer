extern crate image;
extern crate num_cpus;
// extern crate sdl2;

mod geometry;
mod scene;
mod utils;

use geometry::sphere::{Sphere};
use geometry::ray::{Ray};
use geometry::vec3::{Vec3};
use scene::light::{Light};
use scene::camera::{Camera};
use scene::{Scene};
use utils::material_factory;

use std::time::{Instant, Duration};

use image::{ImageBuffer};

// use sdl2::pixels::Color;
// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;

const HEIGHT: usize = 1080;
const WIDTH: usize = 1920;
const FOV: f32 = 3.14 / 3.;
const BACKGROUND_COLOR: (u8, u8, u8) = (50, 178, 203);
const MAX_REFLECTIONS_ALLOWED: usize = 4;

fn main() -> Result<(), String> {
    run_static();

    Ok(())
}

fn run_static() {
    let scene = create_scene();
    let start = Instant::now();

    render_static(&scene);
    let duration = start.elapsed();
    println!("time per frame:{0}ms", duration.as_millis());
}

// fn run_sdl() {
//     let sdl_context = sdl2::init().unwrap();
//     let video_subsystem = sdl_context.video().unwrap();

//     let window = video_subsystem.window("rust-sdl2 demo: Video", WIDTH as u32, HEIGHT as u32)
//         .position_centered()
//         .opengl()
//         .build()
//         .map_err(|e| e.to_string())
//         .unwrap();

//     let mut canvas = window.into_canvas().build().map_err(|e| e.to_string()).unwrap();

//     canvas.set_draw_color(Color::RGB(0, 0, 0));
//     canvas.clear();
//     canvas.present();
//     let mut event_pump = sdl_context.event_pump().unwrap();

//     let mut scene = create_scene();

//     'running: loop {
//         for event in event_pump.poll_iter() {
//             match event {
//                 Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
//                     break 'running
//                 },
//                 Event::KeyDown { keycode: Some(Keycode::D), .. } => {
//                     scene.camera.move_by(&Vec3::new(1., 0., 0.));
//                 },
//                 Event::KeyDown { keycode: Some(Keycode::A), .. } => {
//                     scene.camera.move_by(&Vec3::new(-1., 0., 0.));
//                 },
//                 Event::KeyDown { keycode: Some(Keycode::W), .. } => {
//                     scene.camera.move_by(&Vec3::new(0., 1., 0.));
//                 },
//                 Event::KeyDown { keycode: Some(Keycode::S), .. } => {
//                     scene.camera.move_by(&Vec3::new(0., -1., 0.));
//                 },
//                 Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
//                     scene.camera.move_by(&Vec3::new(0., 0., 1.));
//                 },
//                 Event::KeyDown { keycode: Some(Keycode::X), .. } => {
//                     scene.camera.move_by(&Vec3::new(0., 0., -1.));
//                 },
//                 Event::KeyDown { keycode: Some(Keycode::KpPlus), .. } => {
//                     for light in &mut scene.lights {
//                         let old_intensity = light.intensity;
//                         light.set_intensity(old_intensity + 0.1);
//                     }
//                 },
//                 Event::KeyDown { keycode: Some(Keycode::KpMinus), .. } => {
//                     for light in &mut scene.lights {
//                         let old_intensity = light.intensity;
//                         light.set_intensity(old_intensity - 0.1);
//                     }
//                 },
//                 Event::KeyDown { keycode: Some(Keycode::Kp4), .. } => {
//                     scene.camera.rotate_by(&Vec3::new(0., 1., 0.));
//                 },
//                 Event::KeyDown { keycode: Some(Keycode::Kp6), .. } => {
//                     scene.camera.rotate_by(&Vec3::new(0., -1., 0.));
//                 },
//                 Event::KeyDown { keycode: Some(Keycode::Kp8), .. } => {
//                     scene.camera.rotate_by(&Vec3::new(0., 0., 1.));
//                 },
//                 Event::KeyDown { keycode: Some(Keycode::Kp2), .. } => {
//                     scene.camera.rotate_by(&Vec3::new(0., 0., -1.));
//                 },
//                 Event::KeyDown { keycode: Some(Keycode::Kp7), .. } => {
//                     scene.camera.rotate_by(&Vec3::new(1., 0., 0.));
//                 },
//                 Event::KeyDown { keycode: Some(Keycode::Kp9), .. } => {
//                     scene.camera.rotate_by(&Vec3::new(-1., 0., 0.));
//                 },
//                 _ => {}
//             }
//         }
//         ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        
//         let start = Instant::now();

//         render(&scene, &mut canvas);
//         let duration = start.elapsed();

//         println!("fps:{0:.2}, time per frame:{1}ms", 1000. / duration.as_millis() as f32, duration.as_millis());
//         println!("x:{}, y:{}, z:{}", scene.camera.position.x, scene.camera.position.y, scene.camera.position.z);
//     }
// }

// fn render(scene: &Scene, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
//     for y in 0..HEIGHT {
//         for x in 0..WIDTH {
//             let color = render_pixel(x as f32, y as f32, scene);
//             canvas.set_draw_color(Color::RGB(color.0, color.1, color.2));
//             canvas.draw_point(sdl2::rect::Point::new(x as i32, y as i32)).unwrap();
//         }
//     }

//     canvas.present();
// }

fn render_static(scene: &Scene) {
    let img = ImageBuffer::from_fn(WIDTH as u32, HEIGHT as u32, |x, y| {
        let color = render_pixel(x as f32, y as f32, scene);
        image::Rgb([color.0 as u8, color.1 as u8, color.2 as u8])
    });

    img.save("test.png").unwrap();
}

fn render_pixel(x: f32, y: f32, scene: &Scene) -> (u8, u8, u8) {
    let direction = Vec3::new(
        (x + 0.5) - WIDTH as f32 / 2.,
        -(y + 0.5) + HEIGHT as f32 / 2.,
        -(HEIGHT as f32) / (2. * (FOV / 2.).tan())
    // ).scale_by_matrix(scene.camera.rotation_matrix).normalize();
    ).normalize();

    let origin = Vec3::new(scene.camera.position.x, scene.camera.position.y, scene.camera.position.z);
    let mut ray = Ray::new(origin, direction, std::f32::MAX);

    let pixel = cast_ray(&mut ray, scene, 0);

    return pixel;
}

fn cast_ray(ray: &mut Ray, scene: &Scene, depth: usize) -> (u8, u8, u8) {
    if depth > MAX_REFLECTIONS_ALLOWED {
        return BACKGROUND_COLOR;
    }

    for sphere in &scene.objects {
        if sphere.ray_intersect(ray) {
            return get_pixel_color(ray, sphere, &scene, depth);
        }
    }

    BACKGROUND_COLOR
}

fn scene_intersects(ray: &mut Ray, scene: &Scene) -> bool {
    for sphere in &scene.objects {
        if sphere.ray_intersect(ray) {
            return true;
        }
    }

    false
}

fn get_pixel_color(ray: &Ray, sphere: &Sphere, scene: &Scene, depth: usize) -> (u8, u8, u8) {
    let hit_point = ray.origin.plus(&ray.direction.scale(ray.t));
    let hit_normal = hit_point.minus(&sphere.center).normalize();

    let reflect_direction = reflect(&ray.direction, &hit_normal).normalize();
    let refract_direction = refract(&ray.direction, &hit_normal, sphere.material.refractive_index, 1.).normalize();
    let reflect_origin = if reflect_direction.dot_product(&hit_normal) < 0. {
        hit_point.minus(&hit_normal.scale(1e-3))
    } else {
        hit_point.plus(&hit_normal.scale(1e-3))
    };
    let refract_origin = if refract_direction.dot_product(&hit_normal) < 0. {
        hit_point.minus(&hit_normal.scale(1e-3))
    } else {
        hit_point.plus(&hit_normal.scale(1e-3))
    };
    let mut reflected_ray = Ray::new(reflect_origin, reflect_direction, std::f32::MAX);
    let mut refracted_ray = Ray::new(refract_origin, refract_direction, std::f32::MAX);
    let reflect_color = cast_ray(&mut reflected_ray, &scene, depth + 1);
    let refract_color = cast_ray(&mut refracted_ray, &scene, depth + 1);

    let mut diffuse_light_intensity = 0.;
    let mut specular_light_intensity = 0.;

    for light in &scene.lights {
        let light_direction = light.position.minus(&hit_point).normalize();

        let shadow_origin = if light_direction.dot_product(&hit_normal) < 0. {
            hit_point.minus(&hit_normal.scale(1e-3))
        } else {
            hit_point.plus(&hit_normal.scale(1e-3))
        };
        let mut bounced_light_ray = Ray::new(shadow_origin, light_direction.clone(), std::f32::MAX);

        if scene_intersects(&mut bounced_light_ray, scene) {
            continue;
        };

        diffuse_light_intensity += light.intensity * f32::max(0., light_direction.dot_product(&hit_normal));
        specular_light_intensity += f32::max(0., reflect(&light_direction, &hit_normal).dot_product(&ray.direction))
            .powf(sphere.material.specular_exponent) * light.intensity;
    }

    let pixel = sphere.material.color
        .scale(diffuse_light_intensity * sphere.material.albedo.0)
        .plus(&Vec3::new(255., 255., 255.).scale(specular_light_intensity * sphere.material.albedo.1))
        .plus(&Vec3::new(reflect_color.0 as f32, reflect_color.1 as f32, reflect_color.2 as f32).scale(sphere.material.albedo.2))
        .plus(&Vec3::new(refract_color.0 as f32, refract_color.1 as f32, refract_color.2 as f32).scale(sphere.material.albedo.3));

    utils::limit_color(pixel)
}

fn reflect(light: &Vec3, normal: &Vec3) -> Vec3 {
    light.minus(&normal.scale(2. * light.dot_product(normal)))
}

fn refract(light: &Vec3, normal: &Vec3, eta_t: f32, eta_i: f32)  -> Vec3 {
    let cosi = -f32::max(-1., f32::min(1., light.dot_product(&normal)));
    if cosi < 0. {
        return refract(light, &normal.scale(-1.), eta_i, eta_t);
    };

    let eta = eta_i / eta_t;
    let k = 1. - eta.powi(2) * (1. - cosi.powi(2));

    if k < 0. {
        Vec3::new(1., 0., 0.)
    } else {
        light.scale(eta).plus(&normal.scale(eta * cosi - k.sqrt()))
    }
}

fn create_scene() -> Scene {
    let camera = Camera::new(Vec3::new(0., 0., 0.,));

    let test_sphere = Sphere::new(
        Vec3::new(-3.0, 0., -16.0),
        2.0,
        material_factory::get_dark_green_plastic()
    );
    let test_sphere2 = Sphere::new(
        Vec3::new(-1.0, -1.5, -12.0),
        2.0,
        material_factory::get_glass()
    );
    let test_sphere3 = Sphere::new(
        Vec3::new(1.5, -0.5, -18.0),
        3.0,
        material_factory::get_red_rubber(),
    );
    let test_sphere4 = Sphere::new(
        Vec3::new(7., 5., -18.0),
        4.0,
        material_factory::get_mirror(),
    );

    let mut objects = vec![test_sphere, test_sphere2, test_sphere3, test_sphere4]; 
    objects.sort_by(|sphere1, sphere2| {
        let a = sphere1.center.minus(&camera.position).length();
        let b = sphere2.center.minus(&camera.position).length();

        a.partial_cmp(&b).unwrap()
    });

    let lights = vec![
        Light::new(Vec3::new(-20., 20., 20.), 1.5),
        Light::new(Vec3::new(30., 50., -25.), 1.8),
        Light::new(Vec3::new(30., 20., 30.), 1.7),
    ];

    Scene {
        lights,
        camera,
        objects,
    }
}