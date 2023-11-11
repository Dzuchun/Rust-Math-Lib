use gen_math_lib::mean::*;

#[test]
fn arith_1() {
    // arrange
    let arr: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];

    // act
    let res = arith(arr);

    // assert
    assert_eq!(res.unwrap(), 3.0);
}

#[test]
fn arith_2() {
    // arrange
    let arr: [f64; 0] = [];

    // act
    let res = arith(arr);

    // assert
    assert!(res.is_none());
}

#[test]
fn harmonic_1() {
    // arrange
    let arr: [f64; 2] = [1.0, 4.0];

    // act
    let res = harmonic(arr);

    // assert
    assert_eq!(res.unwrap(), 1.6)
}

#[test]
fn harmonic_2() {
    // arrange
    let arr: [f64; 2] = [1.0, 7.0];

    // act
    let res = harmonic(arr);

    // assert
    assert_eq!(res.unwrap(), 1.75)
}

#[test]
fn geometric_1() {
    // arrange
    let arr: [f64; 2] = [2.0, 8.0];

    // act
    let res = geometric::<_, f64, _, f64>(arr);

    // assert
    assert_eq!(res.unwrap(), 4.0)
}

#[test]
fn geometric_2() {
    // arrange
    let arr: [f64; 3] = [4.0, 8.0, 54.0];

    // act
    let res = geometric::<_, f64, _, f64>(arr);

    // assert
    assert!((res.unwrap() - 12.0).abs() < 1E-7);
}

#[test]
fn general_1() {
    // arrange
    let arr: [f64; 8] = [1.0, 2.0, 5.0, -7.0, 15.0, 84.0, 100.0, 120.0];

    // act
    let res = general((|v: f64| v.powi(2), f64::sqrt), arr);

    // assert
    assert!((res.unwrap() - 63.00793601).abs() < 1E-7);
}

#[test]
fn general_2() {
    // arrange
    let arr: [f64; 8] = [1.0, 2.0, 5.0, 7.0, 15.0, 84.0, 100.0, 120.0];

    // act
    let res = general((|v: f64| v.powi(5), |v: f64| v.powf(0.2)), arr);

    // assert
    assert!((res.unwrap() - 26.63671933).abs() < 1E-7);
}

#[test]
fn general_3() {
    // arrange
    let arr: [f64; 8] = [1.0, 2.0, 5.0, 7.0, 15.0, 84.0, 100.0, -120.0];

    // act
    let res = general((|v: f64| v.powf(0.5), |v: f64| v.powi(2)), arr);

    // assert
    assert!(res.is_none());
}

#[test]
fn general_4() {
    // arrange
    let arr: [f64; 0] = [];

    // act
    let res = general((|v: f64| v.powf(77.0), |v: f64| v.powf(1.0 / 77.0)), arr);

    // assert
    assert!(res.is_none());
}
