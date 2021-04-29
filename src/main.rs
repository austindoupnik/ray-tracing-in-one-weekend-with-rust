use std::borrow::BorrowMut;
use std::io;
use std::path::Path;
use std::rc::Rc;

use crate::aarect::{XyRect, XzRect, YzRect};
use crate::block::Block;
use crate::bvh_node::{BvhNode};
use crate::camera::Camera;
use crate::color::{Color, write_color};
use crate::hittable::{HitRecord, Hittable, RotateY, Translate};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::moving_sphere::MovingSphere;
use crate::point3::Point3;
use crate::random::{random, random_in_range};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor};
use crate::vec3::Vec3;
use crate::constant_medium::ConstantMedium;

mod vec3;
mod color;
mod ray;
mod point3;
mod hittable;
mod sphere;
mod hittable_list;
mod random;
mod camera;
mod material;
mod moving_sphere;
mod aabb;
mod bvh_node;
mod texture;
mod perlin;
mod aarect;
mod block;
mod constant_medium;

fn ray_color(r: &Ray, background: &Color, world: &dyn Hittable, depth: u32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if !world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        return *background;
    }

    let mut scattered = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0);
    let mut attenuation = Color::new(0.0, 0.0, 0.0);
    let emitted = rec.mat_ptr.as_ref().unwrap().emitted(rec.u, rec.v, &rec.p);

    if !rec.mat_ptr.as_ref().unwrap().scatter(r, &rec, &mut attenuation, &mut scattered) {
        return emitted;
    }

    emitted + attenuation * ray_color(&scattered, background, world, depth - 1)
}

fn random_scene() -> HittableList {
    let mut objects: Vec<Rc<dyn Hittable>> = Vec::new();

    let checker = Rc::new(CheckerTexture::new(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
    objects.push(Rc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Rc::new(Lambertian::new_from_texture(checker)))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Point3::new(a as f64 + 0.9 * random(), 0.2, b as f64 + 0.9 * random());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    let center1 = center + Vec3::new(0.0, random_in_range(0.0, 0.5), 0.0);
                    objects.push(Rc::new(MovingSphere::new(center, center1, 0.0, 1.0, 0.2, sphere_material.clone())));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_in_range(0.5, 1.0);
                    let fuzz = random_in_range(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    objects.push(Rc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else {
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    objects.push(Rc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    objects.push(Rc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Rc::new(material1))));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    objects.push(Rc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Rc::new(material2))));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    objects.push(Rc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Rc::new(material3))));

    let bvh_node = BvhNode::new_from_list(&mut objects);

    let mut world = HittableList::new();
    world.add(Rc::new(bvh_node));

    world
}

fn two_spheres() -> HittableList {
    let checker = Rc::new(CheckerTexture::new(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -10.0, 0.0), 10.0, Rc::new(Lambertian::new_from_texture(checker.clone())))));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, Rc::new(Lambertian::new_from_texture(checker.clone())))));

    world
}

fn two_perlin_spheres() -> HittableList {
    let pertext = Rc::new(NoiseTexture::new(4.0));

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Rc::new(Lambertian::new_from_texture(pertext.clone())))));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, Rc::new(Lambertian::new_from_texture(pertext.clone())))));

    world
}

fn earth() -> HittableList {
    let earth_texture = Rc::new(ImageTexture::new(Path::new("earthmap.jpg")));
    let earth_surface = Rc::new(Lambertian::new_from_texture(earth_texture));
    let globe = Rc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    let mut world = HittableList::new();
    world.add(globe);

    world
}

fn simple_light() -> HittableList {
    let pertext = Rc::new(NoiseTexture::new(4.0));

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Rc::new(Lambertian::new_from_texture(pertext.clone())))));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, Rc::new(Lambertian::new_from_texture(pertext.clone())))));

    world.add(Rc::new(XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, Rc::new(DiffuseLight::new(Rc::new(SolidColor::new(Color::new(4.0, 4.0, 4.0))))))));

    world
}

fn cornell_box() -> HittableList {
    let red = Rc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));

    let light = Rc::new(DiffuseLight::new(Rc::new(SolidColor::new(Color::new(15.0, 15.0, 15.0)))));

    let mut world = HittableList::new();

    world.add(Rc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.add(Rc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));

    world.add(Rc::new(XzRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light)));

    world.add(Rc::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
    world.add(Rc::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    world.add(Rc::new(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));

    let box1 = Rc::new(Block::new(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 330.0, 165.0), white.clone()));
    let box1 = Rc::new(RotateY::new(box1.clone(), 15.0));
    let box1 = Rc::new(Translate::new(box1.clone(), Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let box2 = Rc::new(Block::new(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 165.0, 165.0), white.clone()));
    let box2 = Rc::new(RotateY::new(box2.clone(), -18.0));
    let box2 = Rc::new(Translate::new(box2.clone(), Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    world
}

fn cornell_smoke() -> HittableList {
    let red = Rc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));

    let light = Rc::new(DiffuseLight::new(Rc::new(SolidColor::new(Color::new(7.0, 7.0, 7.0)))));

    let mut world = HittableList::new();

    world.add(Rc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.add(Rc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));

    world.add(Rc::new(XzRect::new(113.0, 443.0, 127.0, 432.0, 554.0, light)));

    world.add(Rc::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    world.add(Rc::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
    world.add(Rc::new(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));

    let box1 = Rc::new(Block::new(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 330.0, 165.0), white.clone()));
    let box1 = Rc::new(RotateY::new(box1.clone(), 15.0));
    let box1 = Rc::new(Translate::new(box1.clone(), Vec3::new(265.0, 0.0, 295.0)));
    world.add(Rc::new(ConstantMedium::new(box1, 0.01, Rc::new(SolidColor::new(Color::new(0.0, 0.0, 0.0))))));

    let box2 = Rc::new(Block::new(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 165.0, 165.0), white.clone()));
    let box2 = Rc::new(RotateY::new(box2.clone(), -18.0));
    let box2 = Rc::new(Translate::new(box2.clone(), Vec3::new(130.0, 0.0, 65.0)));
    world.add(Rc::new(ConstantMedium::new(box2, 0.01, Rc::new(SolidColor::new(Color::new(1.0, 1.0, 1.0))))));

    world
}

fn final_scene() -> HittableList {
    let mut boxes1: Vec<Rc<dyn Hittable>> = Vec::new();
    let ground = Rc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + (i as f64) * w;
            let z0 = -1000.0 + (j as f64) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_in_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.push(Rc::new(Block::new(Point3::new(x0, y0, z0), Point3::new(x1, y1, z1), ground.clone())))
        }
    }

    let mut objects = HittableList::new();
    objects.add(Rc::new(BvhNode::new_from_list(&mut boxes1)));

    let light = Rc::new(DiffuseLight::new(Rc::new(SolidColor::new(Color::new(7.0, 7.0, 7.0)))));
    objects.add(Rc::new(XzRect::new(123.0, 423.0, 147.0, 412.0, 554.0, light)));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    objects.add(Rc::new(MovingSphere::new(center1, center2, 0.0, 1.0, 50.0, moving_sphere_material)));

    objects.add(Rc::new(Sphere::new(Point3::new(260.0, 150.0, 45.0), 50.0, Rc::new(Dielectric::new(1.5)))));
    objects.add(Rc::new(Sphere::new(Point3::new(0.0, 150.0, 145.0), 50.0, Rc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)))));

    let boundary = Rc::new(Sphere::new(Point3::new(360.0, 150.0, 145.0), 70.0, Rc::new(Dielectric::new(1.5))));
    objects.add(boundary.clone());
    objects.add(Rc::new(ConstantMedium::new(boundary.clone(), 0.2, Rc::new(SolidColor::new(Color::new(0.2, 0.4, 0.9))))));

    let boundary = Rc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 5000.0, Rc::new(Dielectric::new(1.5))));
    objects.add(Rc::new(ConstantMedium::new(boundary.clone(), 0.0001, Rc::new(SolidColor::new(Color::new(1.0, 1.0, 1.0))))));

    let emat = Rc::new(Lambertian::new_from_texture(Rc::new(ImageTexture::new(Path::new("earthmap.jpg")))));
    objects.add(Rc::new(Sphere::new(Point3::new(400.0, 200.0, 400.0), 100.0, emat)));

    let pertext = Rc::new(NoiseTexture::new(0.1));
    objects.add(Rc::new(Sphere::new(Point3::new(220.0, 280.0, 300.0), 80.0, Rc::new(Lambertian::new_from_texture(pertext)))));

    let mut boxes2: Vec<Rc<dyn Hittable>> = Vec::new();
    let white = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.push(Rc::new(Sphere::new(Point3::random_in_range(0.0, 165.0), 10.0, white.clone())));
    }

    objects.add(Rc::new(Translate::new(Rc::new(RotateY::new(Rc::new(BvhNode::new_from_list(&mut boxes2)), 15.0)), Vec3::new(-100.0, 270.0, 395.0))));

    objects
}

fn main() {
    let aspect_ratio;

    let image_width;
    let samples_per_pixel;
    let max_depth = 50;

    let world;
    let lookfrom;
    let lookat;
    let vfov;
    let aperture;
    let background;

    match 0 {
        1 => {
            world = random_scene();
            aspect_ratio = 16.0 / 9.0;
            image_width = 400;
            samples_per_pixel = 100;
            background = Color::new(0.70, 0.80, 1.00);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            world = two_spheres();
            aspect_ratio = 16.0 / 9.0;
            image_width = 400;
            samples_per_pixel = 100;
            background = Color::new(0.70, 0.80, 1.00);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.0;
        }
        3 => {
            world = two_perlin_spheres();
            aspect_ratio = 16.0 / 9.0;
            image_width = 400;
            samples_per_pixel = 100;
            background = Color::new(0.70, 0.80, 1.00);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.0;
        }
        4 => {
            world = earth();
            aspect_ratio = 16.0 / 9.0;
            image_width = 400;
            samples_per_pixel = 100;
            background = Color::new(0.70, 0.80, 1.00);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.0;
        }
        5 => {
            world = simple_light();
            aspect_ratio = 16.0 / 9.0;
            image_width = 400;
            samples_per_pixel = 400;
            background = Color::new(0.0, 0.0, 0.0);
            lookfrom = Point3::new(26.0, 3.0, 6.0);
            lookat = Point3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }
        6 => {
            world = cornell_box();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            background = Color::new(0.0, 0.0, 0.0);
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
            aperture = 0.0;
        }
        7 => {
            world = cornell_smoke();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            background = Color::new(0.0, 0.0, 0.0);
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
            aperture = 0.0;
        }
        8 | _ => {
            world = final_scene();
            aspect_ratio = 1.0;
            image_width = 800;
            samples_per_pixel = 10000;
            background = Color::new(0.0, 0.0, 0.0);
            lookfrom = Point3::new(478.0, 278.0, -600.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
            aperture = 0.0;
        }
    }

    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let dist_to_focus = 10.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let cam = Camera::new(lookfrom, lookat, vup, vfov, aspect_ratio, aperture, dist_to_focus, 0.0, 1.0);

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random::random()) / (image_width - 1) as f64;
                let v = (j as f64 + random::random()) as f64 / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &background, &world, max_depth);
            }
            write_color(io::stdout().borrow_mut(), pixel_color, samples_per_pixel).unwrap();
        }
    }

    eprint!("\nDone.\n");
}
