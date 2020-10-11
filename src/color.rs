use std::io::Write;
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(f: &mut impl Write, color: Color) {
    write!(
        f,
        "{} {} {}\n",
        (255.999 * color.e[0]) as u32,
        (255.999 * color.e[1]) as u32,
        (255.999 * color.e[2]) as u32,
    );
}