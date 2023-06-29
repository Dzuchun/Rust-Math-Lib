use crate::traits::Zero;
use std::{
    iter::Iterator,
    ops::{AddAssign, Div, MulAssign, Sub},
};
pub trait ArithIteratorType:
    Sized
    + AddAssign
    + Sub<Output = Self>
    + Div<Output = Self>
    + Zero
    + PartialOrd<Self>
    + Copy
    + 'static
{
}

impl ArithIteratorType for f64 {}
impl ArithIteratorType for i32 {}

struct ArithIterator<T: ArithIteratorType> {
    current: T,
    end: T,
    step: T,
}

impl<T: ArithIteratorType> Iterator for ArithIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if (self.end - self.current) / self.step < *T::zero() {
            None
        } else {
            let tmp = self.current;
            self.current += self.step;
            Some(tmp)
        }
    }
}

pub fn arith<T: ArithIteratorType>(
    begin: T,
    end: T,
    step: T,
) -> Result<Box<dyn Iterator<Item = T>>, String> {
    if step == *T::zero() && begin == end {
        return Ok(Box::new([begin].into_iter()));
    }
    if step == *T::zero() || (end - begin) / step < *T::zero() {
        Err(String::from(format!(
            "Iterator must be able to reach <end> by adding <step> to <begin> value."
        )))
    } else {
        Ok(Box::new(ArithIterator {
            current: begin,
            end,
            step,
        }))
    }
}

use crate::traits::Metrized;
pub trait GeometricIteratorType:
    Sized + Div<Output = Self> + Metrized + Copy + MulAssign + Zero + PartialEq + 'static
{
}

impl GeometricIteratorType for i32 {}
impl GeometricIteratorType for f64 {}

struct GeometricIterator<T: GeometricIteratorType> {
    current: T,
    end: T,
    step: T,
}

impl<T: GeometricIteratorType> Iterator for GeometricIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.end / self.current).distance(&T::zero()).ln() / self.step.distance(&T::zero()).ln()
            < 0.0
        {
            None
        } else {
            let tmp: T = self.current;
            self.current *= self.step;
            Some(tmp)
        }
    }
}

pub fn geometric<T: GeometricIteratorType>(
    begin: T,
    end: T,
    denominator: T,
) -> Result<Box<dyn Iterator<Item = T>>, String> {
    if (denominator == *T::zero() && end == *T::zero())
        || (denominator.distance(T::zero()) == 1.0 && begin.distance(&end) == 0.0)
    {
        if begin == end {
            return Ok(Box::new([end].into_iter()));
        } else {
            return Ok(Box::new([begin, end].into_iter()));
        }
    }
    if (denominator == *T::zero())
        || (denominator.distance(T::zero()) == 1.0)
        || ((end / begin).distance(T::zero()).ln() / denominator.distance(T::zero()).ln() < 0.0)
    {
        Err(String::from(format!(
            "Iterator must be able to reach <end> by multiplying <step> by <begin> value."
        )))
    } else {
        Ok(Box::new(GeometricIterator {
            current: begin,
            end,
            step: denominator,
        }))
    }
}
