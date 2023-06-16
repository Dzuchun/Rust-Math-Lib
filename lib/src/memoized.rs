use std::collections::HashMap;

use crate::traits::Metrized;

pub struct Memoized<'a, E, F> {
    func: Box<dyn FnMut(E) -> F + 'a>,
}

impl<'a, E, F> std::ops::Deref for Memoized<'a, E, F>
where
    E: Metrized,
{
    type Target = dyn FnMut(E) -> F + 'a;

    fn deref(&self) -> &Self::Target {
        &*self.func
    }
}

impl<'a, E, F> std::ops::DerefMut for Memoized<'a, E, F>
where
    E: Metrized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.func
    }
}

use crate::matrix::{literal_from_data, Matrix};

// source: https://stackoverflow.com/a/39647997
#[derive(Debug, Copy, Clone)]
struct HashableF64(f64);

impl HashableF64 {
    fn key(&self) -> u64 {
        self.0.to_bits()
    }
}

impl std::hash::Hash for HashableF64 {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.key().hash(state)
    }
}

impl PartialEq for HashableF64 {
    fn eq(&self, other: &HashableF64) -> bool {
        self.key() == other.key()
    }
}

impl Eq for HashableF64 {}
impl Metrized for HashableF64 {
    fn distance(&self, other: &Self) -> f64 {
        (self.0 - other.0).abs()
    }
}

impl<'a> Memoized<'a, f64, f64> {
    pub fn from(func: &'a (dyn Fn(f64) -> f64), max_distance: f64) -> Memoized<'a, f64, f64> {
        let mut estimated_calls: HashMap<HashableF64, f64> = HashMap::new();
        let mut computed_calls: HashMap<HashableF64, f64> = HashMap::new();
        Memoized {
            func: Box::new(move |e: f64| {
                let e = HashableF64(e);
                if let Some(val) = estimated_calls.get(&e) {
                    return val.clone();
                }
                if let Some(val) = computed_calls.get(&e) {
                    return val.clone();
                }
                // function was nor computed, nor estimated for this argument.
                let mut close_calls = computed_calls
                    .iter()
                    .filter(|(k, _)| k.distance(&e) <= max_distance)
                    .collect::<Vec<(&HashableF64, &f64)>>();
                if close_calls.len() < 7 {
                    // at least 7 close calls needed to perform estimation
                    let computed = func(e.clone().0);
                    computed_calls.insert(e, computed.clone());
                    return computed;
                }
                // can estimate!
                close_calls.sort_by(|&(k1, _), &(k2, _)| {
                    k1.distance(&e).partial_cmp(&k2.distance(&e)).unwrap()
                });
                let close_calls: [(&HashableF64, &f64); 7] = close_calls[..7].try_into().unwrap();
                let main_matrix = literal_from_data::<7, 7, f64>(close_calls.map(|(k, _)| {
                    let dx = k.0 - e.0;
                    [
                        1.0,
                        dx,
                        dx.powi(2),
                        dx.powi(3),
                        dx.powi(4),
                        dx.powi(5),
                        dx.powi(6),
                    ]
                }));
                let main_det = main_matrix.det_recursion_all().unwrap();
                let aux_matrix = literal_from_data::<7, 7, f64>(close_calls.map(|(k, v)| {
                    let dx = k.0 - e.0;
                    [
                        v.clone(),
                        dx,
                        dx.powi(2),
                        dx.powi(3),
                        dx.powi(4),
                        dx.powi(5),
                        dx.powi(6),
                    ]
                }));
                let estimated = aux_matrix.det_recursion_all().unwrap() / main_det;
                estimated_calls.insert(e, estimated);
                estimated
            }),
        }
    }
}
