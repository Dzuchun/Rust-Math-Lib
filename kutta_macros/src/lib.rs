use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{parse::Parse, parse_macro_input, Expr, LitFloat, LitInt, Token};

fn coefficient(name: char, index: usize) -> Ident {
    Ident::new(&format!("{}_{}", name, index), Span::call_site())
}

fn bi_coefficient(name: char, i: usize, j: usize) -> Ident {
    Ident::new(&format!("{}_{}_{}", name, i, j), Span::call_site())
}

enum Coefficient {
    Static(f64),
    Expression(Expr),
}

impl ToTokens for Coefficient {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Coefficient::Expression(expr) => expr.to_tokens(tokens),
            Coefficient::Static(val) => val.to_tokens(tokens),
        }
    }
}

// Notations were taken from the wikipage: https://en.wikipedia.org/wiki/Runge%E2%80%93Kutta_methods
struct KuttaArgs {
    name: Ident,
    order: usize,
    a: Vec<Coefficient>,
    b: Vec<Coefficient>,
    c: Vec<Coefficient>,
}

fn parse_num(input: syn::parse::ParseStream) -> syn::Result<f64> {
    Ok(if input.peek(LitInt) {
        f64::from(input.parse::<LitInt>()?.base10_parse::<i8>()?)
    } else {
        input.parse::<LitFloat>()?.base10_parse::<f64>()?
    })
}

fn parse_float(input: syn::parse::ParseStream) -> syn::Result<Coefficient> {
    if input.peek(Token![!]) {
        input.parse::<Token![!]>()?;
        Ok(Coefficient::Expression(input.parse()?))
    } else {
        let numerator = parse_num(input)?;
        let denominator = if input.peek(Token![/]) {
            input.parse::<Token![/]>()?;
            parse_num(input)?
        } else {
            1.0
        };
        Ok(Coefficient::Static(numerator / denominator))
    }
}

impl Parse for KuttaArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut order = 0usize;
        while !input.peek(Token![-]) {
            b.push(parse_float(input)?);
            input.parse::<Token![|]>()?;
            for _ in 0..order {
                a.push(parse_float(input)?);
            }
            order += 1;
        }
        while input.peek(Token![-]) {
            input.parse::<Token![-]>()?;
        }
        input.parse::<Token![|]>()?;

        let mut c = Vec::with_capacity(order);
        for _ in 0..order {
            c.push(parse_float(input)?);
        }

        Ok(Self {
            name,
            order,
            a,
            b,
            c,
        })
    }
}

#[proc_macro]
pub fn general_kutta_step(input: TokenStream) -> TokenStream {
    let KuttaArgs {
        name,
        order,
        a,
        b,
        c,
    } = parse_macro_input!(input as KuttaArgs);

    let mut a = a.into_iter(); // PTSSHHHHHhhhh....
    let mut b = b.into_iter();
    let mut c = c.into_iter();

    let mut constants = quote!();
    for i in 0..order {
        for j in 0..i {
            let a_expr = a.next().expect("This makes no sense, please contact me");
            let a_ident = bi_coefficient('A', i, j);
            constants = quote!(
                #constants
                static ref #a_ident: f64 = (#a_expr) as f64;
            );
        }
        let b_ident = coefficient('B', i);
        let b_expr = b.next().expect("This makes no sense, please contact me");
        let c_ident = coefficient('C', i);
        let c_expr = c.next().expect("This makes no sense, please contact me");
        constants = quote!(
            #constants
            static ref #b_ident: f64 = (#b_expr) as f64;
            static ref #c_ident: f64 = (#c_expr) as f64;
        );
    }

    constants = quote!(::lazy_static::lazy_static! {
        #constants
    });

    let mut k_ident;
    let mut eval = quote!();

    for i in 0..order {
        let dx = if i > 0 {
            let mut prev_k = coefficient('k', 0);
            let mut a_c = bi_coefficient('A', i, 0);
            let mut dx = quote!(<D as Clone>::clone(&#prev_k) * *#a_c);
            for j in 1..i {
                prev_k = coefficient('k', j);
                a_c = bi_coefficient('A', i, j);
                dx = quote!(#dx + <D as Clone>::clone(&#prev_k) * *#a_c);
            }
            quote!( + (#dx) * <T as Clone>::clone(&dt))
        } else {
            quote!()
        };
        let b = coefficient('B', i);
        k_ident = coefficient('k', i);
        constants = quote!(
          #constants
          let #k_ident: D = der(<X as Clone>::clone(&x0) #dx, <T as Clone>::clone(&t0) + <T as Clone>::clone(&dt) * *#b);
        );

        let c = coefficient('C', i);
        if i == 0 {
            eval = quote!(#k_ident * *#c);
        } else {
            eval = quote!(#eval + #k_ident * *#c);
        }
    }

    quote!(
        fn #name<X, T, D, Der>(x0: X, t0: T, dt: T, der: &Der) -> X
    where
      X: Clone + ::std::ops::Add<Output = X> + ::std::ops::Mul<f64, Output = X>,
      T: Clone + ::std::ops::Add<Output = T> + ::std::ops::Mul<f64, Output = T>,
      D: Clone + ::std::ops::Add<Output = D> + ::std::ops::Mul<f64, Output = D> + ::std::ops::Mul<T, Output = X>,
      Der: Fn(X, T) -> D,
    {
        #constants
        x0 + (#eval) * dt
    })
    .into()
}

// TODO make a feature-lock for lazy_static
