use std::marker::PhantomData;

use num_traits::Signed;

pub trait Reciprocal<O> {
    fn rcp(&self) -> O;
    fn invs(self) -> O;
}

macro_rules! impl_for_int {
    ($tp:ty) => {
        impl Reciprocal<f64> for $tp {
            fn rcp(&self) -> f64 {
                self.invs()
            }

            fn invs(self) -> f64 {
                1.0f64 / (self as f64)
            }
        }
    };
}

impl_for_int! {usize}
impl_for_int! {u8}
impl_for_int! {u16}
impl_for_int! {u32}
impl_for_int! {u64}
impl_for_int! {u128}

impl_for_int! {isize}
impl_for_int! {i8}
impl_for_int! {i16}
impl_for_int! {i32}
impl_for_int! {i64}
impl_for_int! {i128}

impl_for_int! {f32}
impl_for_int! {f64}

pub trait Reversible<X, Y> {
    fn fwd(&self, x: X) -> Y;
    fn bwd(&self, y: Y) -> X;
}

pub struct FnReversed<X, Y, Fwd, Bwd> {
    fwd_fn: Fwd,
    bwd_fn: Bwd,
    _ph: PhantomData<(X, Y)>,
}

pub fn fn_reversed<X, Y>(
    forward: impl Fn(X) -> Y,
    backward: impl Fn(Y) -> X,
) -> impl Reversible<X, Y> {
    FnReversed {
        fwd_fn: forward,
        bwd_fn: backward,
        _ph: PhantomData,
    }
}

impl<X, Y, Fwd, Bwd> Reversible<X, Y> for FnReversed<X, Y, Fwd, Bwd>
where
    Fwd: Fn(X) -> Y,
    Bwd: Fn(Y) -> X,
{
    fn fwd(&self, x: X) -> Y {
        (self.fwd_fn)(x)
    }

    fn bwd(&self, y: Y) -> X {
        (self.bwd_fn)(y)
    }
}

impl<X, Y, Fwd, Bwd> Reversible<X, Y> for (Fwd, Bwd)
where
    Fwd: Fn(X) -> Y,
    Bwd: Fn(Y) -> X,
{
    fn fwd(&self, x: X) -> Y {
        self.0(x)
    }

    fn bwd(&self, y: Y) -> X {
        self.1(y)
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
