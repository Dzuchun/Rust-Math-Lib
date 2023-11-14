pub fn pochhammer(n: &u128, k: &u128) -> Option<u128> {
    let mut result: u128 = 1;
    for p in *n..*n + *k {
        if result > u128::MAX / p {
            return None;
        }
        result *= p;
    }
    Some(result)
}

pub fn binomial(n: &u128, k: &u128) -> Option<u128> {
    if *k == 0 || k == n {
        return Some(1);
    }

    if *k == 1 || *k == *n - 1 {
        return Some(*n);
    }

    let k: u128 = if *k > *n / 2 { n - k } else { *k };
    Some(pochhammer(&(*n - k + 1), &k)? / pochhammer(&1, &k)?)
}
