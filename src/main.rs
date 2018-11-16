extern crate image;
extern crate num;
extern crate rustfractals;

use rustfractals::complex::complex::Complex;
use rustfractals::newtonfractal;
use rustfractals::wasmimpl;
use std::io::Write;
use std::str::FromStr;


static W: i32 = 100;
static H: i32 = 100;
static ITER: u32 = 100;
static Z0: Complex = Complex { re: -0.7, im: -1.0 };
static ZN: Complex = Complex { re: 1.0, im: 0.7 };


fn main() {
    newtonfractal::newtone_fractal::draw(W, H, ITER, Z0, ZN);
    wasmimpl::wasmimpl::wasmdraw(W, H, ITER);
}