extern crate nalgebra as na;
use na::{Vector3, Rotation3};

fn main() {
    const NX: u32 = 200;
    const NY: u32 = 100;

    println!("P3\n{} {} \n255\n", NX, NY);
    for j in 0..(NY - 1) {
        for i in 0..NX {
            let r: f32 = (i as f32)/ (NX as f32);
            let g: f32 = (j as f32) / (NY as f32);
            let b: f32 = 0.2;
            let col = Vector3::new(1, 2, 3);

            let ir: i32 = (255.99 * r) as i32;
            let ig: i32 = (255.99 * g) as i32;
            let ib: i32 = (255.99 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        } 
    }
}
