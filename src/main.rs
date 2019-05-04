extern crate image as image_crate;
mod vec3;
use piston_window::{clear, image, PistonWindow, Texture, TextureSettings, WindowSettings};
use vec3::Vec3;
const W: usize = 640;
const H: usize = 480;
fn main() {
    let mut window: PistonWindow = WindowSettings::new("Raytrace?", [W as u32, H as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut buf: Vec<u8> = vec![255; (W * H * 4) as usize];
    render(&mut buf);
    let img = image_crate::ImageBuffer::from_vec(W as u32, H as u32, buf).unwrap();

    let texture = Texture::from_image(&mut window.factory, &img, &TextureSettings::new()).unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0, 0.0, 0.5, 1.0], graphics);
            image(&texture, context.transform, graphics)
        });
    }
}
fn render(img: &mut [u8]) {
    for x in 0..W {
        for y in 0..H {
            img[(x + y * W) * 4] = (x as f64 / W as f64 * 255.0) as u8;
            img[(x + y * W) * 4 + 1] = (y as f64 / H as f64 * 255.0) as u8;
            img[(x + y * W) * 4 + 2] = 51 as u8;
        }
    }
}
