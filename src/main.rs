extern crate image;
extern crate rustfractals;
extern crate num;
extern crate num_complex;

use rustfractals::newtone_fractal;

static ITER: u32 = 100;
static MIN: f64 = 1e-6;
static MAX: f64 = 1e+6;
static WIDTH: i32 = 500;
static HEIGHT: i32 = 500;
static X_0: f64 = -0.7;
static X_N: f64 = 1.0;
static Y_0: f64 = -1.0;
static Y_N: f64 = 0.7;

fn main() {
    newtone_fractal::draw(WIDTH, HEIGHT, ITER, X_0, X_N, Y_0, Y_N);
}
