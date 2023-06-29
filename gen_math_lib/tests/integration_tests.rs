use gen_math_lib::integration::*;

#[test]
fn euler_1() {
    // arrange
    let begin: f64 = 0.0;
    let end: f64 = std::f64::consts::PI;
    let func = f64::sin;

    // act
    let res: f64 = euler(begin, end, func, 1E-5, 1E-4);

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
    let res: f64 = euler(begin, end, func, 1E-9, 1e-5);

    // assert
    assert!((res - -1.0).abs() < 1E-7, "{res}");
}
