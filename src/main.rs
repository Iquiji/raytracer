extern crate image as image_crate;
mod hitable;
mod hitable_list;
mod ray;
mod sphere;
mod vec3;
use crate::hitable::Hitable;
use hitable::{hitableEnum, HitRecord};
use hitable_list::HitableList;
use piston_window::{clear, image, PistonWindow, Texture, TextureSettings, WindowSettings};
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;
const W: usize = 320;
const H: usize = 240;
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
    image_crate::save_buffer("buf.png", &buf, W as u32 , H as u32 ,image_crate::ColorType::RGBA(8));
/*    let texture = Texture::from_image(&mut window.factory, &img, &TextureSettings::new()).unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0, 0.0, 0.5, 1.0], graphics);
            image(&texture, context.transform, graphics)
        });
    }*/
}
fn render(img: &mut [u8]) {
    let lower_left_corner = Vec3::new(-2.0, -1.5, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 3.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let mut world: HitableList = HitableList {
        hitable: vec![
            hitableEnum::SphereE(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
            hitableEnum::SphereE(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
        ],
    };
    for x in 0..W {
        if x%W == 0{
            println!("{}",x);
        }
        for y in 0..H {
            let u = (x as f64 / W as f64);
            let v = (y as f64 / H as f64);
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let col = color(&r, &world);
            img[(x + y * W) * 4 + 0] = (255.0 * col.r()) as u8;
            img[(x + y * W) * 4 + 1] = (255.0 * col.g()) as u8;
            img[(x + y * W) * 4 + 2] = (255.0 * col.b()) as u8;
        }
    }
}
fn color(r: &Ray, world: &HitableList) -> Vec3 {
    let rec: HitRecord = world.hit(&r, 0.0, std::f64::MAX).unwrap();
    match rec {
        HitRecord => {
            return Vec3::new(
                rec.normal.x() + 1.0,
                rec.normal.y() + 1.0,
                rec.normal.z() + 1.0,
            ) * 0.5;
        }
        _ => {}
    }
    let unit_direction: Vec3 = Vec3::unit_vector(&r.direction());
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
}
