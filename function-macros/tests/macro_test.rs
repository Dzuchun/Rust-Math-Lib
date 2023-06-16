use function_macros::*;

#[test]
fn macro1() {
    let res = ffat!(30, if ^ == 0.0 {0.0} else { if ^ % 2.0 == 0.0 { 1.0 / ^ } else { -1.0 / ^ } })(
        1.0 / std::f64::consts::E - 1.0,
    );
    println!("{res}");
}

#[test]
fn macro2() {
    let res = ffrt!(30, if ^ == 0.0 {1.0} else { 1.0 / ^ })(1.0);
    println!("{res}");
}
