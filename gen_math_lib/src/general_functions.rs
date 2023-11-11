use crate::integration::euler;
use crate::macro_functions::*;
use crate::traits::Metrized;

pub const E1: fn(f64) -> f64 = |x: f64| {
    euler(x, 1000.0, 1E-10, 1E-3, |t: f64| {
        if t == 0.0 {
            1E3
        } else {
            EXP(-t) / t
        }
    })
};

pub const EI: fn(f64) -> Option<f64> = |x: f64| {
    if x <= 0.0 {
        None
    } else {
        Some(euler(-1000.0, x, 1E-10, 1E-3, |t: f64| {
            if t.distance(&0.0) < 1e-3 {
                100.0
            } else {
                EXP(t) / t
            }
        }))
    }
};

pub const LI: fn(f64) -> Option<f64> = |x: f64| {
    if x <= 1.0 {
        None
    } else {
        Some(euler(0.0, x, 1E-12, 1E-4, |t: f64| {
            if t == 0.0 {
                0.0
            } else if t.distance(&1.0) < 1E-3 {
                100.0
            } else {
                1.0 / LN(t).unwrap()
            }
        }))
    }
};
