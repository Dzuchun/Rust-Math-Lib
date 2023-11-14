use std::{cell::RefCell, cmp::Ordering, ops::Sub};

use nalgebra::{ComplexField, Scalar, U7};

use num_traits::{FromPrimitive, Pow};

use crate::traits::Metrized;

pub fn from_fn<X, F, D, P>(func: F, max_distance: D) -> impl Fn(X) -> X
where
    X: Scalar + Metrized<Output = D> + Sub<X, Output = X> + Pow<P, Output = X> + ComplexField,
    F: Fn(X) -> X,
    D: PartialOrd,
    P: FromPrimitive,
{
    let estimated_calls: RefCell<Vec<(X, X)>> = RefCell::default();
    let computed_calls: RefCell<Vec<(X, X)>> = RefCell::default();

    move |x: X| {
        for (px, py) in estimated_calls
            .borrow()
            .iter()
            .chain(computed_calls.borrow().iter())
        {
            if *px == x {
                return py.clone();
            }
        }
        // function was nor computed, nor estimated for this argument.
        let brw = computed_calls.borrow();
        let close_calls = brw
            .iter()
            .filter(|(k, _)| k.distance(&x) <= max_distance)
            .collect::<Vec<&(X, X)>>();
        if close_calls.len() < 7 {
            // at least 7 close calls needed to perform estimation
            let computed = func(x.clone());
            drop(brw);
            computed_calls.borrow_mut().push((x, computed.clone()));
            return computed;
        }
        // can estimate!
        let xi = x.clone();
        let mut close_calls: Vec<_> = close_calls
            .into_iter()
            .map(move |(cx, cy)| (cx.clone() - xi.clone(), cy.clone()))
            .collect();
        close_calls.sort_by(|(k1, _), (k2, _)| {
            k1.distance(&x)
                .partial_cmp(&k2.distance(&x))
                .unwrap_or(Ordering::Equal)
        });
        drop(brw);
        let close_calls: [(X, X); 7] = [
            close_calls[0].clone(),
            close_calls[1].clone(),
            close_calls[2].clone(),
            close_calls[3].clone(),
            close_calls[4].clone(),
            close_calls[5].clone(),
            close_calls[6].clone(),
        ];
        let main_matrix = nalgebra::Matrix::from_iterator_generic(
            U7,
            U7,
            close_calls[..7].iter().flat_map(move |(cx, _)| {
                let dx = cx.clone();
                std::array::from_fn::<X, 7, _>(move |i| {
                    dx.clone().pow(
                        P::from_usize(i)
                            .expect("Should be able to raise elements into usize-compatible power"),
                    )
                })
            }),
        );
        let main_det = main_matrix.determinant();
        let aux_matrix =
            nalgebra::Matrix::from_iterator_generic(
                U7,
                U7,
                close_calls[..7].iter().flat_map(move |(cx, cy)| {
                    let dx = cx.clone();
                    [
                        cy.clone(),
                        dx.clone(),
                        dx.clone().pow(P::from_usize(2).expect(
                            "Should be able to raise elements into usize-compatible power",
                        )),
                        dx.clone().pow(P::from_usize(3).expect(
                            "Should be able to raise elements into usize-compatible power",
                        )),
                        dx.clone().pow(P::from_usize(4).expect(
                            "Should be able to raise elements into usize-compatible power",
                        )),
                        dx.clone().pow(P::from_usize(5).expect(
                            "Should be able to raise elements into usize-compatible power",
                        )),
                        dx.pow(P::from_usize(6).expect(
                            "Should be able to raise elements into usize-compatible power",
                        )),
                    ]
                    .into_iter()
                }),
            );
        let aux_det = aux_matrix.determinant();

        let estimated = aux_det / main_det;
        estimated_calls.borrow_mut().push((x, estimated.clone()));
        estimated
    }
}
