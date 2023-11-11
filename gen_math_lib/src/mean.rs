use std::{
    iter::Iterator,
    ops::{AddAssign, Div, MulAssign},
};

use num_traits::{One, Pow, Zero};

pub fn arith<T, I>(nums: I) -> Option<T>
where
    I: IntoIterator<Item = T>,
    T: Zero + AddAssign + Div<Output = T> + FromPrimitive,
{
    let fold = nums.into_iter().fold((T::zero(), 0usize), |mut acc, next| {
        acc.0 += next;
        acc.1 += 1;
        acc
    });
    Some(fold.0.div(T::from_usize(fold.1)?))
}

pub fn harmonic<T, Ti, I>(nums: I) -> Option<T>
where
    I: IntoIterator<Item = T>,
    T: Reciprocal<Output = Ti>,
    Ti: Zero + AddAssign + Reciprocal<Output = T> + Div<Output = Ti> + FromPrimitive,
{
    let fold = nums
        .into_iter()
        .try_fold((Ti::zero(), 0usize), |mut acc, next| {
            acc.0 += next.invs()?;
            acc.1 += 1;
            Some(acc)
        })?;
    let divided: Ti = fold.0.div(Ti::from_usize(fold.1)?);
    divided.invs()
}

pub fn geometric<T, Ti, I, P>(nums: I) -> Option<T>
where
    I: IntoIterator<Item = T>,
    Ti: One + MulAssign<T> + Pow<P, Output = T>,
    P: FromPrimitive,
{
    let fold = nums.into_iter().fold((Ti::one(), 0.0), |mut acc, next| {
        acc.0 *= next;
        acc.1 += 1.0;
        acc
    });
    Some(fold.0.pow(P::from_f32(1.0 / fold.1)?))
}

use num_traits::FromPrimitive;

use crate::traits::{Reciprocal, Reversible};

pub fn general<T, Ti, I, F>(func: F, nums: I) -> Option<T>
where
    I: IntoIterator<Item = T>,
    Ti: Zero + AddAssign + Div<Output = Ti> + FromPrimitive,
    F: Reversible<T, Ti>,
{
    let fold = nums
        .into_iter()
        .fold((Ti::zero(), 0usize), |mut acc, next| {
            acc.0 += func.fwd(next);
            acc.1 += 1;
            acc
        });
    Some(func.bwd(fold.0.div(Ti::from_usize(fold.1)?)))
}
