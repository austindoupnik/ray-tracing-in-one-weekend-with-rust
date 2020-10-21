use crate::vec3::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

impl Ray {
    #[allow(dead_code)]
    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    #[allow(dead_code)]
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.dir
    }
}