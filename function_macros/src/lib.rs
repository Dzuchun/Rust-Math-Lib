extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn ffat(item: TokenStream) -> TokenStream {
    let input = item.to_string();
    let comma_index = input.find(',').unwrap();
    let nomes = input[..comma_index].parse::<u128>().unwrap();
    let expr = input[(comma_index + 1)..].to_string();
    let mut res = String::from("{\n");
    for k in 0..=nomes {
        let copy = expr.clone().replace("^", &format!("{k}.0"));
        res.push_str(&format!("const a{k}: f64 = {copy};\n"));
    }
    res.push_str("|x| { a0 + ");
    for k in 1..=nomes {
        res.push_str(&format!("x * (a{k} +"));
    }
    res.push_str("0.0");
    for _ in 1..=nomes {
        res.push_str(")");
    }
    res.push_str("}\n}");
    res.parse().unwrap()
}

#[proc_macro]
pub fn ffrt(item: TokenStream) -> TokenStream {
    let input = item.to_string();
    let comma_index = input.find(',').unwrap();
    let nomes = input[..comma_index].parse::<u128>().unwrap();
    let expr = input[(comma_index + 1)..].to_string();
    let mut res = String::from("{\n");
    res.push_str("|x| {\n");
    let copy = expr.clone().replace("^", &format!("0.0"));
    res.push_str(&format!("let mut nome: f64 = {copy};\n"));
    res.push_str("let mut res: f64 = nome;\n");
    for k in 1..=nomes {
        let copy = expr.clone().replace("^", &format!("{k}.0"));
        res.push_str(&format!("nome *= {copy}*x;\n"));
        res.push_str("res += nome;\n");
    }
    res.push_str("res\n}\n}");
    res.parse().unwrap()
}

#[proc_macro]
pub fn fffbt(item: TokenStream) -> TokenStream {
    let input = item.to_string();
    let comma_index = input.find(',').unwrap();
    let cycles = input[..comma_index].parse::<usize>().unwrap();
    let expressions = input[(comma_index + 1)..].split(",").collect::<Vec<&str>>();
    let exprs = expressions.len() as usize;
    let mut res = String::from("{\n");
    res.push_str("|x| {\n");
    res.push_str("let mut xp: f64 = 1.0;\n");
    res.push_str("let mut res: f64 = 0.0;\n");
    for (i, expr) in expressions.iter().enumerate() {
        let copy = expr.clone().replace("^", &format!("{i}.0"));
        res.push_str(&format!("let mut nome{i}: f64 = {copy} * xp;\n"));
        res.push_str(&format!("res += nome{i};\n"));
        res.push_str(&format!("xp *= x;\n"));
    }
    for cycle in 1..=cycles {
        for (i, expr) in expressions.iter().enumerate() {
            let copy = expr
                .clone()
                .replace("^", &format!("{}.0", i + exprs * cycle));
            res.push_str(&format!("nome{i} *= {copy} * xp;\n"));
            res.push_str(&format!("res += nome{i};\n"));
        }
    }
    res.push_str("res\n}\n}");
    res.parse().unwrap()
}
