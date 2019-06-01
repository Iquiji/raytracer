#![warn(clippy::all)]
extern crate image as image_crate;
use rand::prelude::*;
mod camera;
mod hitable;
mod hitable_list;
mod material;
mod ray;
mod sphere;
mod vec3;
use crate::hitable::Hitable;
use camera::Camera;
use hitable::{HitableEnum};
use hitable_list::HitableList;
use material::{Lambertian, Material, MaterialEnum, Metal};
//use piston_window::{clear, image, PistonWindow, Texture, TextureSettings, WindowSettings};
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;
const W: usize = 640;
const H: usize = 480;
const NS: usize = 100;
fn main() {
    /*
    let mut window: PistonWindow = WindowSettings::new("Raytrace?", [W as u32, H as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();
    */
    let mut buf: Vec<u8> = vec![255; (W * H * 4) as usize];
    render(&mut buf);

    //let img = image_crate::ImageBuffer::from_vec(W as u32, H as u32, buf).unwrap();
    image_crate::save_buffer(
        "buf.png",
        &buf,
        W as u32,
        H as u32,
        image_crate::ColorType::RGBA(8),
    ).expect("could not save img");
    /*    let texture = Texture::from_image(&mut window.factory, &img, &TextureSettings::new()).unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0, 0.0, 0.5, 1.0], graphics);
            image(&texture, context.transform, graphics)
        });
    }*/
}
fn render(img: &mut [u8]) {
    let world: HitableList = HitableList {
        hitable: vec![
            HitableEnum::SphereE(Sphere::new(
                Vec3::new(0.0, 0.0, -1.0),
                0.5,
                MaterialEnum::Lambertian(Lambertian::new(0.8, 0.3, 0.3)),
            )),
            HitableEnum::SphereE(Sphere::new(
                Vec3::new(0.0, 100.5, -1.0),
                100.0,
                MaterialEnum::Lambertian(Lambertian::new(0.8, 0.8, 0.0)),
            )),
            HitableEnum::SphereE(Sphere::new(
                Vec3::new(1.0, 0.0, -1.0),
                0.5,
                MaterialEnum::Metal(Metal::new(0.8, 0.6, 0.2,1.0)),
            )),
            HitableEnum::SphereE(Sphere::new(
                Vec3::new(-1.0, 0.0, -1.0),
                0.5,
                MaterialEnum::Metal(Metal::new(0.8, 0.8, 0.8,0.09)),
            )),
        ],
    };
    let cam = Camera::std();
    for x in 0..W {
        if x % (W / 10) == 0 {
            println!("{}%", (x as f64 / W as f64) * 100.0);
        }
        for y in 0..H {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            let mut rng = rand::thread_rng();
            for _ in 0..NS {
                let u = (x as f64 + rng.gen::<f64>()) / W as f64;
                let v = (y as f64 + rng.gen::<f64>()) / H as f64;
                let r: Ray = cam.get_ray(u, v);
                col += color(&r, &world, 0);
            }
            col /= NS as f64;
            col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());
            img[(x + y * W) * 4] = (255.0 * col.r()) as u8;
            img[(x + y * W) * 4 + 1] = (255.0 * col.g()) as u8;
            img[(x + y * W) * 4 + 2] = (255.0 * col.b()) as u8;
        }
    }
}
fn color(r: &Ray, world: &HitableList, depth: u32) -> Vec3 {
    let rec = world.hit(&r, 0.001, std::f64::MAX);
    if let Some(hit_record) = rec {
        let mut attunation: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let scattered = hit_record.material.scatter(r, &hit_record, &mut attunation);
        match scattered {
            Some(ref r2) if depth < 50 => {
                return attunation * color(&r2, &world, depth + 1);
            }
            _ => {
                return Vec3::new(0.0, 0.0, 0.0);
            }
        }
    }
    let unit_direction: Vec3 = Vec3::unit_vector(&r.direction());
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}
