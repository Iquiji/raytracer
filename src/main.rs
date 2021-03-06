#![warn(clippy::all)]
extern crate image as image_crate;
use rand::prelude::*;
mod graphics;
mod math;
mod object;
use graphics::camera::Camera;
use math::ray::Ray;
use math::vec3::Vec3;
use object::hitable::{Hitable, HitableEnum};
use object::hitable_list::HitableList;
use object::material::{Dielectric, Lambertian, Material, MaterialEnum, Metal};
use object::sphere::Sphere;
use piston_window::{
    clear, image, Event, EventLoop, Loop, PistonWindow, Texture, TextureSettings, Transformed,
    WindowSettings,Window,
};
use rayon::prelude::*;
const W: usize = 600;
const H: usize = 600;
const NS: usize = 10;
const MAX_DEPTH: u32 = 5;
fn main() {
    let mut world: HitableList = HitableList {
        hitable: vec![
            HitableEnum::SphereE(Sphere::new(
                Vec3::new(0.0, 0.0, 0.0),
                0.5,
                MaterialEnum::Lambertian(Lambertian::new(0.8, 0.3, 0.3)),
            )),
            HitableEnum::SphereE(Sphere::new(
                Vec3::new(0.0, 100.5, 0.0),
                100.0,
                MaterialEnum::Lambertian(Lambertian::new(0.8, 0.8, 0.0)),
            )),
            HitableEnum::SphereE(Sphere::new(
                Vec3::new(1.0, 0.0, 0.0),
                0.5,
                MaterialEnum::Metal(Metal::new(0.8, 0.6, 0.2, 1.0)),
            )),
            HitableEnum::SphereE(Sphere::new(
                Vec3::new(-1.0, 0.0, 0.0),
                0.5,
                MaterialEnum::Dielectric(Dielectric::new(1.5)),
            )),
            HitableEnum::SphereE(Sphere::new(
                Vec3::new(-1.0, 0.0, 0.0),
                -0.45,
                MaterialEnum::Dielectric(Dielectric::new(1.5)),
            )),
        ],
    };
    let mut cam = Camera::new(
        Vec3::new(-2.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        (W as f64) / (H as f64),
    );
    let mut window: PistonWindow = WindowSettings::new("Raytrace?", [W as u32, H as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();
    /*
        image_crate::save_buffer(
            "buf.png",
            &buf,
            W as u32,
            H as u32,
            image_crate::ColorType::RGBA(8),
        )
        .expect("could not save img");
    */

    let mut forward: bool = true;
    let mut angle: f64 = 0.0;
    window.set_ups(60);
    window.set_ups_reset(1);
    let mut tctx = window.create_texture_context();
    let mut changed = true;
    let mut texture = None;
    let mut font = window
        .load_font("FantasqueSansMono-Regular.ttf")
        .expect("font not found");
    //font.preload_printable_ascii(12).expect("couldnt load printable ascii");
    let mut info_text: String = "".to_owned();
    while let Some(event) = window.next() {
        if let Event::Loop(Loop::Update(_args)) = event {
            changed = true;
        }
        if changed {
            let start = std::time::Instant::now();
            let window_size = window.size();
            let mut buf: Vec<u8> = vec![255; (window_size.width * window_size.height * 4.0) as usize];
            animate(&mut world, &mut forward, &mut cam, &mut angle,window_size);
            render(&mut buf, &world, &cam,window_size);
            let img = image_crate::ImageBuffer::from_vec(window_size.width as u32,window_size.height as u32, buf).unwrap();
            texture = Texture::from_image(&mut tctx, &img, &TextureSettings::new()).ok();
            info_text = format!(
                "Rendering {}x{}@{} pixel took {:?}ms",
                window_size.width,
                window_size.height,
                NS,
                start.elapsed().as_millis()
            );
            changed = false;
        }
        window.draw_2d(&event, |context, graphics, device| {
            clear([1.0, 0.0, 0.5, 1.0], graphics);
            image(
                texture.as_ref().expect("rendered texture"),
                context.transform.scale(1.0,1.0),
                graphics,
            );
            piston_window::text::Text::new_color([0.0, 0.0, 0.0, 1.0], 10)
                .draw(
                    &info_text,
                    &mut font,
                    &piston_window::draw_state::DrawState::default(),
                    context.transform.trans(12.0, 18.0),
                    graphics,
                )
                .expect("draw info text");
            font.factory.encoder.flush(device);
        });
    }
}
fn animate(world: &mut HitableList, forward: &mut bool, cam: &mut Camera, angle: &mut f64,window_size: piston_window::Size) {
    let buf = &mut world.hitable[4];
    match buf {
        HitableEnum::SphereE(ref mut sph) => {
            if sph.center.x() > 1.0 {
                *forward = false;
            } else if sph.center.x() < -1.0 {
                *forward = true;
            }
            if *forward {
                sph.center.e[0] += 0.01;
            } else {
                sph.center.e[0] -= 0.01;
            }
        }
    }
    let buf = &mut world.hitable[3];
    match buf {
        HitableEnum::SphereE(ref mut sph) => {
            if sph.center.x() > 1.0 {
                *forward = false;
            } else if sph.center.x() < -1.0 {
                *forward = true;
            }
            if *forward {
                sph.center.e[0] += 0.01;
            } else {
                sph.center.e[0] -= 0.01;
            }
        }
    }
    let rad: f64 = *angle * std::f64::consts::PI / 180.0;
    *cam = cam.mv(-cam.lookfrom + Vec3::new(rad.cos() * 2.0, -2.0, rad.sin() * 2.0), (window_size.width as f64)/(window_size.height as f64));
    *angle += 1.0;
}
fn render(img: &mut [u8], world: &HitableList, cam: &Camera,window_size: piston_window::Size) {
    img.par_chunks_mut((window_size.width * 4.0) as usize)
        .enumerate()
        .for_each(|(y, chunk)| {
            for x in 0..window_size.width as usize {
                let mut col = Vec3::new(0.0, 0.0, 0.0);
                let mut rng = rand::thread_rng();
                for _ in 0..NS {
                    let u = (x as f64 + rng.gen::<f64>()) / window_size.width as f64;
                    let v = (y as f64 + rng.gen::<f64>()) / window_size.height as f64;
                    let r: Ray = cam.get_ray(u, v);
                    col += color(&r, &world, 0);
                }
                col /= NS as f64;
                col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());
                chunk[x * 4] = (255.0 * col.r()) as u8;
                chunk[x * 4 + 1] = (255.0 * col.g()) as u8;
                chunk[x * 4 + 2] = (255.0 * col.b()) as u8;
            }
        })
}
fn color(r: &Ray, world: &HitableList, depth: u32) -> Vec3 {
    let rec = world.hit(&r, 0.001, std::f64::MAX);
    if let Some(hit_record) = rec {
        let mut attunation: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let scattered = hit_record.material.scatter(r, &hit_record, &mut attunation);
        match scattered {
            Some(ref r2) if depth < MAX_DEPTH => {
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
