use std::io::Write;

use crate::vec3::Vec3;

pub type Color = Vec3;

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn write_color(f: &mut impl Write, color: Color, samples_per_pixel: u32) -> std::io::Result<()> {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = f64::sqrt(color.x() * scale);
    let g = f64::sqrt(color.y() * scale);
    let b = f64::sqrt(color.z() * scale);

    write!(
        f,
        "{} {} {}\n",
        (256.0 * clamp(r, 0.0, 0.999)) as u32,
        (256.0 * clamp(g, 0.0, 0.999)) as u32,
        (256.0 * clamp(b, 0.0, 0.999)) as u32,
    )
}