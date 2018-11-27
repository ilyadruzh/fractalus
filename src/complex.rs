extern crate wasm_bindgen;

pub mod complex {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    #[derive(Copy, Clone, Debug)]
    pub struct Complex {
        pub re: f64,
        pub im: f64,
    }

    #[wasm_bindgen]
    pub fn mul(self_: Complex, other: Complex) -> Complex {
        Complex {
            re: (self_.re.clone() * other.re.clone()) - (self_.im.clone() * other.im.clone()),
            im: (self_.re * other.im) + (self_.im * other.re),
        }
    }

    #[wasm_bindgen]
    pub fn add(self_: Complex, other: Complex) -> Complex {
        Complex {
            re: self_.re + other.re,
            im: self_.im + other.im,
        }
    }

    #[wasm_bindgen]
    pub fn sub(self_: Complex, other: Complex) -> Complex {
        Complex {
            re: self_.re - other.re,
            im: self_.im - other.im,
        }
    }

    #[wasm_bindgen]
    pub fn sub_f64(self_: Complex, other: f64) -> Complex {
        Complex {
            re: self_.re - other,
            im: self_.im,
        }
    }

    #[wasm_bindgen]
    pub fn abs(z: Complex) -> f64 {
        f64::sqrt((z.re * z.re) + (z.im * z.im))
    }

    #[wasm_bindgen]
    pub fn arg(z: Complex) -> f64 {
        z.im.atan2(z.re)
    }

    // Done
    #[wasm_bindgen]
    pub fn scale(z: Complex, n: f64) -> Complex {
        Complex {
            re: z.re.clone() * n.clone(),
            im: z.im.clone() * n,
        }
    }

    // Done
    #[wasm_bindgen]
    pub fn norm_sqr(self_: Complex) -> f64 {
        self_.re.clone() * self_.re.clone() + self_.im.clone() * self_.im.clone()
    }

    // Done
    #[wasm_bindgen]
    pub fn div(self_: Complex, other: Complex) -> Complex {
        let norm_sqr = norm_sqr(other);

        Complex {
            re: (self_.re.clone() * other.re.clone() + self_.im.clone() * other.im.clone())
                / norm_sqr.clone(),
            im: (self_.im * other.re - self_.re * other.im) / norm_sqr.clone(),
        }
    }
}
