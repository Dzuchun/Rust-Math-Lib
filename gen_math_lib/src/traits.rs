pub trait Sequential {
    fn inc(&mut self);
    fn dec(&mut self);
}

impl Sequential for i32 {
    fn inc(&mut self) {
        *self += 1;
    }

    fn dec(&mut self) {
        *self -= 1;
    }
}

pub trait Recipical {
    type Rec;
    fn recipical(&self) -> Option<Self::Rec>;
}

impl Recipical for i32 {
    type Rec = f64;
    fn recipical(&self) -> Option<f64> {
        if *self == 0 {
            return None;
        } else {
            return Some(1.0 / (*self as f64));
        }
    }
}

pub struct Reversible<T, U> {
    pub call_box: Box<dyn Fn(T) -> Option<U>>,
    pub rev_box: Box<dyn Fn(U) -> Option<T>>,
}

impl<T, U> Reversible<T, U> {
    pub fn call(&self, arg: T) -> Option<U> {
        (*self.call_box)(arg)
    }

    pub fn rev(&self, arg: U) -> Option<T> {
        (*self.rev_box)(arg)
    }
}

impl Reversible<f64, f64> {
    pub fn pow(e: f64) -> Reversible<f64, f64> {
        let r = 1.0 / e;
        return Reversible {
            call_box: Box::new(move |x| {
                let res = x.powf(e);
                if !res.is_nan() {
                    Some(res)
                } else {
                    None
                }
            }),
            rev_box: Box::new(move |y| {
                let res = y.powf(r);
                if !res.is_nan() {
                    Some(res)
                } else {
                    None
                }
            }),
        };
    }
}

pub trait Zero {
    fn zero() -> &'static Self;
}

impl Zero for i32 {
    fn zero() -> &'static Self {
        const R: i32 = 0;
        &R
    }
}
impl Zero for f64 {
    fn zero() -> &'static Self {
        const R: f64 = 0.0;
        &R
    }
}

pub trait Metrized {
    fn distance(&self, other: &Self) -> f64;
}

impl Metrized for i32 {
    fn distance(&self, other: &i32) -> f64 {
        (*self - *other).abs() as f64
    }
}

impl Metrized for f64 {
    fn distance(&self, other: &f64) -> f64 {
        (*self - *other).abs()
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
