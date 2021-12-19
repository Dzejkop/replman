use std::fmt::Write;

use syn::spanned::Spanned;
use syn::{parse_quote, Attribute, DataEnum, LitStr};

use crate::common::effective_variant_name;
use crate::enum_attributes::EnumAttributes;

#[allow(unused_must_use)]
pub fn derive_help_method(
    input: &DataEnum,
    attrs: &EnumAttributes,
) -> syn::ImplItemMethod {
    let mut help_str = "".to_string();

    for variant in &input.variants {
        let variant_name_str = effective_variant_name(&variant, attrs);
        let doc_lines = extract_docs_from_attrs(&variant.attrs);

        write!(&mut help_str, "{}", variant_name_str);

        if !doc_lines.is_empty() {
            write!(&mut help_str, " - ");
            writeln!(&mut help_str, "{}", &doc_lines[0]);
            let alignment = variant_name_str.len() + 3;
            let alignment: String = (0..alignment).map(|_| ' ').collect();

            for (idx, doc_line) in doc_lines[1..].iter().enumerate() {
                if !doc_line.trim().is_empty() {
                    write!(&mut help_str, "{}{}", alignment, doc_line);
                }

                if idx != doc_lines.len() {
                    writeln!(&mut help_str);
                }
            }
        } else {
            writeln!(&mut help_str);
        }
    }

    let help_str_literal = LitStr::new(&help_str, input.enum_token.span());

    parse_quote! {
        fn help() -> &'static str {
            #help_str_literal
        }
    }
}

fn extract_docs_from_attrs(attrs: &[Attribute]) -> Vec<String> {
    let mut ret = vec![];

    for attr in attrs {
        if attr.path != parse_quote!(doc) {
            continue;
        }

        let attr = attr.parse_meta().expect("Must be meta");
        match attr {
            syn::Meta::NameValue(name_value) => match name_value.lit {
                syn::Lit::Str(lit_str) => {
                    let value = lit_str.value();
                    if value.starts_with(" ") {
                        ret.push(value[1..].to_string());
                    } else {
                        ret.push(value);
                    }
                }
                _ => panic!("Invalid literal kind for a doc comment"),
            },
            _ => panic!("Invalid doc attribute kind"),
        }
    }

    ret
}
