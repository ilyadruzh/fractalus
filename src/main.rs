extern crate chrono;
extern crate fern;
extern crate image;
extern crate num;
extern crate rustfractals;
#[macro_use]
extern crate log;

use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, Data, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};
use rustfractals::complex::complex::Complex;
use rustfractals::logger;
use rustfractals::logger::logger::setup_logging;
use rustfractals::newton_fractal;
use rustfractals::wasm_impl;
use std::io::Write;
use std::str::FromStr;

static W: i32 = 1000;
static H: i32 = 1000;
static ITER: u32 = 100;
static Z0: Complex = Complex { re: -0.7, im: -1.0 };
static ZN: Complex = Complex { re: 1.0, im: 0.7 };

fn main() -> Result<(), PlatformError> {
    setup_logging(3).expect("failed to initialize logging.");

    /// Draw PNG file with Newton's fractal
    newton_fractal::newtone_fractal::draw("fractal.png", W, H, ITER, Z0, ZN); // TODO formula as param

    // /// 2D visualization for Newton's fractal
    // visualize::visualize::show(W, H, ITER, Z0, Z1) 2D graphics
    // /// Wasm implementation
    // wasmimpl::wasmimpl::wasmdraw(W, H, ITER);

    // Window builder. We set title and size
    let main_window = WindowDesc::new(ui_builder)
        .title("Рисуем Бассейн Ньютона!")
        .window_size((200.0, 100.0));

    // Data to be used in the app (=state)
    let data: Counter = Counter(0);

    // Run the app
    AppLauncher::with_window(main_window)
        .use_simple_logger() // Neat!
        .launch(data)
}
