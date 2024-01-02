extern crate proc_macro;

use chrono::prelude::*;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Log)]
pub fn log_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let trait_impl = quote! {
      impl Log for #name {
        fn info(&self, msg: &str) {
          println!("[INFO] {}: {}", stringify!(#name), msg);
        }
        fn warn(&self, msg: &str) {
          println!("[WARN] {}: {}", stringify!(#name), msg);
        }
        fn error(&self, msg: &str) {
          println!("[ERROR] {}: {}", stringify!(#name), msg);
        }
      }
    };

    trait_impl.into()
}

#[proc_macro]
pub fn log_info(input: TokenStream) -> TokenStream {
    let mut output = "[INFO] ".to_string();

    for token in input {
        let token_string = token.to_string();

        match token_string.as_str() {
            "[TIME]" => {
                let time = Local::now().time().to_string();
                output.push_str(&format!("{} ", time));
            }
            _ => {
                output.push_str(&format!("{} ", token_string));
            }
        }
    }

    TokenStream::from(quote! {
      println!("{}", #output);
    })
}
