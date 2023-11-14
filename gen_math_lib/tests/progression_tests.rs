use gen_math_lib::progression::*;

#[test]
fn arith_1() -> Result<(), String> {
    // arrange

    // act
    let it = arithmetic_bounded(0.0, 10.0, 1.0);

    // assert
    assert_eq!(
        it.collect::<Vec<f64>>(),
        vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
    );
    Ok(())
}

#[test]
fn arith_2() -> Result<(), String> {
    // arrange

    // act
    let it = arithmetic_bounded(0.0, 10.0, -1.0);

    // assert
    assert_eq!(it.count(), 0);
    Ok(())
}

#[test]
fn arith_3() -> Result<(), String> {
    // arrange

    // act
    let it = arithmetic_bounded(10.0, 0.0, -1.0);

    // assert
    let mut expected = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    expected.reverse();
    assert_eq!(it.collect::<Vec<f64>>(), expected);
    Ok(())
}

#[test]
fn arith_4() -> Result<(), String> {
    // arrange

    // act
    let it = arithmetic_bounded(0.0, 10.0, 0.0);

    // assert
    assert_eq!(it.count(), 0);
    Ok(())
}

#[test]
fn arith_5() -> Result<(), String> {
    // arrange

    // act
    let it = arithmetic_bounded(10.0, 10.0, 0.0);

    // assert
    assert_eq!(it.collect::<Vec<f64>>(), Vec::new());
    Ok(())
}

#[test]
fn geometric_1() -> Result<(), String> {
    // arrange

    // act
    let it = geometric(1.0, 2.0);

    // assert
    assert_eq!(
        it.take(10).collect::<Vec<f64>>(),
        vec![1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0, 256.0, 512.0]
    );
    Ok(())
}

#[test]
fn geometric_3() -> Result<(), String> {
    // arrange

    // act
    let it = geometric(1.0, -2.0);

    // assert
    assert_eq!(
        it.take(10).collect::<Vec<f64>>(),
        vec![1.0, -2.0, 4.0, -8.0, 16.0, -32.0, 64.0, -128.0, 256.0, -512.0]
    );
    Ok(())
}

#[test]
fn geometric_6() -> Result<(), String> {
    // arrange

    // act
    let it = geometric(10.0, 0.0);

    // assert
    assert_eq!(it.take(3).collect::<Vec<f64>>(), vec![10.0, 0.0, 0.0]);
    Ok(())
}

#[test]
fn geometric_7() -> Result<(), String> {
    // arrange

    // act
    let it = geometric(0.0, 0.0);

    // assert
    assert_eq!(it.take(2).collect::<Vec<f64>>(), vec![0.0, 0.0]);
    Ok(())
}
