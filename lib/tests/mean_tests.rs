use gen_math_lib::mean::*;

#[test]
fn arith_1() {
    // arrange
    let arr: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];

    // act
    let res = arith(&mut arr.into_iter());

    // assert
    assert_eq!(res.unwrap(), 3.0);
}

#[test]
fn arith_2() {
    // arrange
    let arr: [f64; 0] = [];

    // act
    let res = arith(&mut arr.into_iter());

    // assert
    assert!(res.is_none());
}

#[test]
fn harmonic_1() {
    // arrange
    let arr: [f64; 2] = [1.0, 4.0];

    // act
    let res = harmonic(&mut arr.into_iter());

    // assert
    assert_eq!(res.unwrap(), 1.6)
}

#[test]
fn harmonic_2() {
    // arrange
    let arr: [f64; 2] = [1.0, 7.0];

    // act
    let res = harmonic(&mut arr.into_iter());

    // assert
    assert_eq!(res.unwrap(), 1.75)
}

#[test]
fn geometric_1() {
    // arrange
    let arr: [f64; 2] = [2.0, 8.0];

    // act
    let res = geometric(&mut arr.into_iter());

    // assert
    assert_eq!(res.unwrap(), 4.0)
}

#[test]
fn geometric_2() {
    // arrange
    let arr: [f64; 3] = [4.0, 8.0, 54.0];

    // act
    let res = geometric(&mut arr.into_iter());

    // assert
    assert!((res.unwrap() - 12.0).abs() < 1E-7);
}

use gen_math_lib::traits::Reversible;
#[test]
fn general_1() {
    // arrange
    let arr: [f64; 8] = [1.0, 2.0, 5.0, -7.0, 15.0, 84.0, 100.0, 120.0];
    let general_func = init_general(Reversible::pow(2.0));

    // act
    let res = general_func(&mut arr.into_iter());

    // assert
    assert!((res.unwrap() - 63.00793601).abs() < 1E-7);
}

#[test]
fn general_2() {
    // arrange
    let arr: [f64; 8] = [1.0, 2.0, 5.0, 7.0, 15.0, 84.0, 100.0, 120.0];
    let general_func = init_general(Reversible::pow(0.5));

    // act
    let res = general_func(&mut arr.into_iter());

    // assert
    assert!((res.unwrap() - 26.63671933).abs() < 1E-7);
}

#[test]
fn general_3() {
    // arrange
    let arr: [f64; 8] = [1.0, 2.0, 5.0, 7.0, 15.0, 84.0, 100.0, -120.0];
    let general_func = init_general(Reversible::pow(0.5));

    // act
    let res = general_func(&mut arr.into_iter());

    // assert
    assert!(res.is_none());
}

#[test]
fn general_4() {
    // arrange
    let arr: [f64; 0] = [];
    let general_func = init_general(Reversible::pow(77.0));

    // act
    let res = general_func(&mut arr.into_iter());

    // assert
    assert!(res.is_none());
}
