extern crate image;

mod sphere;
use sphere::{Sphere, Point};
use image::{ImageBuffer};

fn main() {
    render();
}

fn render() {
    let coordinates = Point::new(150.0, 400.0, 32.0);
    let radius = 120.0;
    let color = (12 as u8, 55 as u8, 44 as u8);
    let test_sphere = Sphere::new(coordinates, radius, color);

    let coordinates2 = Point::new(350.0, 200.0, 12.0);
    let radius2 = 76.0;
    let color2 = (225 as u8, 12 as u8, 225 as u8);
    let test_sphere2 = Sphere::new(coordinates2, radius2, color2);


    let img = ImageBuffer::from_fn(512, 512, |x, y| {
        let direction = Point::new(x as f32, y as f32, 100.0);
        let origin = Point::new(x as f32, y as f32, 0.);
        let mut pixel = image::Rgb([185, 185, 185]);

        for sphere in vec![&test_sphere, &test_sphere2] {
            if sphere.ray_intersect(&origin, &direction) {
                pixel = image::Rgb([sphere.color.0, sphere.color.1, sphere.color.2])
            }
        }

        return pixel;
    });

    img.save("test.png").unwrap();
}

struct Light {
    pub position: Point,
    pub intensity: f32,
}

impl Light {
    pub fn new(position: Point, intensity: f32) -> Self {
        Self { position, intensity }
    }
}