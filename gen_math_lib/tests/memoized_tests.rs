use gen_math_lib::memoized::{self};

#[test]
fn memo_1() {
    // arrange
    let func = |x: f64| {
        println!("Called for {x}!");
        x.powf(3.53653) / (1.0 - x.exp())
    };
    let memoized = memoized::from_fn::<_, _, _, u8>(&func, 1.0);
    const ARGS: [f64; 10] = [1.1, 1.11, 1.12, 0.13, 0.14, 0.15, 0.16, 0.17, 0.18, 0.19];

    // act
    let optimized: [f64; 10] = ARGS
        .iter()
        .map(|arg| memoized(*arg))
        .collect::<Vec<f64>>()
        .try_into()
        .unwrap();
    println!("Optimized calls done");
    let non_optimized: [f64; 10] = ARGS
        .iter()
        .map(|arg| func(*arg))
        .collect::<Vec<f64>>()
        .try_into()
        .unwrap();
    println!("Non-optimized calls done");
    let diffs: [f64; 10] = optimized
        .iter()
        .zip(non_optimized)
        .map(|(a, b)| a - b)
        .collect::<Vec<f64>>()
        .try_into()
        .unwrap();

    // assert
    println!("opt={optimized:?},\nnopt={non_optimized:?},\ndiff={diffs:?}");
}
