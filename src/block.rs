use std::rc::Rc;

use crate::aabb::Aabb;
use crate::aarect::{XyRect, XzRect, YzRect};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;

pub struct Block {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList,
}

impl Block {
    pub fn new(p0: Point3, p1: Point3, ptr: Rc<dyn Material>) -> Self {
        let mut sides = HittableList::new();

        sides.add(Rc::new(XyRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p1.z(), ptr.clone())));
        sides.add(Rc::new(XyRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p0.z(), ptr.clone())));

        sides.add(Rc::new(XzRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p1.y(), ptr.clone())));
        sides.add(Rc::new(XzRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p0.y(), ptr.clone())));

        sides.add(Rc::new(YzRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p1.x(), ptr.clone())));
        sides.add(Rc::new(YzRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p0.x(), ptr.clone())));

        Block {
            box_min: p0,
            box_max: p1,
            sides,
        }
    }
}

impl Hittable for Block {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.sides.hit(r, t_min, t_max, rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(self.box_min, self.box_max);
        true
    }
}