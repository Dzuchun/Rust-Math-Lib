use num::complex::Complex64;
use num_traits::Zero;

use crate::{integration, progression};

// TODO generalize this, I guess
pub fn calculate_cauchy(
    function: impl Fn(Complex64) -> Complex64,
    z0: Complex64,
    r: Complex64,
) -> Complex64 {
    static TWO_PI: f64 = 2.0 * std::f64::consts::PI;
    integration::integrate(
        Complex64::zero(),
        0.0,
        // function should've had (r * i) multiplier, but I decided to multiply by these once instead
        |_, t| function(z0 + r * Complex64::cis(t)) * Complex64::cis(t),
        progression::arithmetic_bounded(0.0, TWO_PI, 1E-5),
        integration::rk7_step,
    )
    .last()
    .expect("Should be present, since supplied progression has at least one element")
    .0 * r
        / Complex64::new(TWO_PI, 0.0)
}
