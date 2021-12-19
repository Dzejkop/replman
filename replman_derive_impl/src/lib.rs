use help_method::derive_help_method;
use parse_method::derive_parse_method;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::enum_attributes::EnumAttributes;

mod common;
mod enum_attributes;
mod field_attributes;
mod help_method;
mod parse_method;
mod variant_attributes;

pub fn derive_repl_cmd_impl(input: DeriveInput) -> TokenStream {
    let ty = &input.ident;

    let attrs = EnumAttributes::extract(&input.attrs);

    let data_enum = match &input.data {
        syn::Data::Enum(data_enum) => data_enum,
        _ => panic!("Can only derive ReplCmd for enums"),
    };

    let help_impl = derive_help_method(data_enum, &attrs);
    let parse_impl = derive_parse_method(data_enum, &attrs);

    let output = quote! {
        impl ReplCmd for #ty {
            #help_impl
            #parse_impl
        }
    };

    output
}
