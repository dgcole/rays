extern crate nalgebra as na;
use na::{Vector3, Rotation3};

struct Ray {
    a: Vector3<f32>,
    b: Vector3<f32>
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

fn hit_sphere(center: &Vector3<f32>, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin() - center;

    let a: f32 = na::dot(r.direction(), r.direction());
    let b: f32 = 2.0 * na::dot(&oc, r.direction());
    let c: f32 = na::dot(&oc, &oc) - radius * radius;

    let discriminant: f32 = b * b - 4.0 * a * c;

    if (discriminant < 0.0) {
        return -1.0
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}

fn color(r: Ray) -> Vector3<f32> {
    let mut t: f32 = hit_sphere(&Vector3::new(0.0, 0.0, -1.0), 0.5, &r);
    if (t > 0.0) {
        let n = na::normalize(&(r.point_at_parameter(t) - Vector3::new(0.0, 0.0, -1.0)));
        return 0.5 * Vector3::new(n[0] + 1.0, n[1] + 1.0, n[2] + 1.0);
    }

    let unit_direction = na::normalize(r.direction());

    t = 0.5 * (unit_direction[1] + 1.0);

    return (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
}

fn main() {
    const NX: u32 = 200;
    const NY: u32 = 100;

    println!("P3\n{} {} \n255\n", NX, NY);

    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);
    let origin = Vector3::new(0.0, 0.0, 0.0);

    for j in 0..(NY - 1) {
        let rj = (NY - 1) - j;
        for i in 0..NX {
            let u: f32 = (i as f32) / (NX as f32);
            let v: f32 = (rj as f32) / (NY as f32);

            let r: Ray = Ray {a: origin, b: (lower_left_corner + u * horizontal + v * vertical)};

            let col = color(r);

            let ir: i32 = (255.99 * col[0]) as i32;
            let ig: i32 = (255.99 * col[1]) as i32;
            let ib: i32 = (255.99 * col[2]) as i32;

            println!("{} {} {}", ir, ig, ib);
        } 
    }
}
