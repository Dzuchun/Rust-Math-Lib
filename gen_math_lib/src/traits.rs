use std::{marker::PhantomData, ops::Div};

use num_traits::{CheckedMul, One, Pow, Signed};

pub trait Sequential {
    fn inc(&mut self);
    fn dec(&mut self);

    fn next(&self) -> Self;
    fn prev(&self) -> Self;
}

macro_rules! seq_impl {
    {$name:ty, $one:literal} => {
        impl Sequential for $name {
            fn inc(&mut self) {
                *self += $one;
            }

            fn dec(&mut self) {
                *self -= $one;
            }

            fn next(&self) -> Self {
                *self + $one
            }

            fn prev(&self) -> Self {
                *self - $one
            }
        }
    };
}

seq_impl! {i8, 1}
seq_impl! {i16, 1}
seq_impl! {i32, 1}
seq_impl! {i64, 1}
seq_impl! {i128, 1}
seq_impl! {isize, 1}

seq_impl! {u8, 1}
seq_impl! {u16, 1}
seq_impl! {u32, 1}
seq_impl! {u64, 1}
seq_impl! {u128, 1}
seq_impl! {usize, 1}

seq_impl! {f32, 1.0}
seq_impl! {f64, 1.0}

pub trait Reciprocal {
    type Output;
    fn rcp(&self) -> Option<Self::Output>;
    fn invs(self) -> Option<Self::Output>;
}

impl<T> Reciprocal for T
where
    T: Div<Self> + One + Clone,
{
    type Output = <T as Div>::Output;
    fn rcp(&self) -> Option<Self::Output> {
        self.clone().invs()
    }

    fn invs(self) -> Option<Self::Output> {
        Some(T::one().div(self))
    }
}

pub trait Reversible<X, Y> {
    fn fwd_checked(&self, x: X) -> Option<Y>;
    fn bwd_checked(&self, y: Y) -> Option<X>;
    fn fwd(&self, x: X) -> Y;
    fn bwd(&self, y: Y) -> X;
}

pub struct FnReversed<X, Y, Fwd, Bwd> {
    fwd_fn: Fwd,
    bwd_fn: Bwd,
    _ph: PhantomData<(X, Y)>,
}

pub fn fn_reversed<X, Y>(
    fwd: impl Fn(X) -> Option<Y>,
    bwd: impl Fn(Y) -> Option<X>,
) -> impl Reversible<X, Y> {
    FnReversed {
        fwd_fn: fwd,
        bwd_fn: bwd,
        _ph: PhantomData,
    }
}

impl<X, Y, Fwd, Bwd> Reversible<X, Y> for FnReversed<X, Y, Fwd, Bwd>
where
    Fwd: Fn(X) -> Option<Y>,
    Bwd: Fn(Y) -> Option<X>,
{
    fn fwd_checked(&self, x: X) -> Option<Y> {
        (self.fwd_fn)(x)
    }

    fn bwd_checked(&self, y: Y) -> Option<X> {
        (self.bwd_fn)(y)
    }

    fn fwd(&self, x: X) -> Y {
        (self.fwd_fn)(x).unwrap()
    }

    fn bwd(&self, y: Y) -> X {
        (self.bwd_fn)(y).unwrap()
    }
}

impl<X, Y, Fwd, Bwd, OX, OY> Reversible<X, Y> for (Fwd, Bwd)
where
    Fwd: Fn(X) -> OY,
    Bwd: Fn(Y) -> OX,
    OX: Into<Option<X>>,
    OY: Into<Option<Y>>,
{
    fn fwd_checked(&self, x: X) -> Option<Y> {
        self.0(x).into()
    }

    fn bwd_checked(&self, y: Y) -> Option<X> {
        self.1(y).into()
    }

    fn fwd(&self, x: X) -> Y {
        self.0(x).into().unwrap()
    }

    fn bwd(&self, y: Y) -> X {
        self.1(y).into().unwrap()
    }
}

pub trait Metrized {
    type Output;

    fn distance(&self, other: &Self) -> Self::Output;
}

impl<T> Metrized for T
where
    T: Signed,
{
    type Output = Self;
    fn distance(&self, other: &Self) -> Self {
        self.abs_sub(other).abs()
    }
}

pub trait Halfable {
    fn half(&mut self);
}

impl Halfable for f64 {
    fn half(&mut self) {
        *self /= 2.0;
    }
}

impl Halfable for i32 {
    fn half(&mut self) {
        *self /= 2;
    }
}