extern crate image;
extern crate num;
extern crate num_complex;

pub mod newtone_fractal {
    use image;
    use image::png::PNGEncoder;
    use image::{ColorType, GrayImage, ImageBuffer, Luma, Pixel};
    use num_complex::Complex;
    use std::clone::Clone;
    use std::f64;
    use std::fs::File;
    use std::io::{Error, Write};
    use std::ops::{Add, Div, Mul, Sub};
    use std::str::FromStr;

    pub fn abs(z: Complex<f64>) -> f64 {
        ((z.re * z.re) + (z.im * z.im)).sqrt()
    }

    pub fn choose_color(x: i32, y: i32, n: i32) -> [u8; 3] {
        [0, 0, 0]
    }

    // F(x) = x^numRoots - 1
    pub fn zfunc(z: Complex<f64>) -> Complex<f64> {
        (z * z * z).sub(1 as f64)
    }

    // dF(x) = numRoots*x^(numRoots -1)
    pub fn dfunc(z: Complex<f64>) -> Complex<f64> {
        (z * z).scale(3 as f64)
    }

    pub fn arg(z: Complex<f64>) -> f64 {
        z.im.atan2(z.re)
    }

    #[allow(dead_code)]
    pub fn draw(mx_input: i32, my_input: i32, iter: u32, x_0: f64, x_n: f64, y_0: f64, y_n: f64) {
        let tolerance = 0.00001; // Work until the epsilon squared < this.

        let r1 = Complex { re: 1.0, im: 0.0 };
        let r2 = Complex {
            re: -0.5,
            im: 3.0_f64.sqrt() / 2.0,
        };
        let r3 = Complex {
            re: -0.5,
            im: -3.0_f64.sqrt() / 2.0,
        };

        let mut imgbuf = image::RgbImage::new(mx_input as u32, my_input as u32);

        let mx = mx_input / 2;
        let my = my_input / 2;

        for y in -my..my {
            for x in -mx..mx {
                let mut n = 0;

                // zx = scaled x coordinate of pixel (scaled to lie in the Mandelbrot X scale (-2.5, 1))
                //     zy = scaled y coordinate of pixel (scaled to lie in the Mandelbrot Y scale (-1, 1))
                //     float2 z = float2(zx, zy); //Z is originally set to the pixel coordinates

                let mut zxy = Complex {
                    re: x as f64 * 4.0 / (my_input - 2) as f64,
                    im: -(y as f64 * 4.0 / (mx_input + 2) as f64),
                };

                while n < iter {
                    zxy = zxy.sub(zfunc(zxy) / dfunc(zxy));
                    n = n + 1;
                }

                if abs(zxy - r1) < tolerance {
                    imgbuf.put_pixel(i_to_u(x, mx), i_to_u(y, my), image::Rgb([255, 0, 0]));
                }

                if abs(zxy - r2) <= tolerance {
                    imgbuf.put_pixel(i_to_u(x, mx), i_to_u(y, my), image::Rgb([0, 255, 0]));
                }

                if abs(zxy - r3) <= tolerance {
                    imgbuf.put_pixel(i_to_u(x, mx), i_to_u(y, my), image::Rgb([0, 0, 255]));
                }
            }
        }

        imgbuf.save("fractal.png").unwrap();
    }

    fn i_to_u(point: i32, canvas: i32) -> u32 {
        (point + canvas) as u32
    }
}
