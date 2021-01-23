use crate::geometry::ray::Ray;
use crate::geometry::sphere::Sphere;
use crate::geometry::vec3::Vec3;
use crate::scene::Scene;
use crate::utils;
use crate::utils::rgb::RGB;

use image::{ImageBuffer, Rgb};
use std::sync::mpsc::channel;
use std::sync::Arc;
use threadpool::ThreadPool;

pub fn render_frame(scene: &Arc<Scene>, pool: &ThreadPool) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = (scene.canvas.width, scene.canvas.height);
    let cpus_count = num_cpus::get();
    let chunk_length = height / cpus_count;

    let (tx, rx) = channel();

    (0..cpus_count).for_each(|worker_num| {
        let scene = scene.clone();
        let tx = tx.clone();

        let mut chunk = Vec::with_capacity(chunk_length);
        let start = worker_num * chunk_length;
        let end = start + chunk_length;

        pool.execute(move || {
            for y in start..end {
                let mut row = Vec::with_capacity(scene.canvas.width);

                for x in 0..width {
                    let color = render_pixel(x as f32, y as f32, &scene);
                    row.push(color);
                }

                chunk.push(row);
            }

            tx.send((worker_num, chunk))
                .expect("Something went wrong while calculating chunk in pool");
        });
    });

    let mut buffer = ImageBuffer::new(width as u32, height as u32);

    for (worker_num, chunk) in rx.iter().take(cpus_count) {
        let start = worker_num * chunk_length;
        let end = start + chunk_length;

        for y in start..end {
            for x in 0..width {
                let color = chunk.get(y - start).unwrap().get(x).unwrap();
                let pixel = Rgb(color.as_array());
                buffer.put_pixel(x as u32, y as u32, pixel);
            }
        }
    }

    buffer
}

pub fn render_pixel(x: f32, y: f32, scene: &Scene) -> RGB {
    let direction = Vec3::new(
        (x + 0.5) - scene.canvas.width as f32 / 2.,
        -(y + 0.5) + scene.canvas.height as f32 / 2.,
        -(scene.canvas.height as f32) / (2. * (scene.canvas.fov / 2.).tan()), // ).scale_by_matrix(scene.camera.rotation_matrix).normalize();
    )
    .normalize();

    let origin = Vec3::new(
        scene.camera.position.x,
        scene.camera.position.y,
        scene.camera.position.z,
    );
    let mut ray = Ray::new(origin, direction, std::f32::MAX);

    let pixel = cast_ray(&mut ray, scene, 0);

    return pixel;
}

fn cast_ray(ray: &mut Ray, scene: &Scene, depth: usize) -> RGB {
    if depth > scene.options.reflections_limit {
        return scene.options.background_color.clone();
    }

    let mut pairs: Vec<(f32, usize)> = scene
        .objects
        .iter()
        .enumerate()
        .map(|pair| (pair.1.center.minus(&ray.origin).length(), pair.0))
        .collect();
    pairs.sort_by(|pair1, pair2| pair1.0.partial_cmp(&pair2.0).unwrap());

    let order: Vec<usize> = pairs.iter().map(|pair| pair.1).collect();

    for index in order {
        let sphere = &scene.objects.get(index).unwrap();

        if sphere.ray_intersect(ray) {
            return get_pixel_color(ray, sphere, scene, depth);
        }
    }

    scene.options.background_color.clone()
}

fn scene_intersects(ray: &mut Ray, scene: &Scene) -> bool {
    for sphere in &scene.objects {
        if sphere.ray_intersect(ray) {
            return true;
        }
    }

    false
}

fn get_pixel_color(ray: &Ray, sphere: &Sphere, scene: &Scene, depth: usize) -> RGB {
    let hit_point = ray.origin.plus(&ray.direction.scale(ray.t));
    let hit_normal = hit_point.minus(&sphere.center).normalize();

    let reflect_direction = reflect(&ray.direction, &hit_normal).normalize();
    let refract_direction = refract(
        &ray.direction,
        &hit_normal,
        sphere.material.refractive_index,
        1.,
    )
    .normalize();
    let reflect_origin = utils::move_from_surface(&reflect_direction, &hit_normal, &hit_point);
    let refract_origin = utils::move_from_surface(&refract_direction, &hit_normal, &hit_point);
    let mut reflected_ray = Ray::new(reflect_origin, reflect_direction, std::f32::MAX);
    let mut refracted_ray = Ray::new(refract_origin, refract_direction, std::f32::MAX);

    let reflect_color = cast_ray(&mut reflected_ray, scene, depth + 1);
    let refract_color = cast_ray(&mut refracted_ray, scene, depth + 1);

    let mut diffuse_light_intensity = 0.;
    let mut specular_light_intensity = 0.;

    for light in &scene.lights {
        let light_direction = light.position.minus(&hit_point).normalize();
        let shadow_origin = utils::move_from_surface(&light_direction, &hit_normal, &hit_point);
        let mut bounced_light_ray = Ray::new(shadow_origin, light_direction.clone(), std::f32::MAX);

        if scene_intersects(&mut bounced_light_ray, scene) {
            continue;
        };

        diffuse_light_intensity +=
            light.intensity * f32::max(0., light_direction.dot_product(&hit_normal));
        specular_light_intensity += f32::max(
            0.,
            reflect(&light_direction, &hit_normal).dot_product(&ray.direction),
        )
        .powf(sphere.material.specular_exponent)
            * light.intensity;
    }

    let pixel = sphere
        .material
        .color
        .as_vector()
        .scale(diffuse_light_intensity * sphere.material.albedo.0)
        .plus(
            &Vec3::new(255., 255., 255.).scale(specular_light_intensity * sphere.material.albedo.1),
        )
        .plus(&reflect_color.as_vector().scale(sphere.material.albedo.2))
        .plus(&refract_color.as_vector().scale(sphere.material.albedo.3));

    RGB::from_vector(&pixel)
}

fn reflect(light: &Vec3, normal: &Vec3) -> Vec3 {
    light.minus(&normal.scale(2. * light.dot_product(normal)))
}

fn refract(light: &Vec3, normal: &Vec3, eta_t: f32, eta_i: f32) -> Vec3 {
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
