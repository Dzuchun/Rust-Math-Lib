use gen_math_lib::{general_functions::*, memoized};
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
