#![allow(unused_attributes)]

use std::{
    iter::Sum,
    ops::{Add, Mul, Neg, Sub},
};

use kutta_macros::general_kutta_step;
use num_traits::Zero;

use crate::{
    progression::arithmetic_bounded,
    traits::{Halfable, Metrized},
    util::FnIter,
};

pub fn euler<T, F, R, M>(begin: T, end: T, min_step: T, start_step: T, func: F) -> R
where
    T: Add<T, Output = T> + PartialOrd + Clone + Halfable + Zero,
    F: Fn(T) -> R,
    R: Add<R, Output = R>
        + Mul<T, Output = R>
        + Sum
        + Neg<Output = R>
        + Metrized<Output = M>
        + Clone
        + Halfable,
    M: PartialOrd<f64>,
{
    if end < begin {
        return -euler(end, begin, min_step, start_step, func);
    }
    let mut step: T = start_step;
    step.half();
    step.half();
    let mut guess: R = arithmetic_bounded(begin.clone(), end.clone(), step.clone())
        .map(&func)
        .sum::<R>()
        * step.clone();
    while step > min_step {
        let iterator_result = arithmetic_bounded(begin.clone(), end.clone(), step.clone());
        let better_result: R = iterator_result.map(&func).sum::<R>() * step.clone();
        if better_result.distance(&guess) < 1E-10 {
            break;
        }
        guess = better_result;
        step.half();
    }
    guess
}

#[allow(non_upper_case_globals)]
general_kutta_step! {
    _euler_step:
    0   |
    ------
        | 1
}

#[allow(non_upper_case_globals)]
general_kutta_step! {
    _three_eight_step:
    0   |
    1/3 |  1/3
    2/3 | -1/3  1
    1   |  1   -1   1
    -------
        | 1/8 3/8 3/8 1/8
}

#[allow(non_upper_case_globals)]
general_kutta_step! {
    _rk4_step:
    0   |
    1/2 | 1/2
    1/2 | 0   1/2
    1   | 0   0   1
    -------
        | 1/6 1/3 1/3 1/6
}

#[allow(non_upper_case_globals)]
general_kutta_step! {
    _rk6_step:
    0   |
    1/2 | !4.0f64.sqrt()
    1/2 | 0   1/2
    1   | 0   0   1
    -------
        | 1/6 1/3 1/3 1/6
}

macro_rules! public {
    ($internal:ident, $public:ident) => {
        pub fn $public<X, T, D, Der>(x0: X, t0: T, dt: T, der: &Der) -> X
        where
            X: Clone + Add<Output = X> + Mul<f64, Output = X>,
            T: Clone + Add<Output = T> + Mul<f64, Output = T>,
            D: Clone + Add<Output = D> + Mul<f64, Output = D> + Mul<T, Output = X>,
            Der: Fn(X, T) -> D,
        {
            $internal(x0, t0, dt, der)
        }
    };
}

public! {_rk4_step, rk4_step}

public! {_rk6_step, rk6_step}

pub fn integrate<'inp, 'out, X, T, D, I, Der, Step>(
    mut x: X,
    mut t: T,
    der: Der,
    mut steps: I,
    step: Step,
) -> impl Iterator<Item = (X, T)> + 'out
where
    'inp: 'out,
    X: Clone + Add<Output = X> + Mul<f64, Output = X> + 'inp,
    T: Clone + Add<Output = T> + Mul<f64, Output = T> + Sub<Output = T> + 'inp,
    I: Iterator<Item = T> + 'inp,
    Step: Fn(X, T, T, &Der) -> X + 'inp,
    Der: Fn(X, T) -> D + 'inp,
{
    FnIter::from(move || {
        let next_t = steps.next()?;
        let t0 = std::mem::replace(&mut t, next_t.clone());
        let dt = next_t - t0.clone();
        x = (step)(x.clone(), t0, dt, &der);
        Some((x.clone(), t.clone()))
    })
}
