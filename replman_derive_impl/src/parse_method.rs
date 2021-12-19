use quote::quote;
use syn::{parse_quote, Arm, DataEnum, Fields};

use crate::common::effective_variant_name;
use crate::enum_attributes::EnumAttributes;
use crate::field_attributes::{self, FieldAttributes};
use crate::variant_attributes::VariantAttributes;

pub fn derive_parse_method(
    data_enum: &DataEnum,
    attrs: &EnumAttributes,
) -> syn::ImplItemMethod {
    let variant_matches = variant_matches(data_enum, attrs);

    let parse_impl: syn::ImplItemMethod = syn::parse_quote! {
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
    };

    parse_impl
}

fn variant_matches<'a>(
    data_enum: &'a syn::DataEnum,
    attrs: &'a EnumAttributes,
) -> impl Iterator<Item = Arm> + 'a {
    data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let variant_attributes = VariantAttributes::extract(&variant.attrs);
        let effective_variant_name = effective_variant_name(variant, attrs, &variant_attributes);
        // TODO: Handle aliases
        let effective_variant_name = effective_variant_name.main_name;

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
                    #effective_variant_name => {
                        Ok(Self::#variant_name {
                            #(#field_parses)*
                        })
                    }
                }
            }
            Fields::Unit => {
                parse_quote! {
                    #effective_variant_name => { Ok(Self::#variant_name) }
                }
            }
            Fields::Unnamed(unnamed) => {
                let field_parses = unnamed.unnamed.iter().map(|_| {
                    quote! {
                        ::replman::ReplCmdParse::parse(parts.next().transpose()?)?,
                    }
                });

                parse_quote! {
                    #effective_variant_name => {
                        Ok(Self::#variant_name(
                            #(#field_parses)*
                        ))
                    }
                }
            }
        }
    })
}
