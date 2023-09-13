use colors_transform::{Color, Hsl};
use image::RgbImage;
use num::{complex::ComplexFloat, Complex};
use std::time::Instant;

const MAX_RECURSIONS: u32 = 360 * 4;
const WIDTH_PIXEL: u32 = 1500;
const HEIGHT_PIXEL: u32 = 1000;

fn mandelbrot(x: f32, y: f32) -> u32 {
    let c = Complex::new(x, y);
    let mut z = Complex::new(0.0, 0.0);

    for recursion_depth in 1..MAX_RECURSIONS {
        z = z * z + c;

        if z.abs() > 2.0 {
            return recursion_depth;
        }
    }
    return MAX_RECURSIONS;
}

fn main() {
    let start = Instant::now();
    let mut image = RgbImage::new(WIDTH_PIXEL, HEIGHT_PIXEL);

    for x_image in 0..WIDTH_PIXEL {
        for y_image in 0..HEIGHT_PIXEL {
            let x_math =
                (x_image as f32 - 2.0 / 3.0 * WIDTH_PIXEL as f32) * (3.0 / WIDTH_PIXEL as f32);
            let y_math = (y_image as f32 - HEIGHT_PIXEL as f32 / 2.0) * (2.0 / HEIGHT_PIXEL as f32);

            let recursions = mandelbrot(x_math, y_math);
            let rgb = Hsl::from(recursions as f32 % 360.0, 100.0, 50.0);

            image.put_pixel(
                x_image,
                y_image,
                [
                    rgb.get_red() as u8,
                    rgb.get_green() as u8,
                    rgb.get_blue() as u8,
                ]
                .into(),
            );
        }
    }

    image.save("mandelbrot_minimal.png").unwrap();

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
