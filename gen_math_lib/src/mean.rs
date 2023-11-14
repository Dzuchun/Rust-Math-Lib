use std::{
    iter::Iterator,
    ops::{AddAssign, Div, MulAssign},
};

use num_traits::{One, Pow, Zero};

pub fn arith<T, I>(nums: I) -> Option<T>
where
    I: IntoIterator<Item = T>,
    T: Zero + AddAssign + FromPrimitive + Div<Output = T>,
{
    let fold = nums.into_iter().fold((T::zero(), 0usize), |mut acc, next| {
        acc.0 += next;
        acc.1 += 1;
        acc
    });
    if fold.1 == 0 {
        return None;
    }
    Some(fold.0.div(T::from_usize(fold.1)?))
}

pub fn harmonic<T, Ti, I>(nums: I) -> Option<T>
where
    I: IntoIterator<Item = T>,
    T: Reciprocal<Ti>,
    Ti: Zero + AddAssign + Reciprocal<T> + Div<Output = Ti> + FromPrimitive,
{
    let fold = nums
        .into_iter()
        .try_fold((Ti::zero(), 0usize), |mut acc, next| {
            acc.0 += next.invs()?;
            acc.1 += 1;
            Some(acc)
        })?;
    if fold.1 == 0 {
        return None;
    }
    let divided: Ti = fold.0.div(Ti::from_usize(fold.1)?);
    divided.invs()
}

pub fn geometric<T, Ti, I, P>(nums: I) -> Option<T>
where
    I: IntoIterator<Item = T>,
    Ti: One + MulAssign<T> + Pow<P, Output = T>,
    P: FromPrimitive,
{
    let fold = nums.into_iter().fold((Ti::one(), 0), |mut acc, next| {
        acc.0 *= next;
        acc.1 += 1;
        acc
    });
    if fold.1 == 0 {
        return None;
    }
    Some(fold.0.pow(P::from_f64(fold.1.invs()?)?))
}

use num_traits::FromPrimitive;

use crate::traits::{Reciprocal, Reversible};

pub fn general<T, Ti, I, F>(func: F, nums: I) -> Option<T>
where
    I: IntoIterator<Item = T>,
    Ti: Zero + AddAssign + Div<Output = Ti> + FromPrimitive,
    F: Reversible<T, Ti>,
{
    let mut sum = Ti::zero();
    let mut count = 0;
    for next in nums {
        sum += func.fwd_checked(next)?;
        count += 1;
    }
    if count == 0 {
        return None;
    }
    Some(func.bwd(sum.div(Ti::from_usize(count)?)))
}
