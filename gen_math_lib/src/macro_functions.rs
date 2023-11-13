use function_macros::{function_factored_absolute_tailor, function_factored_relative_tailor};

const EXP_M_2: fn(f64) -> f64 =
    function_factored_relative_tailor!(30, if ^ == 0.0 {1.0} else { 1.0 / ^ });
pub const EXP: fn(f64) -> f64 = |x: f64| {
    let mut additions = 0;
    let mut x = x;
    while x < -2.0 {
        additions -= 1;
        x += 2.0;
    }
    while x > 2.0 {
        additions += 1;
        x -= 2.0;
    }
    EXP_M_2(x) * EXP_M_2(2.0f64).powi(additions)
};

const LN_X_M_1: fn(f64) -> f64 = function_factored_absolute_tailor!(
    30,
    if n == 0.0 {
        0.0
    } else {
        if n % 2.0 == 0.0 {
            -1.0 / (n as f64)
        } else {
            1.0 / (n as f64)
        }
    },
    f64
);

pub const LN: fn(f64) -> Option<f64> = |x: f64| {
    if x <= 0.0 {
        return None;
    }
    let mut additions = 0;
    let mut x = x;
    while x < 0.7 {
        additions -= 1;
        x *= 1.5;
    }
    while x > 1.5 {
        additions += 1;
        x /= 1.5;
    }
    Some(LN_X_M_1(x - 1.0) + (additions as f64) * LN_X_M_1(0.5f64))
};

const SIN_M_2: fn(f64) -> f64 =
    fffbt!(25, 0.0, if (^ as i32) == 1  { 1.0 } else { -1.0 / ^ / (^ - 1.0) });

pub const SIN: fn(f64) -> f64 = |x: f64| {
    use std::f64::consts::{FRAC_PI_2, PI, TAU};

    let x = if x > 0.0 { x % TAU } else { x % TAU + TAU };
    if x < PI {
        if x < FRAC_PI_2 {
            SIN_M_2(x)
        } else {
            SIN_M_2(PI - x)
        }
    } else {
        if x < FRAC_PI_2 * 3.0 {
            -SIN_M_2(x - PI)
        } else {
            -SIN_M_2(TAU - x)
        }
    }
};

pub const COS: fn(f64) -> f64 = |x: f64| SIN(std::f64::consts::FRAC_PI_2 - x);

pub const SINC: fn(f64) -> f64 = |x: f64| if x == 0.0 { 1.0 } else { SIN(x) / x };
