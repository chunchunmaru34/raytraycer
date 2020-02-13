extern crate image;
extern crate num_cpus;

mod geometry;
mod renderer;
mod scene;
mod sdl;
mod utils;

use geometry::sphere::Sphere;
use geometry::vec3::Vec3;
use scene::camera::Camera;
use scene::light::Light;
use scene::{Canvas, Scene, SceneOptions};
use std::sync::Arc;
use std::time::Instant;
use utils::material_factory;
use utils::rgb::RGB;

const HEIGHT: usize = 720;
const WIDTH: usize = 1280;
const FOV: f32 = 3.14 / 3.;
const BACKGROUND_COLOR: RGB = RGB::new(178, 178, 178);
const MAX_REFLECTIONS_ALLOWED: usize = 4;

fn main() -> Result<(), String> {
    // run_static();
    run_dynamic();

    Ok(())
}

fn run_static() {
    let scene = Arc::new(create_scene());
    let start = Instant::now();

    render_static(&scene);
    let duration = start.elapsed();
    println!("time per frame:{0}ms", duration.as_millis());
}

fn render_static(scene: &Arc<Scene>) {
    let buffer = renderer::render_frame(scene);

    buffer.save("test.png").unwrap();
}

fn run_dynamic() {
    sdl::run_sdl(create_scene());
}

fn create_scene() -> Scene {
    let camera = Camera::new(Vec3::new(0., 0., 0.));

    let mut objects = vec![
        Sphere::new(
            Vec3::new(-3.0, 0., -16.0),
            2.0,
            material_factory::get_dark_green_plastic(),
        ),
        Sphere::new(
            Vec3::new(-1.0, -1.5, -12.0),
            2.0,
            material_factory::get_glass(),
        ),
        Sphere::new(
            Vec3::new(1.5, -0.5, -18.0),
            3.0,
            material_factory::get_red_rubber(),
        ),
        Sphere::new(
            Vec3::new(7., 5., -18.0),
            4.0,
            material_factory::get_mirror(),
        ),
    ];

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
