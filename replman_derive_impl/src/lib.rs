#![allow(clippy::useless_conversion)]

use convert_case::Casing;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Arm, DeriveInput, Fields};

use crate::enum_attributes::EnumAttributes;
use crate::field_attributes::FieldAttributes;

mod enum_attributes;
mod field_attributes;

pub fn derive_repl_cmd_impl(input: DeriveInput) -> TokenStream {
    let ty = &input.ident;

    let attrs = EnumAttributes::extract(&input.attrs);

    let data_enum = match input.data {
        syn::Data::Enum(data_enum) => data_enum,
        _ => panic!("Can only derive ReplCmd for enums"),
    };

    let variant_matches = variant_matches(&data_enum, &attrs);

    let output = quote! {
        impl ReplCmd for #ty {
            fn help() -> &'static str {
                ""
            }

            fn parse<'a, I>(mut parts: I) -> anyhow::Result<Self>
            where
                Self: Sized,
                I: Iterator<Item = anyhow::Result<&'a str>> + 'a
            {
                let cmd_word = parts.next().unwrap()?;

                match cmd_word {
                    #(#variant_matches)*
                    cmd => Err(anyhow::anyhow!("unrecognized command '{}'", cmd)),
                }
            }
        }
    };

    output.into()
}

fn variant_matches<'a>(
    data_enum: &'a syn::DataEnum,
    attrs: &'a EnumAttributes,
) -> impl Iterator<Item = Arm> + 'a {
    data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let mut variant_name_str = variant_name.to_string();
        if let Some(rename_all) = attrs.rename_all.as_ref() {
            variant_name_str = variant_name_str.to_case(*rename_all);
        }

        match &variant.fields {
            Fields::Named(named) => {
                let field_parses = named.named.iter().map(|field| {
                    let ident = field.ident.as_ref().unwrap();

                    let field_attributes = FieldAttributes::extract(&field.attrs);

                    match field_attributes.default {
                        field_attributes::FieldDefault::None => quote! {
                            #ident: ::replman::ReplCmdParse::parse(parts.next().transpose()?)?,
                        },
                        field_attributes::FieldDefault::Some(default_value) => quote! {
                            #ident: ::replman::ReplCmdParse::parse_default(parts.next().transpose()?.unwrap_or(#default_value))?,
                        },
                        field_attributes::FieldDefault::Default => quote! {
                            #ident: match parts.next() {
                                Some(s) => ::replman::ReplCmdParse::parse_default(s?)?,
                                None => Default::default(),
                            },
                        },
                    }
                });

                parse_quote! {
                    #variant_name_str => {
                        Ok(Self::#variant_name {
                            #(#field_parses)*
                        })
                    }
                }
            }
            Fields::Unit => {
                parse_quote! {
                    #variant_name_str => { Ok(Self::#variant_name) }
                }
            }
            Fields::Unnamed(unnamed) => {
                let field_parses = unnamed.unnamed.iter().map(|_| {
                    quote! {
                        ::replman::ReplCmdParse::parse(parts.next().transpose()?)?,
                    }
                });

                parse_quote! {
                    #variant_name_str => {
                        Ok(Self::#variant_name(
                            #(#field_parses)*
                        ))
                    }
                }
            }
        }
    })
}
