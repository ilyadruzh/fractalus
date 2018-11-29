extern crate image;
extern crate num;
extern crate wasm_bindgen;

pub mod newtone_fractal {
    use complex;
    use complex::complex::Complex;
    use complex::complex::{abs, div, mul, scale, sub, sub_f64};
    use image;
    use std::f64;
    use wasm_bindgen::prelude::*;
    use std::thread;
    use rayon::prelude::*;
    use rayon::iter::ParallelIterator;
    use std::fs::File;
    use logger;
    use std::sync::{Mutex, Arc};
    use std::rc::Rc;

    // #[wasm_bindgen]
    // pub fn choose_color(x: i32, y: i32, n: i32) -> [u8; 3] {
    //     [0, 0, 0]
    // }

    // F(x) = x^numRoots - 1
    // TODO: add pow
    pub fn zfunc(z: Complex) -> Complex {

        sub_f64(mul(z, mul(z, z)), 1.0)
    }

    // dF(x) = numRoots*x^(numRoots -1)
    // TODO: pow
    pub fn dfunc(z: Complex, pow_value: f64) -> Complex {

        scale(mul(z, z), pow_value as f64)
    }

    #[allow(dead_code)]
    pub fn draw(mx_input: i32, my_input: i32, iter: u32, z0: Complex, zn: Complex) {

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

        let mut imgbuf = Arc::new(  Mutex::new(image::RgbImage::new(mx_input as u32, my_input as u32)));

        let mx = mx_input / 2;
        let my = my_input / 2;

        info!("start program");

        let my_savings = Arc::new(0001);
        let feed_account = my_savings.clone(); // clones the ref, not the item
        let mobile_account = my_savings.clone();


        for y in -my..my {

            (-mx..mx).into_par_iter().for_each(|x| //.map(|x|
                {

                let mut n = 0;

                // zx = scaled x coordinate of pixel (scaled to lie in the Mandelbrot X scale (-2.5, 1))
                // zy = scaled y coordinate of pixel (scaled to lie in the Mandelbrot Y scale (-1, 1))
                // float2 z = float2(zx, zy); //Z is originally set to the pixel coordinates

                let mut zxy = Complex {
                    re: x as f64 * 4.0 / (my_input - 2) as f64,
                    im: -(y as f64 * 4.0 / (mx_input + 2) as f64),
                };


                while n < iter {
                    // TODO: change 3 to `pow`
                    zxy = sub(zxy, div(zfunc(zxy), dfunc(zxy, 3 as f64)));
                    n = n + 1;
                }

                if abs(sub(zxy, r1)) < tolerance {
                    &mut (imgbuf.put_pixel(i_to_u(x, mx), i_to_u(y, my), image::Rgb([255, 0, 0])));
                }

                if abs(sub(zxy, r2)) <= tolerance {
                    imgbuf.put_pixel(i_to_u(x, mx), i_to_u(y, my), image::Rgb([0, 255, 0]));
                }

                if abs(sub(zxy, r3)) <= tolerance {
                    imgbuf.put_pixel(i_to_u(x, mx), i_to_u(y, my), image::Rgb([0, 0, 255]));
                }
            }
            );


            for x in -mx..mx {

                let mut n = 0;

                // zx = scaled x coordinate of pixel (scaled to lie in the Mandelbrot X scale (-2.5, 1))
                // zy = scaled y coordinate of pixel (scaled to lie in the Mandelbrot Y scale (-1, 1))
                // float2 z = float2(zx, zy); //Z is originally set to the pixel coordinates

                let mut zxy = Complex {
                    re: x as f64 * 4.0 / (my_input - 2) as f64,
                    im: -(y as f64 * 4.0 / (mx_input + 2) as f64),
                };


                while n < iter {
                    // TODO: change 3 to `pow`
                    zxy = sub(zxy, div(zfunc(zxy), dfunc(zxy, 3 as f64)));// div(sub(zxy, zfunc(zxy)), dfunc(zxy, 3 as f64)); // Wrong formula
                    n = n + 1;
                }



                if abs(sub(zxy, r1)) < tolerance {
                    imgbuf.put_pixel(i_to_u(x, mx), i_to_u(y, my), image::Rgb([255, 0, 0]));
                }

                if abs(sub(zxy, r2)) <= tolerance {
                    imgbuf.put_pixel(i_to_u(x, mx), i_to_u(y, my), image::Rgb([0, 255, 0]));
                }

                if abs(sub(zxy, r3)) <= tolerance {
                    imgbuf.put_pixel(i_to_u(x, mx), i_to_u(y, my), image::Rgb([0, 0, 255]));
                }
            }
        }

        imgbuf.save("fractal.png").expect("error in creation PNG");

        info!("end program")
    }

    pub fn i_to_u(point: i32, canvas: i32) -> u32 {
        (point + canvas) as u32
    }
}
