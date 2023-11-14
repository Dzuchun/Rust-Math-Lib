use function_macros::{factored_absolute_tailor, factored_relative_multitailor};
use num::traits::FromPrimitive;

fn float(n: usize) -> f64 {
    <f64 as FromPrimitive>::from_usize(n).expect("Why would you need that many terms?")
}

pub fn exp_m_2() -> impl Fn(f64) -> f64 {
    factored_relative_multitailor!(f64, 30, 1.0; 1.0/float(n))
}

pub fn exp() -> impl Fn(f64) -> f64 {
    let tlr = exp_m_2();
    let exp_2 = tlr(2.0f64);
    move |x: f64| {
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
        tlr(x) * exp_2.powi(additions)
    }
}

pub fn ln_x_r_1() -> impl Fn(f64) -> f64 {
    factored_absolute_tailor!(
        f64,
        30,
        if n == 0 {
            0.0
        } else {
            let nf = float(n);
            if n % 2 == 0 {
                -1.0 / nf
            } else {
                1.0 / nf
            }
        }
    )
}

pub fn ln() -> impl Fn(f64) -> Option<f64> {
    let tlr = ln_x_r_1();
    let ln_hf = tlr(0.5f64);
    move |x: f64| {
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
        Some(tlr(x - 1.0) + f64::from(additions) * ln_hf)
    }
}

pub fn sin_m_2() -> impl Fn(f64) -> f64 {
    factored_relative_multitailor!(f64, 25, 0.0, 1.0; 0.0, {
        let nf = float(n);
        -1.0 / nf / (nf - 1.0)
    })
}

pub fn sin() -> impl Fn(f64) -> f64 {
    use std::f64::consts::{FRAC_PI_2, PI, TAU};
    let tlr = sin_m_2();
    move |x: f64| {
        let x = if x > 0.0 { x % TAU } else { x % TAU + TAU };
        if x < PI {
            if x < FRAC_PI_2 {
                tlr(x)
            } else {
                tlr(PI - x)
            }
        } else if x < FRAC_PI_2 * 3.0 {
            -tlr(x - PI)
        } else {
            -tlr(TAU - x)
        }
    }
}

pub fn cos() -> impl Fn(f64) -> f64 {
    use std::f64::consts::FRAC_PI_2 as PIO2;
    let sin = sin();
    move |x: f64| sin(PIO2 - x)
}

pub fn sinc() -> impl Fn(f64) -> f64 {
    let sin = sin();
    move |x: f64| if x == 0.0 { 1.0 } else { sin(x) / x }
}
