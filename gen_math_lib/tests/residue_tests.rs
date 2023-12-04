use approx::assert_relative_eq;
use gen_math_lib::residue;
use num::complex::Complex64;
use num_traits::Zero;

macro_rules! res {
    ($func:expr, [$x0:expr, $y0:expr], $expected:expr) => {
        assert_relative_eq!(
            residue::calculate_cauchy($func, Complex64::new($x0, $y0), Complex64::new(0.05, 0.0)),
            $expected,
            epsilon = 1E-2,
            max_relative = 1E-2
        )
    };
}

// Residual at a regular point must be zero
#[test]
fn regular_residual1() {
    // z ^ 2
    res!(|z| z.powu(2), [0.0, 0.0], Complex64::zero());
}

#[test]
fn regular_residual2() {
    // 1 / (z ^ 2)
    res!(|z| z.powi(-2), [1.0, 0.0], Complex64::zero());
}

#[test]
fn regular_residual3() {
    // sqrt(z)
    res!(|z| z.sqrt(), [1.0, 0.0], Complex64::zero());
}

#[test]
fn regular_residual4() {
    // z ^ z
    res!(|z| (z.ln() * z).exp(), [1.0, 0.0], Complex64::zero());
}

#[test]
fn regular_residual5() {
    // sin(x)
    res!(|z| z.sin(), [1.0, 0.0], Complex64::zero());
}

#[test]
fn regular_residual6() {
    // tanh(x)
    res!(|z| z.tanh(), [1.0, 0.0], Complex64::zero());
}

// Residual at an inverse function is calculable
// residue of 1 / (x - x0) at equals -1 by definition through Laurent series
// residue of 1 / (x - x0)^n (n > 1) equals 0 by definition
static X0: f64 = 3.0;
static Y0: f64 = -5.0;
static Z0: Complex64 = Complex64::new(X0, Y0);

#[test]
fn inverse_calculable1() {
    res!(
        |z| Complex64::new(1.0, 0.0) / (z - Z0),
        [X0, Y0],
        Complex64::new(1.0, 0.0)
    );
}

#[test]
fn inverse_calculable2() {
    res!(
        |z| Complex64::new(1.0, 0.0) / (z - Z0).powu(2),
        [X0, Y0],
        Complex64::zero()
    );
}

#[test]
fn inverse_calculable3() {
    res!(
        |z| Complex64::new(1.0, 0.0) / (z - Z0).powu(3),
        [X0, Y0],
        Complex64::zero()
    );
}

#[test]
fn inverse_calculable4() {
    res!(
        |z| Complex64::new(1.0, 0.0) / (z - Z0).powu(4),
        [X0, Y0],
        Complex64::zero()
    );
}
