use gen_math_lib::numbers::*;

#[test]
fn pochhammer_1() {
    // arrange
    let n = 1;
    let k = 1;

    // act
    let res = pochhammer(&n, &k);

    // assert
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn pochhammer_2() {
    // arrange
    let n = 10;
    let k = 2;

    // act
    let res = pochhammer(&n, &k);

    // assert
    assert_eq!(res.unwrap(), 110);
}

#[test]
fn binomial_1() {
    // arrange
    let n = 1;
    let k = 1;

    // act
    let res = binomial(&n, &k);

    // assert
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn binomial_2() {
    // arrange
    let n = 2;
    let k = 1;

    // act
    let res = binomial(&n, &k);

    // assert
    assert_eq!(res.unwrap(), 2);
}

#[test]
fn binomial_3() {
    // arrange
    let n = 10;
    let k = 4;

    // act
    let res = binomial(&n, &k);

    // assert
    assert_eq!(res.unwrap(), 210);
}

#[test]
fn binomial_4() {
    // arrange
    let n = 49;
    let k = 24;

    // act
    let res = binomial(&n, &k);

    // assert
    assert_eq!(res.unwrap(), 63205303218876);
}

#[test]
fn binomial_5() {
    // arrange
    let n = 52;
    let k = 24;

    // act
    let res = binomial(&n, &k);

    // assert
    assert_eq!(res.unwrap(), 426384982032100);
}

#[test]
fn binomial_6() {
    // arrange
    let n = 64;
    let k = 30;

    // act
    let res = binomial(&n, &k);

    // assert
    assert!(res.is_none());
}
