use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;
use syn::{Arm, Attribute, DeriveInput, Fields, Lit};

#[derive(Debug, Default)]
struct Attributes {
    rename_all: Option<Case>,
}

fn str_to_case(s: &str) -> Case {
    match s {
        "snake_case" => Case::Snake,
        "kebab-case" => Case::Kebab,
        "PascalCase" => Case::Pascal,
        "camelCase" => Case::Camel,
        "SCREAMING_SNAKE_CASE" => Case::ScreamingSnake,
        case => panic!("{} is not a supported case", case),
    }
}

impl Attributes {
    fn extract(input_attrs: &[Attribute]) -> Self {
        let mut ret = Self::default();

        iter_over_attrs(input_attrs, &mut ret);

        ret
    }
}

fn iter_over_attrs(input_attrs: &[Attribute], ret: &mut Attributes) {
    for attr in input_attrs {
        if &attr.path == &syn::parse_quote!(replman) {
            let meta = attr.parse_meta().expect("Invalid arguments");

            match meta {
                syn::Meta::List(meta_list) => {
                    for nested in &meta_list.nested {
                        try_extract_rename_all(nested, ret);
                    }
                }
                _ => panic!("Invalid first level meta"),
            }
        }
    }
}

fn try_extract_rename_all(nested: &syn::NestedMeta, ret: &mut Attributes) {
    match nested {
        syn::NestedMeta::Meta(nested_meta) => match nested_meta {
            syn::Meta::NameValue(meta_name_value) => {
                if &meta_name_value.path == &syn::parse_quote!(rename_all) {
                    if let Lit::Str(lit_str) = &meta_name_value.lit {
                        ret.rename_all = Some(str_to_case(&lit_str.value()));
                    } else {
                        panic!("Invalid");
                    }
                }
            }
            _ => panic!("Unsupported"),
        },
        _ => panic!("Unsupported"),
    }
}

pub fn derive_repl_cmd_impl(input: DeriveInput) -> TokenStream {
    let ty = &input.ident;

    let attrs = Attributes::extract(&input.attrs);

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

            fn parse(s: &str) -> anyhow::Result<Self>
            where
                Self: Sized {
                let mut parts = s.split(' ');
                let cmd_word = parts.next().unwrap();

                match cmd_word {
                    #(#variant_matches)*
                    cmd => Err(anyhow::anyhow!("unrecognized command '{}'", cmd)),
                }
            }
        }
    };

    println!("{}", output);

    output.into()
}

fn variant_matches<'a>(
    data_enum: &'a syn::DataEnum,
    attrs: &'a Attributes,
) -> impl Iterator<Item = Arm> + 'a {
    data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let mut variant_name_str = variant_name.to_string();
        if let Some(rename_all) = attrs.rename_all.as_ref() {
            variant_name_str = variant_name_str.to_case(*rename_all);
        }

        println!("{:#?}", variant.attrs);

        match &variant.fields {
            Fields::Named(named) => {
                let field_parses = named.named.iter().map(|named| {
                    let ident = named.ident.as_ref().unwrap();
                    let ident_string = ident.to_string();

                    quote! {
                        #ident: parts
                                .next()
                                .ok_or_else(|| anyhow::anyhow!("Missing argument {}", #ident_string))?
                                .parse()?,
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
                let field_parses = unnamed.unnamed.iter().enumerate().map(|(idx, _)| {
                    quote! {
                        parts
                            .next()
                            .ok_or_else(|| anyhow::anyhow!("Missing argument #{}", #idx))?
                            .parse()?,
                    }
                });

                parse_quote! {
                    #variant_name_str => {

                        Ok(Self::#variant_name(
                            #(#field_parses)*
                        ))
                    }
                }
            },
        }
    })
}
