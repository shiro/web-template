extern crate proc_macro;

use proc_macro::TokenStream;
use std::env;
use std::path::Path;

use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::Token;

#[proc_macro_hack]
pub fn dotenv(input: TokenStream) -> TokenStream {
    let _ = dotenv::from_path(Path::new("../../.env.defaults"));
    let _ = dotenv::from_path(Path::new("../../.env"));

    let production = "production".to_string();
    match env::var("NODE_ENV") {
        Ok(production) => {
            let _ = dotenv::from_path(Path::new("../../.env.production.defaults"));
            let _ = dotenv::from_path(Path::new("../../.env.production"));
        }
        _ => {}
    }

    expand_env(input)
}

fn expand_env(input_raw: TokenStream) -> TokenStream {
    let args = <Punctuated<syn::LitStr, Token![,]>>::parse_terminated.parse(input_raw)
        .expect("expected macro to be called with a comma-separated list of string literals");

    let mut iter = args.iter();

    let var_name = match iter.next() {
        Some(s) => s.value(),
        None => panic!("expected 1 or 2 arguments, found none"),
    };

    let err_msg = match iter.next() {
        Some(lit) => lit.value(),
        None => format!("environment variable `{}` not defined", var_name).into(),
    };

    if iter.next().is_some() {
        panic!("expected 1 or 2 arguments, found 3 or more");
    }

    match env::var(var_name) {
        Ok(val) => quote!(#val).into(),
        Err(_) => panic!("{}", err_msg),
    }
}
