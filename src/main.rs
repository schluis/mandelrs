use image::{Rgb, RgbImage};
use num::{complex::ComplexFloat, Complex};
use std::time::Instant;

const MAX_RECURSIONS: u32 = 100;
const WIDTH_PIXEL: u32 = 7500;
const HEIGHT_PIXEL: u32 = 5000;

fn mandelbrot(x: f32, y: f32) -> Rgb<u8> {
    let c = Complex::new(x, y);
    let mut z = Complex::new(0.0, 0.0);

    for _recursion_depth in 1..MAX_RECURSIONS {
        z = z * z + c;

        if z.abs() > 2.0 {
            return [0, 0, 0].into();
        }
    }
    [255, 255, 255].into()
}

fn main() {
    let start = Instant::now();
    let mut image = RgbImage::new(WIDTH_PIXEL, HEIGHT_PIXEL);

    for (x_image, x_math) in (0..WIDTH_PIXEL).map(|x| {
        (
            x,
            (x as f32 - 2.0 / 3.0 * WIDTH_PIXEL as f32) * (3.0 / WIDTH_PIXEL as f32),
        )
    }) {
        for (y_image, y_math) in (0..HEIGHT_PIXEL).map(|y| {
            (
                y,
                (y as f32 - HEIGHT_PIXEL as f32 / 2.0) * (2.0 / HEIGHT_PIXEL as f32),
            )
        }) {
            image.put_pixel(x_image, y_image, mandelbrot(x_math, y_math));
        }
    }

    image.save("mandelbrot_minimal.png").unwrap();

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
