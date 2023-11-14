use gen_math_lib::{general_functions::*, memoized, progression::arithmetic_bounded};
use rand::Rng;

#[test]
#[ignore = "used in manual mode only"]
fn e1_test() {
    // arrange
    let func = memoized::from_fn::<_, _, _, u8>(e1(), 0.1);
    let mut rng = rand::thread_rng();

    // act
    for _ in 0..200 {
        let arg = rng.gen_range(0.0..1.0);
        println!("{arg} {}", func(arg));
    }

    // assert
    assert!(true)
}

#[test]
#[ignore = "used in manual mode only"]
fn ei_test() {
    // arrange
    let ei_ = ei();
    let func = |x| ei_(x).unwrap_or(0.0);
    let func = memoized::from_fn::<_, _, _, u8>(&func, 0.1);
    let mut rng = rand::thread_rng();

    // act
    for _ in 0..200 {
        let arg = rng.gen_range(0.0..1.6);
        println!("{arg} {}", func(arg));
    }

    // assert
    assert!(false)
}

#[test]
#[ignore = "used in manual mode only"]
fn li_test() {
    // arrange
    let li_ = li();
    let func = |x| li_(x).unwrap_or(0.0);
    let func = memoized::from_fn::<_, _, _, u8>(&func, 0.05);
    let mut rng = rand::thread_rng();

    // act
    for _ in 0..200 {
        let arg = rng.gen_range(1.0..2.0);
        println!("{arg} {}", func(arg));
    }

    // assert
    assert!(true)
}

#[test]
#[ignore = "used in manual mode only"]
fn hypergeometric_test() {
    // arrange
    let exp = hypergeometric(-std::f64::consts::E, 0.1, 0.1); // this should be (1-z)^e
    let hyp = hypergeometric(1.0, 1.0, 2.0);
    let ln = |x: f64| x * hyp(-x); // this should be ln(1+z)

    // act
    println!("exp:");
    println!("|x|should|real|");
    arithmetic_bounded(-0.5f64, 0.5, 0.01)
        .map(|x| (x, (1.0 - x).powf(std::f64::consts::E), exp(x)))
        .for_each(|(x, should, real)| println!("{x} {should} {real}"));

    println!("ln:");
    println!("|x|should|real|");
    arithmetic_bounded(-0.5f64, 0.5, 0.01)
        .map(|x| (x, (1.0 + x).ln(), ln(x)))
        .for_each(|(x, should, real)| println!("{x} {should} {real}"));
}

#[test]
#[ignore = "used in manual mode only"]
fn asin_as_hg() {
    // arrange
    let hg = hypergeometric(0.5, 0.5, 1.5);
    let asin = |x: f64| x * hg(x * x);

    // act
    arithmetic_bounded(-1.0, 1.0, 0.01)
        .map(|x| (x, asin(x)))
        .for_each(|(x, y)| println!("{x} {y}"));
}

#[test]
#[ignore = "used in manual mode only"]
fn matrix_hg() {
    // arrange
    let hg = hypergeometric(0.5, 0.5, 1.5);
    let matrix = nalgebra::matrix![
        0.0,1.0;
        1.0,0.0;
    ];

    // act
    let res = hg(matrix);

    // assert
    println!("{res:?}");
}
