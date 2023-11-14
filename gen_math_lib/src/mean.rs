use std::{
    iter::Iterator,
    ops::{AddAssign, Div, MulAssign},
};

use num_traits::{One, Pow, Zero};

pub fn arith<T, I, D>(nums: I) -> T
where
    I: IntoIterator<Item = T>,
    T: Zero + AddAssign + FromPrimitive + Div<D, Output = T>,
    D: FromPrimitive,
{
    let fold = nums.into_iter().fold((T::zero(), 0usize), |mut acc, next| {
        acc.0 += next;
        acc.1 += 1;
        acc
    });
    fold.0 / D::from_usize(fold.1).expect("Should be able to convert into type D")
}

pub fn harmonic<T, Ti, I, D>(nums: I) -> T
where
    I: IntoIterator<Item = T>,
    T: Reciprocal<Ti>,
    Ti: Zero + AddAssign + Reciprocal<T> + Div<D, Output = Ti> + FromPrimitive,
    D: FromPrimitive,
{
    let fold = nums
        .into_iter()
        .fold((Ti::zero(), 0usize), |mut acc, next| {
            acc.0 += next.invs();
            acc.1 += 1;
            acc
        });
    let divided: Ti =
        fold.0 / D::from_usize(fold.1).expect("Should be able to convert into type D");
    divided.invs()
}

pub fn geometric<T, Ti, I, P>(nums: I) -> T
where
    I: IntoIterator<Item = T>,
    Ti: One + MulAssign<T> + Pow<P, Output = T>,
    P: From<f64>,
{
    let fold = nums.into_iter().fold((Ti::one(), 0), |mut acc, next| {
        acc.0 *= next;
        acc.1 += 1;
        acc
    });
    fold.0.pow(P::from(fold.1.invs()))
}

use num_traits::FromPrimitive;

use crate::traits::{Reciprocal, Reversible};

pub fn general<T, Ti, I, F, D>(func: F, nums: I) -> T
where
    I: IntoIterator<Item = T>,
    Ti: Zero + AddAssign + Div<D, Output = Ti> + FromPrimitive,
    F: Reversible<T, Ti>,
    D: FromPrimitive,
{
    let mut sum = Ti::zero();
    let mut count = 0;
    for next in nums {
        sum += func.fwd(next);
        count += 1;
    }
    func.bwd(sum / D::from_usize(count).expect("Should be able to convert into type D"))
}
