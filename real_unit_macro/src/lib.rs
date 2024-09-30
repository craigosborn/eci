use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitFloat};

#[proc_macro]
pub fn ru64(input: TokenStream) -> TokenStream {
    let lit_float = parse_macro_input!(input as LitFloat);
    let value = lit_float.base10_parse::<f64>().unwrap();

    assert!(
        value >= 0.0 && value<= 1.0,
        "Value {lit_float} must be on the interval [0.0, 1.0]"
    );

    let expanded = quote! {
        RealUnit64::new(#value).unwrap()
    };

    TokenStream::from(expanded)
}