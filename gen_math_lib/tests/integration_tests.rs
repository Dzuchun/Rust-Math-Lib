use gen_math_lib::{
    general_functions::hypergeometric,
    integration::{self, *},
    progression::arithmetic_bounded,
};
use nalgebra::matrix;

#[test]
fn euler_1() {
    // arrange
    let begin: f64 = 0.0;
    let end: f64 = std::f64::consts::PI;
    let func = f64::sin;

    // act
    let res: f64 = euler(begin, end, 1E-5, 1E-4, func);

    // assert
    assert!((res - 2.0).abs() < 1E-8, "{res}");
}

#[test]
fn euler_2() {
    // arrange
    let begin: f64 = std::f64::consts::FRAC_PI_2;
    let end: f64 = 0.0;
    let func = f64::sin;

    // act
    let res: f64 = euler(begin, end, 1E-9, 1e-5, func);

    // assert
    assert!((res - -1.0).abs() < 1E-7, "{res}");
}
#[test]
fn kutta_1() {
    // arrange
    let begin: f64 = 0.0;
    let end: f64 = std::f64::consts::FRAC_PI_2;
    let deriv = |_: f64, x: f64| x.sin();
    let step = 1E-5;

    // act
    let (res, _) = integrate(
        0.0,
        begin,
        deriv,
        arithmetic_bounded(begin + step, end, step),
        integration::rk4_step,
    )
    .last()
    .unwrap();

    // assert
    assert!((res - 1.0).abs() < 1E-5, "{res}",);
}

#[test]
fn kutta_2() {
    // arrange
    let begin: f64 = 0.0;
    let end: f64 = std::f64::consts::FRAC_PI_2;
    let deriv = |_: f64, x: f64| x.sin();
    let step = 1E-5;

    // act
    let (res, _) = integrate(
        0.0,
        begin,
        deriv,
        arithmetic_bounded(begin + step, end, step),
        integration::rk7_step,
    )
    .last()
    .unwrap();

    // assert
    assert!((res - 1.0).abs() < 1E-5, "{res}",);
}

#[test]
fn kutta_3() {
    // arrange
    let fun = hypergeometric(0.5, 0.5, -0.5);
    let begin = matrix![
        0.0, 1.0;
        -1.0,0.0
    ];
    let start = 0.0;
    let end: f64 = std::f64::consts::FRAC_PI_8;
    let deriv = |x, _| fun(x);
    let step = 1E-2;

    // act
    for (x, t) in integrate(
        begin,
        start,
        deriv,
        arithmetic_bounded(start + step, end, step),
        integration::rk7_step,
    ) {
        println!("x={x}\nt={t}");
    }

    // assert
}
