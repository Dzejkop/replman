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
    let VariantMatches {
        exact_matches,
        aliases,
        start_with_matches,
    } = variant_matches(data_enum, attrs);

    let parse_impl: syn::ImplItemMethod = syn::parse_quote! {
        fn parse<'a, I>(mut parts: I) -> anyhow::Result<Self>
        where
            Self: Sized,
            I: Iterator<Item = anyhow::Result<&'a str>> + 'a
        {
            let cmd_word = parts.next().unwrap()?;

            match cmd_word {
                #(#exact_matches)*
                #(#aliases)*
                #(#start_with_matches)*
                cmd => Err(anyhow::anyhow!("unrecognized command '{}'", cmd)),
            }
        }
    };

    parse_impl
}

#[derive(Default)]
struct VariantMatches {
    exact_matches: Vec<Arm>,
    aliases: Vec<Arm>,
    start_with_matches: Vec<Arm>,
}

fn variant_matches(
    data_enum: &syn::DataEnum,
    attrs: &EnumAttributes,
) -> VariantMatches {
    let mut variant_matches = VariantMatches::default();

    for variant in data_enum.variants.iter() {
        let variant_name = &variant.ident;
        let variant_attributes = VariantAttributes::extract(&variant.attrs);
        let effective_variant_name =
            effective_variant_name(variant, attrs, &variant_attributes);

        let main_name = &effective_variant_name.main_name;

        let variant_body: syn::Expr = match &variant.fields {
            Fields::Named(named) => {
                let field_parses = named.named.iter().map(|field| {
                    let ident = field.ident.as_ref().unwrap();

                    let field_attributes = FieldAttributes::extract(&field.attrs);

                    match field_attributes.default {
                        field_attributes::FieldDefault::None => quote! {
                            #ident: ::replman::ReplmanParse::parse(parts.next().transpose()?)?,
                        },
                        field_attributes::FieldDefault::Some(default_value) => quote! {
                            #ident: ::replman::ReplmanParse::parse_default(parts.next().transpose()?.unwrap_or(#default_value))?,
                        },
                        field_attributes::FieldDefault::Default => quote! {
                            #ident: match parts.next() {
                                Some(s) => ::replman::ReplmanParse::parse_default(s?)?,
                                None => Default::default(),
                            },
                        },
                    }
                });

                parse_quote! {
                    { Ok(Self::#variant_name { #(#field_parses)* }) }
                }
            }
            Fields::Unit => {
                parse_quote! {
                    { Ok(Self::#variant_name) }
                }
            }
            Fields::Unnamed(unnamed) => {
                let field_parses = unnamed.unnamed.iter().map(|_| {
                    quote! {
                        ::replman::ReplmanParse::parse(parts.next().transpose()?)?,
                    }
                });

                parse_quote! {
                    { Ok(Self::#variant_name(#(#field_parses)*)) }
                }
            }
        };

        variant_matches
            .exact_matches
            .push(parse_quote!( #main_name => #variant_body ));

        variant_matches.aliases.extend(
            effective_variant_name
                .aliases
                .iter()
                .map(|alias| parse_quote!( #alias => #variant_body )),
        );

        variant_matches.start_with_matches.extend(
            effective_variant_name
                .start_withs
                .iter()
                .map(|starts_with| parse_quote!( cmd if cmd.starts_with(#starts_with) => #variant_body ))
        );
    }

    variant_matches
}
