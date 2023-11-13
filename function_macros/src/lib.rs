extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse::Parse, parse_macro_input, LitInt, Token};

struct FFATArgs {
    terms: usize,
    coef_expression: TokenStream2,
    x_type: Ident,
}

impl Parse for FFATArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let x_type = input.parse()?;
        input.parse::<Token![,]>()?;
        let terms = input.parse::<LitInt>()?.base10_parse()?;
        input.parse::<Token![,]>()?;
        let coef_expression = input.parse()?;
        Ok(Self {
            terms,
            coef_expression,
            x_type,
        })
    }
}

fn coefficient(name: char, term: usize) -> Ident {
    Ident::new(&format!("{}_{}", name, term), Span::call_site())
}

#[proc_macro]
pub fn function_factored_absolute_tailor(input: TokenStream) -> TokenStream {
    let FFATArgs {
        terms,
        coef_expression,
        x_type,
    } = parse_macro_input!(input as FFATArgs);
    let mut n = terms - 1;
    let c = coefficient('a', n);
    let mut constants = quote! (
    let n = #n;
    let #c = #coef_expression;
    );
    let mut evaluation = quote!(#c);
    while n > 0 {
        n -= 1;
        let c = coefficient('a', n);
        evaluation = quote! (
            #c
            + (&x).clone() * (#evaluation)
        );
        constants = quote! (
        #constants
        let n = #n;
        let #c = #coef_expression;
        );
    }

    quote! {
    {
        #constants
        move |x: #x_type| #evaluation
    }
        }
    .into()
}

#[proc_macro]
pub fn function_factored_relative_tailor(item: TokenStream) -> TokenStream {
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

// TODO add multi-cycled expansions
