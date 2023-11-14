use gen_math_lib::macro_functions::*;
use gen_math_lib::progression::arithmetic_bounded;
use gen_math_lib::traits::Metrized;

#[test]
fn exp_1() {
    // arrange
    let a = 1.0;

    // act
    let res = exp()(a);

    assert!(res.distance(&std::f64::consts::E) < 1E-7);
}

#[test]
fn exp_2() {
    // arrange
    let a = 0.0;

    // act
    let res = exp()(a);

    assert!(res.distance(&1.0) < 1E-7);
}

#[test]
fn exp_3() {
    // arrange
    let a = -1.0;

    // act
    let res = exp()(a);

    assert!(res.distance(&(1.0 / std::f64::consts::E)) < 1E-7);
}

#[test]
fn exp_4() {
    // arrange
    let a = -100.0;

    // act
    let res = exp()(a);

    assert!(res.distance(&0.0) < 1E-7, "{res}");
}

#[test]
fn ln_1() {
    // arrange
    let a = 1.0;

    // act
    let res = ln()(a);

    // assert
    let res = res.unwrap();
    assert!(res.distance(&0.0) < 1E-7, "{res}");
}

#[test]
fn ln_2() {
    // arrange
    let a = 2.0;

    // act
    let res = ln()(a);

    // assert
    let res = res.unwrap();
    let expected = 2.0f64.ln();
    assert!(res.distance(&expected) < 1E-7, "{res} -- {expected}");
}

#[test]
fn ln_3() {
    // arrange
    let a = -1.0;

    // act
    let res = ln()(a);

    // assert
    assert!(res.is_none())
}

#[test]
fn sin_1() -> Result<(), String> {
    use std::f64::consts::TAU;

    // arrange
    let a = arithmetic_bounded(-TAU, TAU, 0.1).collect::<Vec<f64>>();

    // act
    let res = a.iter().map(|x| sin()(*x)).collect::<Vec<f64>>();
    let expected = a
        .into_iter()
        .map(|x| (x, x.sin()))
        .collect::<Vec<(f64, f64)>>();

    // assert
    let deviation = res
        .into_iter()
        .zip(expected.into_iter())
        .map(|(r, (_, e))| r.distance(&e))
        .reduce(f64::max)
        .unwrap();
    assert!(deviation < 1E-10, "{deviation}");
    Ok(())
}

#[test]
fn cos_1() -> Result<(), String> {
    use std::f64::consts::TAU;

    // arrange
    let a = arithmetic_bounded(-TAU, TAU, 0.1).collect::<Vec<f64>>();

    // act
    let res = a.iter().map(|x| cos()(*x)).collect::<Vec<f64>>();
    let expected = a
        .into_iter()
        .map(|x| (x, x.cos()))
        .collect::<Vec<(f64, f64)>>();

    // assert
    let deviation = res
        .into_iter()
        .zip(expected.into_iter())
        .map(|(r, (_, e))| r.distance(&e))
        .reduce(f64::max)
        .unwrap();
    assert!(deviation < 1E-10, "{deviation}");
    Ok(())
}

#[test]
fn sinc_1() {
    use gen_math_lib::integration;

    // arrange

    // act
    let res = integration::euler(0.0, 1000.0, 1E-7, 1E-4, sinc());

    // assert
    let deviation = res.distance(&std::f64::consts::FRAC_PI_2) / std::f64::consts::FRAC_PI_2;
    assert!(deviation < 1E-3, "{deviation}");
}
