use std::io;
use std::borrow::BorrowMut;
use crate::color::{write_color, Color};
use crate::vec3::{unit_vector, Vec3, dot};
use crate::ray::Ray;
use crate::point3::Point3;

mod vec3;
mod color;
mod ray;
mod point3;

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - *center;
    let a = dot(r.direction(), r.direction());
    let b = 2.0 * dot(oc, r.direction());
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    return discriminant > 0.0;
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    let origin: Point3  = Point3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3  = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical: Vec3  = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);

    let lower_left_corner: Vec3  = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH );

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;

            let r = ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color = ray_color(&r);
            write_color(io::stdout().borrow_mut(), pixel_color).unwrap();
        }
    }

    eprint!("\nDone.\n");
}
