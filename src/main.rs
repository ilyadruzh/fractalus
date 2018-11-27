extern crate image;
extern crate num;
extern crate rustfractals;

use rustfractals::complex::complex::Complex;
use rustfractals::newton_fractal;
use rustfractals::wasm_impl;
use std::io::Write;
use std::str::FromStr;

static W: i32 = 1000;
static H: i32 = 1000;
static ITER: u32 = 1000;
static Z0: Complex = Complex { re: -0.7, im: -1.0 };
static ZN: Complex = Complex { re: 1.0, im: 0.7 };


fn main() {
    newton_fractal::newtone_fractal::draw(W, H, ITER, Z0, ZN);
    // wasmimpl::wasmimpl::wasmdraw(W, H, ITER);
}