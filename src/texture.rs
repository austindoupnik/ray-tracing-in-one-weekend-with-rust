use crate::color::Color;
use crate::point3::Point3;
use std::rc::Rc;

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
