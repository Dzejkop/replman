use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Replman, attributes(replman))]
pub fn derive_repl_cmd(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    replman_derive_impl::derive_repl_cmd_impl(input).into()
}
