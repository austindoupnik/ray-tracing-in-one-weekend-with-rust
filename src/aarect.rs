use std::rc::Rc;
use crate::material::Material;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::aabb::Aabb;
use crate::point3::Point3;
use crate::vec3::Vec3;

pub struct XyRect {
    mp: Rc<dyn Material>,

    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,

    k: f64,
}

impl XyRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mp: Rc<dyn Material>) -> Self {
        XyRect {
            x0,
            x1,
            y0,
            y1,
            k,
            mp,
        }
    }
}

impl Hittable for XyRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max {
            return false;
        }

        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }

        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        rec.set_face_normal(r, &Vec3::new(0.0, 0.0, 1.0));
        rec.mat_ptr = Option::Some(self.mp.clone());
        rec.p = r.at(t);

        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(Point3::new(self.x0, self.y0, self.k - 0.0001), Point3::new(self.x1, self.y1, self.k + 0.0001));
        true
    }
}

pub struct XzRect {
    mp: Rc<dyn Material>,

    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,

    k: f64,
}

impl XzRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mp: Rc<dyn Material>) -> Self {
        XzRect {
            x0,
            x1,
            z0,
            z1,
            k,
            mp,
        }
    }
}

impl Hittable for XzRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t > t_max {
            return false;
        }

        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }

        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        rec.set_face_normal(r, &Vec3::new(0.0, 1.0, 0.0));
        rec.mat_ptr = Option::Some(self.mp.clone());
        rec.p = r.at(t);

        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(Point3::new(self.x0, self.z0, self.k - 0.0001), Point3::new(self.x1, self.z1, self.k + 0.0001));
        true
    }
}

pub struct YzRect {
    mp: Rc<dyn Material>,

    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,

    k: f64,
}

impl YzRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mp: Rc<dyn Material>) -> Self {
        YzRect {
            y0,
            y1,
            z0,
            z1,
            k,
            mp,
        }
    }
}

impl Hittable for YzRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t_min || t > t_max {
            return false;
        }

        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }

        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        rec.set_face_normal(r, &Vec3::new(1.0, 0.0, 0.0));
        rec.mat_ptr = Option::Some(self.mp.clone());
        rec.p = r.at(t);

        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(Point3::new(self.y0, self.z0, self.k - 0.0001), Point3::new(self.y1, self.z1, self.k + 0.0001));
        true
    }
}
