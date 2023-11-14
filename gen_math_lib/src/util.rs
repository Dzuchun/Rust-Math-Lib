use std::marker::PhantomData;

pub struct FnIter<T, F> {
    f: F,
    _ph: PhantomData<T>,
}

impl<T, F> From<F> for FnIter<T, F> {
    fn from(value: F) -> Self {
        Self {
            f: value,
            _ph: PhantomData,
        }
    }
}

impl<T, F> Iterator for FnIter<T, F>
where
    F: FnMut() -> Option<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        (self.f)()
    }
}
