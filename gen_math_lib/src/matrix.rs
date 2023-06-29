use std::ops::{Add, Index, IndexMut, Mul, Neg};

pub trait MatrixElement:
    Add<Output = Self>
    + Mul<Self, Output = Self>
    + Neg<Output = Self>
    + Sized
    + Clone
    + PartialEq<Self>
    + Default
{
}

impl MatrixElement for i32 {}
impl MatrixElement for f64 {}
pub struct Binumerator<'b, const M: usize, const N: usize, E> {
    i: usize,
    j: usize,
    data_source: Box<dyn Fn((usize, usize)) -> &'b E + 'b>,
}

// WOW, I'M IN LOVE WITH RUST
// I'VE JUST HAD SEEMIGLY OWFUL PROBLEM HERE,
// BUT ONCE I GAVE UP AND STARTED IMPLEMENTING IT IN A BAD WAY,
// RUST STOPPED ME AND GUIDED TOWARDS THIS

// HOPE IT'S GOOD ENOUGH
impl<'b, const M: usize, const N: usize, E: 'b> Iterator for Binumerator<'b, M, N, E> {
    type Item = (usize, usize, &'b E);
    fn next(&mut self) -> Option<(usize, usize, &'b E)> {
        if self.j >= N {
            self.j = 0;
            self.i += 1;
        }
        if self.i >= M {
            None
        } else {
            Some((self.i, self.j, (self.data_source)((self.i, self.j))))
        }
    }
}
// still wasn't able to implement mutable iterator tho

pub trait Matrix<'a, const M: usize, const N: usize, E: MatrixElement + 'a>:
    IndexMut<(usize, usize), Output = E> + PartialEq<Self> + 'a
{
    type Transposed;
    fn transposed(&self) -> Self::Transposed;

    // I'VE SPENT, LIKE 3 HOUR IMPLEMENTING COL AND ROW ITERATORS,
    // BUT NO LUCK -- LIFETIMES RUIN IT ALL, FOR SOME REASON.
    // HOW ARE YOU SUPPOSED TO CREATE MUTEABLE ITERATOR, THEN..?

    fn det_recursion(&self, rows: &mut Vec<usize>, cols: &mut Vec<usize>) -> Result<E, String> {
        if rows.len() != cols.len() {
            return Err(String::from(
                "Cannot compute determinant of a non-square matrix",
            ));
        }
        if rows.len() == 0 {
            return Err(String::from(
                "Cannot compute determinant of an empty matrix",
            ));
        }
        if rows.len() == 1 {
            return Ok(self[(*rows.iter().last().unwrap(), *cols.iter().last().unwrap())].clone());
        }
        let mut res = E::default();
        let i = rows.pop().unwrap();
        let mut neg = false;
        for ind in 0..cols.len() {
            let j = cols.remove(ind);
            let nome = self[(i, j)].clone() * self.det_recursion(rows, cols)?;
            if neg {
                res = res + (-nome);
            } else {
                res = res + nome;
            }
            cols.insert(ind, j);
            neg = !neg;
        }
        rows.push(i);
        Ok(res)
    }

    fn det_recursion_all(&self) -> Result<E, String> {
        self.det_recursion(
            &mut (0..M).rev().collect::<Vec<usize>>(),
            &mut (0..N).collect::<Vec<usize>>(),
        )
    }

    fn binumerate(&'a self) -> Binumerator<'a, M, N, E> {
        Binumerator::<'a, M, N, E> {
            i: 0,
            j: 0,
            data_source: Box::new(|t: (usize, usize)| &self[t]),
        }
    }
    fn eq(&'a self, other: &'a Self) -> bool {
        self.binumerate()
            .zip(other.binumerate())
            .all(|((_, _, e1), (_, _, e2))| e1 == e2)
    }

    fn ne(&'a self, other: &'a Self) -> bool {
        self.binumerate()
            .zip(other.binumerate())
            .any(|((_, _, e1), (_, _, e2))| e1 != e2)
    }
}

#[derive(Clone, Debug)]
pub struct LiteralMatrix<const M: usize, const N: usize, E: MatrixElement> {
    data: [[E; N]; M],
}

impl<const M: usize, const N: usize, E: MatrixElement> Index<(usize, usize)>
    for LiteralMatrix<M, N, E>
{
    type Output = E;
    fn index(&self, (i, j): (usize, usize)) -> &E {
        &self.data[i][j]
    }
}

impl<const M: usize, const N: usize, E: MatrixElement> IndexMut<(usize, usize)>
    for LiteralMatrix<M, N, E>
{
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut E {
        &mut self.data[i][j]
    }
}

impl<const M: usize, const N: usize, E: MatrixElement> PartialEq<LiteralMatrix<M, N, E>>
    for LiteralMatrix<M, N, E>
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<'a, const M: usize, const N: usize, E: MatrixElement + 'a> Matrix<'a, M, N, E>
    for LiteralMatrix<M, N, E>
{
    type Transposed = LiteralMatrix<N, M, E>;
    fn transposed(&self) -> Self::Transposed {
        unsafe {
            LiteralMatrix::<N, M, E> {
                data: (0..N)
                    .map(|j| {
                        (0..M)
                            .map(|i| self[(i, j)].clone())
                            .collect::<Vec<E>>()
                            .try_into()
                            .unwrap_unchecked()
                    })
                    .collect::<Vec<[E; M]>>()
                    .try_into()
                    .unwrap_unchecked(),
            }
        }
    }
}

pub fn literal_fill<const M: usize, const N: usize, E>(el: E) -> LiteralMatrix<M, N, E>
where
    E: MatrixElement + Copy,
{
    LiteralMatrix::<M, N, E> { data: [[el; N]; M] }
}

pub fn literal_compute<const M: usize, const N: usize, E>(
    gen: fn(usize, usize) -> E,
) -> LiteralMatrix<M, N, E>
where
    E: MatrixElement,
{
    unsafe {
        LiteralMatrix::<M, N, E> {
            data: (0..M)
                .map(|i| {
                    (0..N)
                        .map(|j| gen(i, j))
                        .collect::<Vec<E>>()
                        .try_into()
                        .unwrap_unchecked()
                })
                .collect::<Vec<[E; N]>>()
                .try_into()
                .unwrap_unchecked(),
        }
    }
}

pub fn literal_from_data<const M: usize, const N: usize, E>(
    data: [[E; N]; M],
) -> LiteralMatrix<M, N, E>
where
    E: MatrixElement,
{
    LiteralMatrix::<M, N, E> { data }
}
