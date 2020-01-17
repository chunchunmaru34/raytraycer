extern crate sdl2;

use crate::geometry::vec3::Vec3;
use crate::renderer;
use crate::scene::Scene;
use crate::utils::rgb::RGB;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

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

pub fn run_sdl(mut scene: Scene) {
    let (mut canvas, sdl_context) = init_sdl(scene.canvas.height as u32, scene.canvas.width as u32);
    let mut event_pump = sdl_context.event_pump().unwrap();

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
        // scene = render(scene, &mut canvas);
        scene = render_parallel(scene, &mut canvas);
        let duration = start.elapsed();

        println!(
            "fps:{0:.2}, time per frame:{1}ms",
            1000. / duration.as_millis() as f32,
            duration.as_millis()
        );
        println!(
            "x:{}, y:{}, z:{}",
            scene.camera.position.x, scene.camera.position.y, scene.camera.position.z
        );
    }
}

fn render_parallel(scene: Scene, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Scene {
    let mut handles = Vec::new();
    let cpus_count = num_cpus::get();
    let chunks_arc = Arc::new(Mutex::new(Vec::with_capacity(cpus_count)));
    // let mut chunks = Vec::with_capacity(cpus_count);
    let chunk_length = scene.canvas.height / cpus_count;

    let (width, height) = (scene.canvas.width, scene.canvas.height);

    let scene_arc = Arc::new(scene);

    (0..cpus_count).for_each(|cpu_num| {
        let (chunks, scene) = (chunks_arc.clone(), scene_arc.clone());

        let mut chunk = Vec::with_capacity(chunk_length);
        let start = cpu_num * chunk_length;
        let end = start + chunk_length;

        let handle = thread::spawn(move || {
            for y in start..end {
                let mut row = Vec::with_capacity(scene.canvas.width);

                for x in 0..scene.canvas.width {
                    let color = renderer::render_pixel(x as f32, y as f32, &scene);
                    row.push(color);
                }

                chunk.push(row);
            }

            chunks.lock().unwrap().push((cpu_num, chunk));
        });

        handles.push(handle);
    });

    for handle in handles {
        handle.join().unwrap();
    }

    let chunks_mutex = Arc::try_unwrap(chunks_arc).ok().unwrap();
    let mut chunks_order = chunks_mutex.lock().unwrap();
    chunks_order.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let chunks: Vec<Vec<RGB>> = chunks_order.iter().fold(Vec::new(), |acc, item| {
        [acc.as_slice(), item.1.as_slice()].concat()
    });

    for y in 0..height {
        for x in 0..width {
            let color = chunks.get(y).unwrap().get(x).unwrap();
            canvas.set_draw_color(Color::RGB(color.r, color.g, color.b));
            canvas
                .draw_point(sdl2::rect::Point::new(x as i32, y as i32))
                .unwrap();
        }
    }

    canvas.present();
    Arc::try_unwrap(scene_arc).ok().unwrap()
}

fn render(scene: Scene, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Scene {
    for y in 0..scene.canvas.height {
        for x in 0..scene.canvas.width {
            let color = renderer::render_pixel(x as f32, y as f32, &scene);
            canvas.set_draw_color(Color::RGB(color.r, color.g, color.b));
            canvas
                .draw_point(sdl2::rect::Point::new(x as i32, y as i32))
                .unwrap();
        }
    }

    canvas.present();

    scene
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
