use std::rc::Rc;

use crate::aabb::Aabb;
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Option<Rc<dyn Material>>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
            normal: Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
            mat_ptr: None,
            t: f64::NEG_INFINITY,
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool;
}

pub struct Translate {
    ptr: Rc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(p: Rc<dyn Hittable>, displacement: Vec3) -> Self {
        Translate {
            ptr: p,
            offset: displacement,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());
        if !self.ptr.hit(&moved_r, t_min, t_max, rec) {
            return false;
        }

        rec.p += self.offset;
        let normal = rec.normal;
        rec.set_face_normal(&moved_r, &normal);

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if !self.ptr.bounding_box(time0, time1, output_box) {
            return false;
        }

        *output_box = Aabb::new(output_box.min() + self.offset, output_box.max() + self.offset);

        true
    }
}

pub struct RotateY {
    ptr: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    has_box: bool,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(p: Rc<dyn Hittable>, angle: f64) -> Self {
        let radians = f64::to_radians(angle);

        let sin_theta = f64::sin(radians);
        let cos_theta = f64::cos(radians);

        let mut bbox = Aabb::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0));
        let has_box = p.bounding_box(0.0, 1.0, &mut bbox);

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i = i as f64;
                    let x = i * bbox.min().x() + (1.0 - i) * bbox.min().x();

                    let j = j as f64;
                    let y = j * bbox.min().y() + (1.0 - j) * bbox.min().y();

                    let k = k as f64;
                    let z = k * bbox.min().z() + (1.0 - k) * bbox.min().z();

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    min = Point3::new(
                        f64::min(min[0], tester[0]),
                        f64::min(min[1], tester[1]),
                        f64::min(min[2], tester[2]),
                    );
                    max = Point3::new(
                        f64::max(max[0], tester[0]),
                        f64::max(max[1], tester[1]),
                        f64::max(max[2], tester[2]),
                    );
                }
            }
        }

        RotateY {
            ptr: p,
            sin_theta,
            cos_theta,
            has_box,
            bbox: Aabb::new(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let origin = Point3::new(
            self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2],
            r.origin[1],
            self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2],
        );

        let direction = Vec3::new(
            self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2],
            r.direction()[1],
            self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2],
        );

        let rotated_r = Ray::new(origin, direction, r.time());

        if !self.ptr.hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }

        let p = Point3::new(
            self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2],
            rec.p[1],
            -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2],
        );

        let normal = Vec3::new(
            self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2],
            rec.normal[1],
            -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2],
        );

        rec.p = p;
        rec.set_face_normal(&rotated_r, &normal);

        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox;

        self.has_box
    }
}