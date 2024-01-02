extern crate proc_macro;

use chrono::prelude::*;
use proc_macro::TokenStream;
use quote::quote;

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
