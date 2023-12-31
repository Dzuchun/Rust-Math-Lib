use approx::assert_abs_diff_eq;
use gen_math_lib::mean::*;

#[test]
fn arith_1() {
    // arrange
    let arr: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];

    // act
    let res = arith::<_, _, f64>(arr);

    // assert
    assert_eq!(res, 3.0);
}

#[test]
fn arith_2() {
    // arrange
    let arr: [f64; 0] = [];

    // act
    let res = arith::<_, _, f64>(arr);

    // assert
    assert!(res.is_nan());
}

#[test]
fn harmonic_1() {
    // arrange
    let arr: [f64; 2] = [1.0, 4.0];

    // act
    let res = harmonic::<_, _, _, f64>(arr);

    // assert
    assert_eq!(res, 1.6)
}

#[test]
fn harmonic_2() {
    // arrange
    let arr: [f64; 2] = [1.0, 7.0];

    // act
    let res = harmonic::<_, _, _, f64>(arr);

    // assert
    assert_eq!(res, 1.75)
}

#[test]
fn geometric_1() {
    // arrange
    let arr: [f64; 2] = [2.0, 8.0];

    // act
    let res = geometric::<_, f64, _, f64>(arr);

    // assert
    assert_eq!(res, 4.0)
}

#[test]
fn geometric_2() {
    // arrange
    let arr: [f64; 3] = [4.0, 8.0, 54.0];

    // act
    let res = geometric::<_, f64, _, f64>(arr);

    // assert
    assert_abs_diff_eq!(res, 12.0, epsilon = 1E-4);
}

#[test]
fn general_1() {
    // arrange
    let arr: [f64; 8] = [1.0, 2.0, 5.0, -7.0, 15.0, 84.0, 100.0, 120.0];

    // act
    let res = general::<_, _, _, _, f64>((|v: f64| v.powi(2), f64::sqrt), arr);

    // assert
    assert!((res - 63.00793601).abs() < 1E-7);
}

#[test]
fn general_2() {
    // arrange
    let arr: [f64; 8] = [1.0, 2.0, 5.0, 7.0, 15.0, 84.0, 100.0, 120.0];

    // act
    let res = general::<_, _, _, _, f64>((|v: f64| v.powi(5), |v: f64| v.powf(0.2)), arr);

    // assert
    assert_abs_diff_eq!(res, 86.6447017925, epsilon = 1E-7);
}

#[test]
fn general_3() {
    // arrange
    let arr: [f64; 8] = [1.0, 2.0, 5.0, 7.0, 15.0, 84.0, 100.0, -120.0];

    // act
    let res = general::<_, _, _, _, f64>((|v: f64| v.powf(0.5), |v: f64| v.powi(2)), arr);

    // assert
    assert!(res.is_nan());
}

#[test]
fn general_4() {
    // arrange
    let arr: [f64; 0] = [];

    // act
    let res = general::<_, _, _, _, f64>((|v: f64| v.powf(77.0), |v: f64| v.powf(1.0 / 77.0)), arr);

    // assert
    assert!(res.is_nan());
}
