use std::cmp::Ordering;
use std::rc::Rc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::point3::Point3;
use crate::random::random_usize_in_range;
use crate::ray::Ray;

pub struct BvhNode {
    pub left: Rc<dyn Hittable>,
    pub right: Rc<dyn Hittable>,
    pub bounding_box: Aabb,
}

impl BvhNode {
    pub fn new(
        objects: &mut Vec<Rc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> BvhNode {
        let axis = random_usize_in_range(0, 3);

        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
            _ => panic!("Undefined axis: {}", axis)
        };

        let object_span = end - start;
        let (left, right) = if object_span == 1 {
            (objects[start].clone(), objects[start].clone())
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                (objects[start].clone(), objects[start + 1].clone())
            } else {
                (objects[start + 1].clone(), objects[start].clone())
            }
        } else {
            objects[start..end].sort_by(comparator);

            let mid = start + object_span / 2;
            let left: Rc<dyn Hittable> = Rc::new(BvhNode::new(objects, start, mid, time0, time1));
            let right: Rc<dyn Hittable> = Rc::new(BvhNode::new(objects, mid, end, time0, time1));
            (left, right)
        };

        let mut box_left = Aabb::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0));
        let mut box_right = Aabb::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0));

        if !left.bounding_box(time0, time1, &mut box_left) || !right.bounding_box(time0, time1, &mut box_right) {
            eprintln!("No bounding box in bvh_node constructor.")
        }

        BvhNode {
            left,
            right,
            bounding_box: Aabb::surrounding_box(&box_left, &box_right),
        }
    }

    pub fn new_from_list(objects: &mut Vec<Rc<dyn Hittable>>) -> Self {
        BvhNode::new(objects, 0, objects.len(), 0.0, 1.0)
    }
}


fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: usize) -> std::cmp::Ordering {
    let mut box_a = Aabb::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0));
    let mut box_b = Aabb::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0));

    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        eprintln!("No bounding box in bvh_node constructor.")
    }

    box_a.min()[axis].partial_cmp(&box_b.min()[axis]).unwrap()
}

fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 2)
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.bounding_box.hit(r, t_min, t_max) {
            return false;
        }

        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let t_max = if hit_left {
            rec.t
        } else {
            t_max
        };

        let hit_right = self.right.hit(r, t_min, t_max, rec);

        hit_left || hit_right
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bounding_box;
        true
    }
}