extern crate image;
extern crate num;
extern crate rustfractals;
extern crate fern;
extern crate chrono;
#[macro_use]
extern crate log;

use rustfractals::complex::complex::Complex;
use rustfractals::newton_fractal;
use rustfractals::wasm_impl;
use rustfractals::logger;
use std::io::Write;
use std::str::FromStr;
use rustfractals::logger::logger::setup_logging;

static W: i32 = 1000;
static H: i32 = 1000;
static ITER: u32 = 100;
static Z0: Complex = Complex { re: -0.7, im: -1.0 };
static ZN: Complex = Complex { re: 1.0, im: 0.7 };


fn main() {

    setup_logging(3).expect("failed to initialize logging.");

    /// Draw PNG file with Newton's fractal
    newton_fractal::newtone_fractal::draw("fractal.png", W, H, ITER, Z0, ZN); // TODO formula as param

    // /// 2D visualization for Newton's fractal
    // visualize::visualize::show(W, H, ITER, Z0, Z1) 2D graphics
    // /// Wasm implementation
    // wasmimpl::wasmimpl::wasmdraw(W, H, ITER);
}