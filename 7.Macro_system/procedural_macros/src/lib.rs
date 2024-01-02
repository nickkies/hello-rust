extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn log_info(input: TokenStream) -> TokenStream {
    input
}
