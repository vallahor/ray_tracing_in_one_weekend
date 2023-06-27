mod camera;
mod color;
mod consts;
mod hittable;
mod hittable_list;
mod material;
mod math;
mod ray;
mod sphere;

use crate::camera::Camera;
use crate::color::*;
use crate::consts::*;
use crate::hittable::*;
use crate::hittable_list::*;
use crate::material::*;
use crate::math::vec3::Vec3;
use crate::ray::Ray;
use crate::sphere::*;
use crate::KeyboardKey::*;
use std::sync::Arc;

use raylib::prelude::*;

const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 800;

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let mut rec: HitRecord = Default::default();
    if world.hit(ray, 0.001, INFINITY, &mut rec) {
        let mut scattered: Ray = Default::default();
        let mut attenuation: Vec3 = Default::default();

        if let Some(mat) = &rec.mat {
            if mat.scatter(&ray, &rec, &mut attenuation, &mut scattered) {
                return attenuation * ray_color(&scattered, world, depth - 1);
            }
            return Vec3::new(0.0, 0.0, 0.0);
        }
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
}

fn main() {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200 as i32;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 30;
    let max_depth = 50;

    let (mut rl, thread) = raylib::init()
        .size(image_width as i32, image_height as i32)
        .title("ray tracing in one weekend")
        .build();

    let world = random_scene();

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.;

    // Camera
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let mut buffer =
        vec![vec![Color::new(0, 0, 0, 0); (image_width + 1) as usize]; (image_height + 1) as usize];

    let mut rendered = false;

    while !rl.window_should_close() {
        if rl.is_key_pressed(KEY_SPACE) {
            rendered = false;
        }

        if !rendered {
            for y in 0..image_height as usize {
                for x in 0..image_width as usize {
                    let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);

                    for _ in 0..samples_per_pixel {
                        let u = (x as f32 + random()) / image_width as f32;
                        let v = (y as f32 + random()) / image_height as f32;
                        let r = cam.get_ray(u, v);
                        pixel_color = pixel_color + ray_color(&r, &world, max_depth);
                    }

                    write_color(&mut pixel_color, samples_per_pixel);
                    buffer[image_height as usize - y][x] = Color::new(
                        pixel_color.x as u8,
                        pixel_color.y as u8,
                        pixel_color.z as u8,
                        255,
                    );
                }

                println!("{}% in Y.", (y as f32 / image_height as f32) * 100.0);
            }
            println!("AEHO");
            rendered = true;
        }

        let mut render = rl.begin_drawing(&thread);
        render.clear_background(Color::new(45, 42, 64, 255));
        render.draw_text("AEHO HORA DO SHOW PORRA", 20, 20, 20, Color::BLACK);

        for y in 0..image_height as usize {
            for x in 0..image_width as usize {
                let color = buffer[y as usize][x as usize];
                render.draw_pixel(x as i32, y as i32, color);
            }
        }
    }
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Vec3::new(a as f32 + 0.9 * random(), 0.2, b as f32 + 0.9 * random());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = random_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    return world;
}
