use gen_math_lib::general_functions::*;
use gen_math_lib::memoized::Memoized;
use rand::Rng;

#[test]
#[ignore = "used in manual mode only"]
fn e1_test() {
    // arrange
    let mut func = Memoized::from(&E1, 0.1);
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
    let func = |x| *EI(x).get_or_insert(0.0);
    let mut func = Memoized::from(&func, 0.1);
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
    let func = |x| *LI(x).get_or_insert(0.0);
    let mut func = Memoized::from(&func, 0.05);
    let mut rng = rand::thread_rng();

    // act
    for _ in 0..200 {
        let arg = rng.gen_range(1.0..2.0);
        println!("{arg} {}", func(arg));
    }

    // assert
    assert!(true)
}
