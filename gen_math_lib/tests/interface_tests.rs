use gen_math_lib::traits::*;

#[test]
fn sequential_1() {
  // arrange
  let mut a = 1;

  // act
  a.inc();

  // assert
  assert_eq!(a, 2);
}
