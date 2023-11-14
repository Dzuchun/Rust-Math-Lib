extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Expr, LitInt, Token};

struct TailorArgs {
    terms: usize,
    x_type: Ident,
    coef_expression: Expr,
}

impl Parse for TailorArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let x_type = input.parse()?;
        input.parse::<Token![,]>()?;
        let terms = input.parse::<LitInt>()?.base10_parse()?;
        input.parse::<Token![,]>()?;
        let coef_expression = input.parse()?;
        Ok(Self {
            terms,
            x_type,
            coef_expression,
        })
    }
}

fn coefficient(name: char, term: usize) -> Ident {
    Ident::new(&format!("{}_{}", name, term), Span::call_site())
}

fn evaluation(x_type: &Ident, terms: usize) -> TokenStream2 {
    let mut n = terms - 1;
    let mut c = coefficient('a', n);
    let mut evaluation = quote!(<#x_type as Clone>::clone(&#c));
    while n > 0 {
        n -= 1;
        c = coefficient('a', n);
        evaluation = quote! (
            <#x_type as Clone>::clone(&#c)
            + <#x_type as Clone>::clone(&x) * (#evaluation)
        );
    }
    evaluation
}

#[proc_macro]
pub fn factored_absolute_tailor(input: TokenStream) -> TokenStream {
    let TailorArgs {
        terms,
        coef_expression,
        x_type,
    } = parse_macro_input!(input as TailorArgs);
    let evaluation = evaluation(&x_type, terms);

    let mut c = coefficient('a', 0);
    let mut constants = quote! (
    let mut n = 0;
    let #c = #coef_expression;
    );
    for i in 1..terms {
        c = coefficient('a', i);
        constants = quote! (
        #constants
        n = #i;
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

struct MultiTailorArgs {
    terms: usize,
    x_type: Ident,
    first_expressions: Vec<Expr>,
    transitive_expressions: Vec<Expr>,
}

impl Parse for MultiTailorArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let x_type = input.parse()?;
        input.parse::<Token![,]>()?;
        let terms = input.parse::<LitInt>()?.base10_parse()?;
        let mut first_expressions = Vec::new();
        while input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            first_expressions.push(input.parse::<Expr>()?);
        }
        if first_expressions.is_empty() {
            return syn::Result::Err(syn::Error::new(
                input.span(),
                "Should provide expressions for transition factors",
            ));
        }
        input.parse::<Token![;]>()?;
        let mut transitive_expressions = Vec::with_capacity(first_expressions.len());
        for _ in 0..first_expressions.len() - 1 {
            transitive_expressions.push(input.parse()?);
            input.parse::<Token![,]>()?;
        }
        transitive_expressions.push(input.parse()?);

        Ok(Self {
            terms,
            x_type,
            first_expressions,
            transitive_expressions,
        })
    }
}

#[proc_macro]
pub fn factored_relative_multitailor(input: TokenStream) -> TokenStream {
    let MultiTailorArgs {
        terms,
        first_expressions,
        transitive_expressions,
        x_type,
    } = parse_macro_input!(input as MultiTailorArgs);
    let evaluation = evaluation(&x_type, terms);

    let constants = 'consts: {
        let period = first_expressions.len();
        let mut constants = quote!();
        let mut c;
        for (i, first_expression) in first_expressions.iter().enumerate() {
            if i >= terms {
                break 'consts constants;
            }
            c = coefficient('a', i);
            constants = quote!(
                #constants
                let #c = #first_expression;
            );
        }
        constants = quote!(
            #constants
            let mut n;
        );

        let mut i = 0;
        let mut prev_c;
        let mut transitive_expression;
        for n in period..terms {
            c = coefficient('a', n);
            prev_c = coefficient('a', n - period);
            transitive_expression = &transitive_expressions[i];
            constants = quote!(
                #constants
                n = #n;
                let #c = <#x_type as Clone>::clone(&#prev_c) * {#transitive_expression};
            );
            i += 1;
            if i == period {
                i = 0;
            }
        }
        constants
    };

    quote! {
    {
        #constants
        move |x: #x_type| #evaluation
    }
        }
    .into()
}

// TODO add multi-cycled expansions
