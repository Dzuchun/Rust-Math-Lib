use gen_math_lib::matrix::*;

#[test]
fn matrix_transpose() {
    // arrange
    let m = literal_fill::<3, 4, _>(0.0f64);

    // act
    let tr = m.transposed();

    // assert
    let e = literal_fill::<4, 3, _>(0.0f64);
    assert_eq!(tr, e);
}
#[test]
fn matrix_generate() {
    // arrange

    // act
    let m = literal_compute::<2, 2, _>(|i, j| i as f64 - j as f64);

    // assert
    let e = literal_from_data([[0.0, -1.0], [1.0, 0.0]]);
    assert_eq!(m, e);
}

#[test]
fn matrix_det1() {
    // arrange
    let m = literal_fill::<3, 4, _>(1.0f64);

    // act
    let det = m.det_recursion_all();

    // assert
    assert!(det.is_err());
}

#[test]
fn matrix_det2() {
    // arrange
    let m = literal_fill::<0, 0, _>(1.0f64);

    // act
    let det = m.det_recursion_all();

    // assert
    assert!(det.is_err());
}

#[test]
fn matrix_det3() {
    // arrange
    let m = literal_fill::<3, 3, _>(10.0f64);

    // act
    let det = m.det_recursion_all();

    // assert
    assert_eq!(det.unwrap(), 0.0);
}

#[test]
fn matrix_det4() {
    // arrange
    let m = literal_from_data::<2, 2, _>([[1.0, -1.0], [1.0, 0.0]]);

    // act
    let det = m.det_recursion_all();

    // assert
    assert_eq!(det.unwrap(), 1.0);
}

#[test]
fn matrix_det5() {
    const NUMS: [f64; 5] = [1.0, 2.0, 3.0, 4.0, -5.0];
    // arrange
    let m = literal_compute::<5, 5, _>(|i, j| (NUMS[i] as f64).powi(j as i32));

    // act
    let det = m.det_recursion_all();

    // assert
    let det = det.unwrap();
    let expected: f64 = (0..5)
        .map(|i| (0..i).map(|j| NUMS[i] - NUMS[j]).product::<f64>())
        .product();
    assert_eq!(det, expected);
}
