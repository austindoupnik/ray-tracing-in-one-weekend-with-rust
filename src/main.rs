use std::borrow::BorrowMut;
use std::io;
use std::rc::Rc;

use crate::camera::Camera;
use crate::color::{Color, write_color};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::point3::Point3;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

mod vec3;
mod color;
mod ray;
mod point3;
mod hittable;
mod sphere;
mod hittable_list;
mod random;
mod camera;
mod material;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation = Color::new(0.0, 0.0, 0.0);

        if rec.mat_ptr.as_ref().unwrap().scatter(r, &rec, &mut attenuation, &mut scattered) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = Vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;

    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground.clone())));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center.clone())));
    world.add(Rc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left.clone())));
    world.add(Rc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.45, material_left.clone())));
    world.add(Rc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right.clone())));
    let world = world;

    let lookfrom = Point3::new(3.0, 3.0, 2.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (lookfrom - lookat).length();

    let cam = Camera::new(lookfrom, lookat, Vec3::new(0.0, 1.0, 0.0), 20.0, aspect_ratio, 2.0, dist_to_focus);

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random::random()) / (image_width - 1) as f64;
                let v = (j as f64 + random::random()) as f64 / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            write_color(io::stdout().borrow_mut(), pixel_color, samples_per_pixel).unwrap();
        }
    }

    eprint!("\nDone.\n");
}
