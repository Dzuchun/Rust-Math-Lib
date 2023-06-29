use std::{
    iter::Sum,
    ops::{Add, Mul, Neg},
};

use crate::{
    progression::{arith as arith_iterator, ArithIteratorType},
    traits::{Halfable, Metrized},
};

pub fn euler<T, F, R>(begin: T, end: T, func: F, min_step: T, start_step: T) -> R
where
    T: ArithIteratorType + Halfable + Add<T, Output = T>,
    F: Fn(T) -> R,
    R: Add<R, Output = R>
        + Mul<T, Output = R>
        + Sum
        + Neg<Output = R>
        + Metrized
        + Clone
        + Halfable,
{
    if end < begin {
        return -euler(end, begin, func, min_step, start_step);
    }
    let mut step: T = start_step;
    step.half();
    step.half();
    let mut guess: R = arith_iterator(begin, end, step)
        .unwrap()
        .map(&func)
        .sum::<R>()
        * step;
    while step > min_step {
        let iterator_result = arith_iterator(begin, end, step);
        if iterator_result.is_err() {
            break;
        }
        let better_result: R = iterator_result.unwrap().map(&func).sum::<R>() * step;
        if better_result.distance(&guess) < 1E-10 {
            break;
        }
        guess = better_result;
        step.half();
    }
    guess
}
