use std::{
    iter::Sum,
    ops::{Add, Mul, Neg},
};

use num_traits::Zero;

use crate::{
    progression::arithmetic_bounded,
    traits::{Halfable, Metrized},
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
