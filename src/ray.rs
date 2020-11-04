use crate::point3::Point3;
use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3, time: f64) -> Ray {
        Ray {
            origin,
            dir,
            time,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.dir
    }

    pub fn time(&self) -> f64 {
        self.time
    }
}