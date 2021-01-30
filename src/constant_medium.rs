use std::rc::Rc;
use crate::hittable::{Hittable, HitRecord};
use crate::material::{Material, Isotropic};
use crate::texture::Texture;
use crate::ray::Ray;
use crate::aabb::Aabb;
use crate::random::random;
use crate::vec3::Vec3;

pub struct ConstantMedium {
    boundary: Rc<dyn Hittable>,
    phase_function: Rc<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(b: Rc<dyn Hittable>, d: f64, a: Rc<dyn Texture>) -> Self {
        ConstantMedium {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Rc::new(Isotropic::new(a)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::new();
        if ! self.boundary.hit(r, f64::NEG_INFINITY, f64::INFINITY, &mut rec1) {
            return false;
        }

        let mut rec2 = HitRecord::new();
        if ! self.boundary.hit(r, rec1.t + 0.0001, f64::INFINITY, &mut rec2) {
            return false;
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        };
        if rec2.t > t_max {
            rec2.t = t_max;
        };

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random().log2();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p =  r.at(rec.t);

        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat_ptr = Option::Some(self.phase_function.clone());

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }
}