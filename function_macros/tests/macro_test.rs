use function_macros::*;

#[test]
fn macro1() {
    let pow = 0.4;
    let lnp1 = factored_absolute_tailor!(f64, 30, {
        {
            if n == 0 {
                0.0
            } else {
                let r = 1.0 / (n as f64);
                if n % 2 == 0 {
                    -r
                } else {
                    r
                }
            }
        }
    });
    let res = lnp1(std::f64::consts::E.powf(pow) - 1.0) - pow;
    println!("{res}");
}

#[test]
fn macro2() {
    let e = factored_relative_multitailor!(f64, 30, 1.0; 1.0 / (n as f64))(1.0);
    println!("{}", e - std::f64::consts::E);
}
