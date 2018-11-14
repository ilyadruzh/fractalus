extern crate image;
extern crate num;
extern crate rustfractals;

use rustfractals::newtone_fractal;
use rustfractals::complex::complex::Complex;
use std::io::Write;
use std::str::FromStr;



fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 6 {
        writeln!(
            std::io::stderr(),
            "How to run: rustfractals FILENAME PIXELS UPPERLEFT LOWERRIGHT ITERATION"
        ).unwrap();

        writeln!(
            std::io::stderr(),
            "Example: {} newton-fractal.png 1000x750 -0.7,-1.0 1.0,0.7 500",
            args[0]
        ).unwrap();

        std::process::exit(1);
    }
    let bounds = parse_pair(&args[2], 'x')
        .expect("");

    let upper_left = parse_complex(&args[3]).expect("");
    let lower_right = parse_complex(&args[4]).expect("");
    let iter = parse_u32(&args[5]).expect("");

    let mut pixels = vec![0; bounds.0 as usize * bounds.1 as usize];

    newtone_fractal::draw(bounds.0, bounds.1, iter, upper_left, lower_right);
}

fn parse_complex(s: &str) -> Option<Complex> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

fn parse_u32(s: &str) -> Result<u32, std::num::ParseIntError> {
    match u32::from_str(s) {
        Ok(res) => Ok(res),
        Err(err) => Err(err),
    }
}
