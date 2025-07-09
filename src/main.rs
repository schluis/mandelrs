use colors_transform::{Color, Hsl};
use image::RgbImage;
use num::{Complex, complex::ComplexFloat};
use rayon::prelude::*;
use std::time::Instant;

const MAX_RECURSIONS: u32 = 360 * 4;
const WIDTH_PIXEL: u32 = 3000;
const HEIGHT_PIXEL: u32 = 2323;
const ZOOM: f32 = 300000.0;
const ZOOM_X: f32 = 0.35989;
const ZOOM_Y: f32 = 0.64;

fn mandelbrot(x: f32, y: f32) -> u32 {
    let c = Complex::new(x, y);
    let mut z = Complex::new(0.0, 0.0);

    for recursion_depth in 1..MAX_RECURSIONS {
        z = z * z + c;

        if z.abs() > 2.0 {
            return recursion_depth;
        }
    }
    MAX_RECURSIONS
}

fn main() {
    let start = Instant::now();

    let pixels: Vec<[u8; 3]> = (0..HEIGHT_PIXEL)
        .into_par_iter()
        .flat_map(|y_image| {
            (0..WIDTH_PIXEL).into_par_iter().map(move |x_image| {
                let x_math = (x_image as f32 - 2.0 / 3.0 * WIDTH_PIXEL as f32)
                    * (3.1 / WIDTH_PIXEL as f32 / ZOOM)
                    + ZOOM_X;
                let y_math = (y_image as f32 - HEIGHT_PIXEL as f32 / 2.0)
                    * (2.4 / HEIGHT_PIXEL as f32 / ZOOM)
                    - ZOOM_Y;

                let recursions = mandelbrot(x_math, y_math);
                let rgb = Hsl::from(
                    (recursions as f32).powi(2) as f32 % 360.0,
                    100.0,
                    50.0 - (recursions as f32).log2() / (MAX_RECURSIONS as f32).log2() * 50.0,
                );

                [
                    rgb.get_red() as u8,
                    rgb.get_green() as u8,
                    rgb.get_blue() as u8,
                ]
            })
        })
        .collect();

    let mut image = RgbImage::new(WIDTH_PIXEL, HEIGHT_PIXEL);
    for (i, pixel) in pixels.iter().enumerate() {
        let x = (i as u32) % WIDTH_PIXEL;
        let y = (i as u32) / WIDTH_PIXEL;
        image.put_pixel(x, y, (*pixel).into());
    }

    image.save("mandelbrot_minimal.png").unwrap();

    let duration = start.elapsed();
    println!("Time elapsed: {duration:?}");
}
