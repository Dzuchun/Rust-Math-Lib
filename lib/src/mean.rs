use std::iter::Iterator;

pub fn arith<T>(nums: &mut T) -> Option<f64>
where
    T: Iterator<Item = f64>,
{
    let mut sum: f64 = nums.next()?;
    let mut l: usize = 1;
    while let Some(n) = nums.next() {
        sum += n;
        l += 1;
    }
    Some(sum / (l as f64))
}

pub fn harmonic<T>(nums: &mut T) -> Option<f64>
where
    T: Iterator<Item = f64>,
{
    let first: f64 = nums.next()?;
    if first == 0.0 {
        return Some(0.0);
    }
    if first < 0.0 {
        return None;
    }
    let mut sum: f64 = 1.0 / first;
    let mut l: usize = 1;
    while let Some(n) = nums.next() {
        if n == 0.0 {
            return Some(0.0);
        }
        if n < 0.0 {
            return None;
        }
        sum += 1.0 / n;
        l += 1;
    }
    if sum == 0.0 {
        return Some(f64::INFINITY);
    }
    Some((l as f64) / sum)
}

pub fn geometric<T>(nums: &mut T) -> Option<f64>
where
    T: Iterator<Item = f64>,
{
    let mut prod: f64 = nums.next()?;
    let mut l: usize = 1;
    while let Some(n) = nums.next() {
        prod *= n;
        if prod == 0.0 {
            return Some(0.0);
        }
        l += 1;
    }
    Some(prod.powf(1.0 / (l as f64)))
}

use crate::traits::Reversible;

pub fn init_general<T>(func: Reversible<f64, f64>) -> Box<dyn Fn(&mut T) -> Option<f64>>
where
    T: Iterator<Item = f64>,
{
    Box::new(move |nums: &mut T| {
        let first: f64 = nums.next()?;
        let mut sum = func.call(first)?;
        let mut l: usize = 1;
        while let Some(n) = nums.next() {
            sum += func.call(n)?;
            l += 1;
        }
        Some(func.rev(sum / (l as f64))?)
    })
}
