extern crate image;
extern crate num_cpus;

mod sphere;
use sphere::{Sphere, Point, Light, Ray, Vec3};
use image::{ImageBuffer};
use std::time::{Instant};
// use std::thread;

const HEIGHT: usize = 512;
const WIDTH: usize = 512;

fn main() {
    let start = Instant::now();
    render_parallel();
    // render();
    let duration = start.elapsed();
    println!("{}", duration.as_millis());
}

// fn render() {
//     let coordinates = Point::new(150.0, 400.0, 32.0);
//     let radius = 120.0;
//     let color = (12 as u8, 55 as u8, 44 as u8);
//     let test_sphere = Sphere::new(coordinates, radius, color);

//     let coordinates2 = Point::new(350.0, 200.0, 12.0);
//     let radius2 = 76.0;
//     let color2 = (225 as u8, 12 as u8, 225 as u8);
//     let test_sphere2 = Sphere::new(coordinates2, radius2, color2);

//     let light = Light::new(Point::new(500., 500., 150.), 20.);

//     let img = ImageBuffer::from_fn(512, 512, |x, y| {
//         let direction = Point::new(x as f32, y as f32, 100.0);
//         let origin = Point::new(x as f32, y as f32, 0.);
//         let ray = Ray::new(origin, direction);

//         let mut pixel = image::Rgb([185, 185, 185]);

//         for sphere in vec![&test_sphere, &test_sphere2] {
//             if sphere.ray_intersect(&ray) {
//                 pixel = image::Rgb([sphere.color.0, sphere.color.1, sphere.color.2])
//             }
//         }

//         return pixel;
//     });

//     img.save("test.png").unwrap();
// }

fn render() {
    let coordinates = Vec3::new(10.0, 10.0, 12.0);
    let dark_green = (12 as u8, 55 as u8, 44 as u8);
    let test_sphere = Sphere::new(coordinates, 50.0, dark_green);

    let coordinates2 = Vec3::new(50.0, 60.0, 12.0);
    let pink = (225 as u8, 12 as u8, 225 as u8);
    let test_sphere2 = Sphere::new(coordinates2, 10.0, pink);

    // let light = Light::new(Point::new(500., 500., 150.), 20.);
    let scene = vec![&test_sphere, &test_sphere2];

    let img = ImageBuffer::from_fn(WIDTH as u32, HEIGHT as u32, |x, y| {
        render_pixel(x as f32, y as f32, &scene)
    });

    img.save("test.png").unwrap();
}

fn render_parallel() {
    // SCENE
    let coordinates = Vec3::new(10.0, 10.0, 12.0);
    let dark_green = (12 as u8, 55 as u8, 44 as u8);
    let test_sphere = Sphere::new(coordinates, 20.0, dark_green);

    let coordinates2 = Vec3::new(200.0, 300.0, 12.0);
    let pink = (225 as u8, 12 as u8, 225 as u8);
    let test_sphere2 = Sphere::new(coordinates2, 12.0, pink);

    // let light = Light::new(Point::new(500., 500., 150.), 20.);
    let scene = vec![&test_sphere, &test_sphere2];

    // RENDER
    let mut image_buffer = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    // let cpu_count = num_cpus::get();
    let cpu_count = 1;
    let chunk_size = ((HEIGHT as f32 / cpu_count as f32)).ceil() as usize;

    for i in 0..cpu_count {
        // thread::spawn( || {
            let start_point = i * chunk_size;
            for y in start_point..(start_point + chunk_size) {
                for x in 0..WIDTH {
                    let pixel = render_pixel(x as f32, y as f32, &scene);
                    image_buffer.put_pixel(x as u32, y as u32, pixel);
                }
            }
        // });
    }
}

fn render_pixel(x: f32, y: f32, scene: &Vec<&Sphere>) -> image::Rgb<u8> {
    let direction = Vec3::new(x, y , 1.).normalize();
    let origin = Vec3::new(x, y, 0.);
    // let origin = Vec3::new(0., 0., 0.);
    let mut ray = Ray::new(origin, direction, std::f32::MAX);

    let pixel = cast_ray(&mut ray, scene);

    return pixel;
}

fn cast_ray(ray: &mut Ray, scene: &Vec<&Sphere>) -> image::Rgb<u8> {
    let mut pixel = image::Rgb([185, 185, 185]);

    for sphere in scene {
        if sphere.ray_intersect(ray) {
            pixel = image::Rgb([sphere.color.0, sphere.color.1, sphere.color.2])
        }
    }

    return pixel;
}