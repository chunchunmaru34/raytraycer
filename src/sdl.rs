extern crate image;
extern crate sdl2;

use crate::geometry::vec3::Vec3;
use crate::renderer;
use crate::scene::Scene;

use std::sync::Arc;
use std::time::{Duration, Instant};
use threadpool::ThreadPool;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

fn init_sdl(height: u32, width: u32) -> (Canvas<Window>, sdl2::Sdl) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", width, height)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    (canvas, sdl_context)
}

pub fn run_sdl(mut scene: Scene, no_parallel: bool) {
    let (mut canvas, sdl_context) = init_sdl(scene.canvas.height as u32, scene.canvas.width as u32);
    let mut event_pump = sdl_context.event_pump().unwrap();

    let cpus_count = if no_parallel { 1 } else { num_cpus::get() };
    let pool = ThreadPool::new(cpus_count);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => handle_event(event, &mut scene),
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        let start = Instant::now();
        scene = render(scene, &mut canvas, &pool);
        let duration = start.elapsed();

        println!(
            "fps:{0:.2}, time per frame:{1}ms",
            1000. / duration.as_millis() as f32,
            duration.as_millis()
        );
        // println!(
        //     "x:{}, y:{}, z:{}",
        //     scene.camera.position.x, scene.camera.position.y, scene.camera.position.z
        // );
    }
}

fn render(
    scene: Scene,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    pool: &ThreadPool,
) -> Scene {
    let (width, height) = (scene.canvas.width, scene.canvas.height);
    let scene_arc = Arc::new(scene);

    let buffer = renderer::render_frame(&scene_arc, &pool);

    for y in 0..height {
        for x in 0..width {
            let color = buffer.get_pixel(x as u32, y as u32);
            canvas.set_draw_color(Color::RGB(color[0], color[1], color[2]));
            canvas
                .draw_point(sdl2::rect::Point::new(x as i32, y as i32))
                .unwrap();
        }
    }
    canvas.present();
    Arc::try_unwrap(scene_arc).ok().unwrap()
}

fn handle_event(event: Event, scene: &mut Scene) {
    match event {
        Event::KeyDown {
            keycode: Some(Keycode::D),
            ..
        } => {
            scene.camera.move_by(&Vec3::new(1., 0., 0.));
        }
        Event::KeyDown {
            keycode: Some(Keycode::A),
            ..
        } => {
            scene.camera.move_by(&Vec3::new(-1., 0., 0.));
        }
        Event::KeyDown {
            keycode: Some(Keycode::W),
            ..
        } => {
            scene.camera.move_by(&Vec3::new(0., 1., 0.));
        }
        Event::KeyDown {
            keycode: Some(Keycode::S),
            ..
        } => {
            scene.camera.move_by(&Vec3::new(0., -1., 0.));
        }
        Event::KeyDown {
            keycode: Some(Keycode::Z),
            ..
        } => {
            scene.camera.move_by(&Vec3::new(0., 0., 1.));
        }
        Event::KeyDown {
            keycode: Some(Keycode::X),
            ..
        } => {
            scene.camera.move_by(&Vec3::new(0., 0., -1.));
        }
        Event::KeyDown {
            keycode: Some(Keycode::KpPlus),
            ..
        } => {
            for light in &mut scene.lights {
                let old_intensity = light.intensity;
                light.set_intensity(old_intensity + 0.1);
            }
        }
        Event::KeyDown {
            keycode: Some(Keycode::KpMinus),
            ..
        } => {
            for light in &mut scene.lights {
                let old_intensity = light.intensity;
                light.set_intensity(old_intensity - 0.1);
            }
        }
        Event::KeyDown {
            keycode: Some(Keycode::Kp4),
            ..
        } => {
            scene.camera.rotate_by(&Vec3::new(0., 1., 0.));
        }
        Event::KeyDown {
            keycode: Some(Keycode::Kp6),
            ..
        } => {
            scene.camera.rotate_by(&Vec3::new(0., -1., 0.));
        }
        Event::KeyDown {
            keycode: Some(Keycode::Kp8),
            ..
        } => {
            scene.camera.rotate_by(&Vec3::new(0., 0., 1.));
        }
        Event::KeyDown {
            keycode: Some(Keycode::Kp2),
            ..
        } => {
            scene.camera.rotate_by(&Vec3::new(0., 0., -1.));
        }
        Event::KeyDown {
            keycode: Some(Keycode::Kp7),
            ..
        } => {
            scene.camera.rotate_by(&Vec3::new(1., 0., 0.));
        }
        Event::KeyDown {
            keycode: Some(Keycode::Kp9),
            ..
        } => {
            scene.camera.rotate_by(&Vec3::new(-1., 0., 0.));
        }
        _ => {}
    }
}
