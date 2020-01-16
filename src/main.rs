extern crate image;
extern crate num_cpus;
// extern crate sdl2;

mod geometry;
mod renderer;
mod scene;
mod utils;

use geometry::sphere::Sphere;
use geometry::vec3::Vec3;
use scene::camera::Camera;
use scene::light::Light;
use scene::{Canvas, Scene, SceneOptions};
use std::time::{Duration, Instant};
use utils::material_factory;
use utils::rgb::RGB;

use image::ImageBuffer;

// use sdl2::pixels::Color;
// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;

const HEIGHT: usize = 1080;
const WIDTH: usize = 1920;
const FOV: f32 = 3.14 / 3.;
const BACKGROUND_COLOR: RGB = RGB::new(178, 178, 178);
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
        let color = renderer::render_pixel(x as f32, y as f32, scene);
        image::Rgb([color.r, color.g, color.b])
    });

    img.save("test.png").unwrap();
}

fn create_scene() -> Scene {
    let camera = Camera::new(Vec3::new(0., 0., 0.));

    let test_sphere = Sphere::new(
        Vec3::new(-3.0, 0., -16.0),
        2.0,
        material_factory::get_dark_green_plastic(),
    );
    let test_sphere2 = Sphere::new(
        Vec3::new(-1.0, -1.5, -12.0),
        2.0,
        material_factory::get_glass(),
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
        options: SceneOptions {
            reflections_limit: MAX_REFLECTIONS_ALLOWED,
            background_color: BACKGROUND_COLOR,
        },
        canvas: Canvas {
            width: WIDTH,
            height: HEIGHT,
            fov: FOV,
        },
    }
}
