extern crate nalgebra as na;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rand::prelude::*;
use na::{Vector3, Rotation3};

enum MaterialType {
    LAMBERTIAN,
    METAL
}

struct Material {
    t: MaterialType,
    albedo: Vector3<f32>
}

struct Ray {
    a: Vector3<f32>,
    b: Vector3<f32>
}

struct HitRecord {
    t: f32,
    p: Vector3<f32>,
    normal: Vector3<f32>,
    mat: Material
}

struct Sphere {
    center: Vector3<f32>,
    radius: f32,
    mat: Material
}

struct Camera {
    lower_left_corner: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
    origin: Vector3<f32>
}

impl Ray {
    fn origin(&self) -> &Vector3<f32> {
        return &self.a;
    }

    fn direction(&self) -> &Vector3<f32> {
        return &self.b;
    }

    fn point_at_parameter(&self, t: f32) -> Vector3<f32> {
        return self.a + self.b * t;
    }
}

impl Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;

        let a: f32 = na::dot(r.direction(), r.direction());
        let b: f32 = na::dot(&oc, r.direction());
        let c: f32 = na::dot(&oc, &oc) - (self.radius * self.radius);

        let discriminant: f32 = b * b - a * c;

        if (discriminant > 0.0) {
            let mut temp: f32 = (-b - discriminant.sqrt()) / a;

            if (temp < t_max && temp > t_min) {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
            temp = (-b + discriminant.sqrt()) / a;
            if (temp < t_max && temp > t_min) {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }
        return false;
    }
}

impl Camera {
    fn get_ray(&self, u: f32, v: f32) -> Ray {
        return Ray {a: self.origin, b: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin};
    }
}

fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
    return v - 2.0 * na::dot(v, n) * n;
}

impl Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> (bool, Vector3<f32>, Ray) {
        let cond: bool = false;
        let attenuation: Vector3<f32> = self.albedo;
        let mut scattered: Ray = Ray {a: rec.p, b: Vector3::new(0.0, 0.0, 0.0)};

        match self.t {
            MaterialType::LAMBERTIAN => {
                let target = rec.p + rec.normal + random_in_unit_sphere();
                scattered.b = target - rec.p;

                return (true, attenuation, scattered);
            },
            MaterialType::METAL => {
                let norm = na::normalize(r.direction());
                scattered.b = reflect(&norm, &rec.normal);

                return ((na::dot(scattered.direction(), &rec.normal) > 0.0), self.albedo, scattered);
            }
        }
        return (false, Vector3::new(0.0, 0.0, 0.0), Ray{a: Vector3::new(0.0, 0.0, 0.0), b: Vector3::new(0.0, 0.0, 0.0)});
    }
}

fn hit(r: &Ray, t_min: f32, t_max: f32, mut rec: &mut HitRecord, world: &Vec<Sphere>) -> bool {
    let mut temp_rec: HitRecord = HitRecord {
        t: 0.0,
        p: Vector3::new(0.0, 0.0, 0.0),
        normal: Vector3::new(0.0, 0.0, 0.0),
        mat: Material {
            t: MaterialType::LAMBERTIAN,
            albedo: Vector3::new(0.0, 0.0, 0.0)
        }
    };

    let mut hit_anything: bool = false;
    let mut closest_so_far: f64 = t_max as f64;

    for i in 0 .. world.len() {
        if (world[i].hit(&r, t_min, closest_so_far as f32, &mut temp_rec)) {
            hit_anything = true;
            closest_so_far = temp_rec.t as f64;

            rec.normal = temp_rec.normal;
            rec.t = temp_rec.t;
            rec.p = temp_rec.p;
        }
    }

    return hit_anything;
}

fn random_in_unit_sphere() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    let mut p = Vector3::new(0.0, 0.0, 0.0);

    while {
        let r0: f32 = rng.gen();
        let r1: f32 = rng.gen();
        let r2: f32 = rng.gen();

        p = 2.0 * Vector3::new(r0, r1, r2) - Vector3::new(1.0, 1.0, 1.0);

        na::magnitude_squared(&p) >= 1.0
    } {}

    return p;
}

fn color(r: Ray, world: &Vec<Sphere>) -> Vector3<f32> {
    let mut rec: HitRecord = HitRecord {
        t: 0.0,
        p: Vector3::new(0.0, 0.0, 0.0),
        normal: Vector3::new(0.0, 0.0, 0.0),
        mat: Material {
            t: MaterialType::LAMBERTIAN,
            albedo: Vector3::new(0.0, 0.0, 0.0)
        }
    };

    if hit(&r, 0.001, std::f32::MAX, &mut rec, world) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        return 0.5 * color(Ray{a: rec.p, b: target - rec.p}, world);
    } else {
        let unit_direction = na::normalize(r.direction());
        let t: f32 = 0.5 * (unit_direction[1] + 1.0);

        return (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
    }
}

fn main() {
    const NX: u32 = 320;
    const NY: u32 = 240;
    const NS: u32 = 100;

    let mut rng = rand::thread_rng();

    let path = Path::new("rays.ppm");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    file.write(format!("P3\n{} {} \n255\n", NX, NY).as_bytes());

    let cam = Camera {
        lower_left_corner: Vector3::new(-2.0, -1.0, -1.0),
        horizontal: Vector3::new(4.0, 0.0, 0.0),
        vertical: Vector3::new(0.0, 3.0, 0.0),
        origin: Vector3::new(0.0, 0.0, 0.0)
    };

    let s0 = Sphere {
        center: Vector3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        mat: Material {
            t: MaterialType::LAMBERTIAN,
            albedo: Vector3::new(0.0, 0.0, 0.0)
        }
    };

    let s1 = Sphere {
        center: Vector3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        mat: Material {
            t: MaterialType::LAMBERTIAN,
            albedo: Vector3::new(0.0, 0.0, 0.0)
        }
    };

    let mut world: Vec<Sphere> = Vec::new();
    world.push(s0);
    world.push(s1);

    for j in 0..(NY - 1) {
        let rj = (NY - 1) - j;
        for i in 0..NX {
            let mut col = Vector3::new(0.0, 0.0, 0.0);

            for k in 0 .. NS {
                let ur: f32 = rng.gen();
                let vr: f32 = rng.gen();
                let u: f32 = ((i as f32) + ur) / (NX as f32);
                let v: f32 = ((rj as f32) + vr) / (NY as f32);

                let r = cam.get_ray(u, v);

                let p = r.point_at_parameter(2.0);

                col += color(r, &world);
            }

            col /= (NS as f32);

            col = Vector3::new((col[0] as f32).sqrt(), (col[1] as f32).sqrt(), (col[2] as f32).sqrt());

            let ir: i32 = (255.99 * col[0]) as i32;
            let ig: i32 = (255.99 * col[1]) as i32;
            let ib: i32 = (255.99 * col[2]) as i32;

            file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes());
        } 
    }
}
