use crate::color::Color;
use crate::point3::Point3;
use std::rc::Rc;
use crate::perlin::Perlin;

pub trait Texture {
    fn value(self: &Self, u: f64, v: f64, p: &Point3) -> Color;
}

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(color_value: Color) -> Self {
        SolidColor {
            color_value
        }
    }
}

impl Texture for SolidColor {
    fn value(self: &Self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color_value
    }
}

pub struct CheckerTexture {
    odd: Rc<dyn Texture>,
    even: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Color, even: Color) -> Self {
        CheckerTexture {
            odd: Rc::new(SolidColor::new(odd)),
            even: Rc::new(SolidColor::new(even)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(self: &Self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = f64::sin(10.0 * p.x()) * f64::sin(10.0 * p.y()) * f64::sin(10.0 * p.z());
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(self: &Self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + f64::sin(self.scale * p.z() + 10.0 * self.noise.turb(p, Option::None)))
    }
}