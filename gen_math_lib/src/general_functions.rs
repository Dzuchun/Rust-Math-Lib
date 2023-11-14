use crate::integration::euler;
use crate::macro_functions::*;
use crate::traits::Metrized;

pub fn e1() -> impl Fn(f64) -> f64 {
    let exp = exp();
    move |x: f64| {
        euler(x, 1000.0, 1E-10, 1E-3, |t: f64| {
            if t == 0.0 {
                1E3
            } else {
                exp(-t) / t
            }
        })
    }
}

pub fn ei() -> impl Fn(f64) -> Option<f64> {
    let exp = exp();
    move |x: f64| {
        (x > 0.0).then_some(euler(-1000.0, x, 1E-10, 1E-3, |t: f64| {
            if t.distance(&0.0) < 1e-3 {
                1000.0
            } else {
                exp(t) / t
            }
        }))
    }
}

pub fn li() -> impl Fn(f64) -> Option<f64> {
    let ln = ln();
    move |x: f64| {
        if x <= 1.0 {
            None
        } else {
            Some(euler(0.0, x, 1E-12, 1E-4, |t: f64| {
                if t == 0.0 {
                    0.0
                } else if t.distance(&1.0) < 1E-3 {
                    1000.0
                } else {
                    1.0 / ln(t).unwrap()
                }
            }))
        }
    }
}
