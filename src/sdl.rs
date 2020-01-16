// extern crate sdl2;

// use sdl2::pixels::Color;
// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;

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