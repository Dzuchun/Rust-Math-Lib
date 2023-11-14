use std::{
    iter::Iterator,
    ops::{Add, Mul},
};

use num_traits::Zero;

struct ArithmeticIterator<V> {
    current: V,
    step: V,
}

impl<V> Iterator for ArithmeticIterator<V>
where
    V: Clone + Add<V, Output = V>,
{
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        let new = self.current.clone() + self.step.clone();
        Some(std::mem::replace(&mut self.current, new))
    }
}

pub fn arithmetic<V>(start: V, step: V) -> impl Iterator<Item = V>
where
    V: Clone + Add<V, Output = V>,
{
    ArithmeticIterator {
        current: start,
        step,
    }
}

pub fn arithmetic_bounded<V>(start: V, end: V, step: V) -> impl Iterator<Item = V>
where
    V: Clone + Add<V, Output = V> + PartialOrd + Zero,
{
    let empty = matches!(
        start.partial_cmp(&end),
        Some(std::cmp::Ordering::Equal) | None
    );
    let finite = start.partial_cmp(&end) == V::zero().partial_cmp(&step);
    let go = !empty & finite;
    arithmetic(start.clone(), step)
        .take_while(move |v| go && start.partial_cmp(&end) == v.partial_cmp(&end))
}

struct GeometricIterator<V, M> {
    current: V,
    denominator: M,
}

impl<V, M> Iterator for GeometricIterator<V, M>
where
    V: Clone + Mul<M, Output = V>,
    M: Clone,
{
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        let new = self.current.clone() * self.denominator.clone();
        Some(std::mem::replace(&mut self.current, new))
    }
}

pub fn geometric<V, M>(start: V, denominator: M) -> impl Iterator<Item = V>
where
    V: Clone + Mul<M, Output = V>,
    M: Clone,
{
    GeometricIterator {
        current: start,
        denominator,
    }
}
