use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            mat_ptr,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = f64::sqrt(discriminant);

            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.mat_ptr = Some(self.mat_ptr.clone());
                return true;
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                rec.mat_ptr = Some(self.mat_ptr.clone());
                return true;
            }
        }

        return false;
    }
}