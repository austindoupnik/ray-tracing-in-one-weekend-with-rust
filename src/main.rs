use std::io;
use std::borrow::BorrowMut;
use crate::color::{write_color, Color};
use crate::vec3::{unit_vector, Point3, Vec3};
use crate::ray::Ray;
use std::convert::TryInto;

mod vec3;
mod color;
mod ray;

fn ray_color(r: &Ray) -> Color {
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color { e: [1.0, 1.0, 1.0] } + t * Color { e: [0.5, 0.7, 1.0] }
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    const ORIGIN: Point3 = Point3 { e: [0.0, 0.0, 0.0] };
    const HORIZONTAL: Vec3 = Vec3 { e: [ VIEWPORT_WIDTH, 0.0, 0.0] };
    const VERTICAL: Vec3 = Vec3 { e: [ 0.0, VIEWPORT_HEIGHT, 0.0] };

    let lower_left_corner: Vec3 = ORIGIN - HORIZONTAL / 2.0 - VERTICAL / 2.0 - Vec3 { e: [0.0, 0.0, FOCAL_LENGTH ] };

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;

            let r = Ray { origin: ORIGIN, dir: lower_left_corner + u * HORIZONTAL + v * VERTICAL - ORIGIN };
            let pixel_color = ray_color(&r);
            write_color(io::stdout().borrow_mut(), pixel_color);
        }
    }

    eprint!("\nDone.\n");
}
