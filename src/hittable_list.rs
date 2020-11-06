use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::aabb::Aabb;
use crate::point3::Point3;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: vec![],
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for object in self.objects.as_slice() {
            let mut temp_rec = HitRecord::new();
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if self.objects.is_empty() {
            return false
        }

        let mut first_box = true;
        for object in self.objects.as_slice() {
            let mut temp_box = Aabb::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0));
            if !object.bounding_box(time0, time1, &mut temp_box) {
                return false
            }

            *output_box = if first_box {
                temp_box
            } else {
                Aabb::surrounding_box(output_box, &temp_box)
            };
            first_box = false;
        }

        true
    }
}