extern crate wasm_bindgen;

pub mod wasmimpl {
    use complex::complex::{abs, div, mul, scale, sub, sub_f64, Complex};
    use std::f64;
    use std::thread::spawn;
    use wasm_bindgen::prelude::*;

    const NTHREADS: usize = 8;

    // #[wasm_bindgen]
    // pub fn choose_color(x: i32, y: i32, n: i32) -> [u8; 3] {
    //     [0, 0, 0]
    // }

    // F(x) = x^numRoots - 1
    // TODO: add pow
    #[wasm_bindgen]
    pub fn zfunc(z: Complex) -> Complex {
        sub_f64(mul(z, mul(z, z)), 1.0)
    }

    // dF(x) = numRoots*x^(numRoots -1)
    // TODO: pow
    #[wasm_bindgen]
    pub fn dfunc(z: Complex) -> Complex {
        scale(mul(z, z), 3 as f64)
    }

    #[allow(dead_code)]
    #[wasm_bindgen]
    pub fn wasmdraw(mx_input: i32, my_input: i32, iter: u32) -> &[i32] {
        let tolerance = 0.00001;

        // Thread 1
        let r1 = Complex { re: 1.0, im: 0.0 };

        // Thread 2
        let r2 = Complex {
            re: -0.5,
            im: 3.0_f64.sqrt() / 2.0,
        };

        // Thread 3
        let r3 = Complex {
            re: -0.5,
            im: -3.0_f64.sqrt() / 2.0,
        };

        let mut imgbuf = [u32; mx_input * my_input * 12];

        let mx = mx_input / 2;
        let my = my_input / 2;

        for y in -my..my {
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
                    zxy = sub(zxy, div(zfunc(zxy), dfunc(zxy)));
                    n = n + 1;
                }

                if abs(sub(zxy, r1)) < tolerance {
                    // How to convert (u32, u32, [u32;3]) to [u32]
                    imgbuf.push(i_to_u(x, mx));
                    imgbuf.push(i_to_u(y, my));
                    imgbuf.push(255);
                    imgbuf.push(0);
                    imgbuf.push(0);
                }

                if abs(sub(zxy, r2)) <= tolerance {
                    imgbuf.push(i_to_u(x, mx));
                    imgbuf.push(i_to_u(y, my));
                    imgbuf.push(0);
                    imgbuf.push(255);
                    imgbuf.push(0);
                }

                if abs(sub(zxy, r3)) <= tolerance {
                    imgbuf.push(i_to_u(x, mx));
                    imgbuf.push(i_to_u(y, my));
                    imgbuf.push(0);
                    imgbuf.push(0);
                    imgbuf.push(255);
                }
            }
        }

        imgbuf
    }

    #[wasm_bindgen]
    pub fn i_to_u(point: i32, canvas: i32) -> u32 {
        (point + canvas) as u32
    }
}
