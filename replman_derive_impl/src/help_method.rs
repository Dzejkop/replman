use std::fmt::Write;

use syn::spanned::Spanned;
use syn::{parse_quote, DataEnum, LitStr};

use crate::common::effective_variant_name;
use crate::enum_attributes::EnumAttributes;
use crate::variant_attributes::VariantAttributes;

#[allow(unused_must_use)] // for the results of write! and writeln!
pub fn derive_help_method(
    input: &DataEnum,
    attrs: &EnumAttributes,
) -> syn::ImplItemMethod {
    let mut help_str = String::new();

    for variant in &input.variants {
        let mut help_line = String::new();

        let variant_attributes = VariantAttributes::extract(&variant.attrs);
        let variant_name_str =
            effective_variant_name(variant, attrs, &variant_attributes);

        let doc_lines = &variant_attributes.doc_lines;

        write!(&mut help_line, "{}", variant_name_str);

        match &variant.fields {
            syn::Fields::Named(named_args) => {
                for arg in &named_args.named {
                    write!(
                        &mut help_line,
                        " <{}>",
                        arg.ident.as_ref().unwrap()
                    );
                }
            }
            syn::Fields::Unnamed(unnamed) => {
                for (idx, _) in unnamed.unnamed.iter().enumerate() {
                    write!(&mut help_line, " <{}>", idx);
                }
            }
            _ => (),
        }

        if !doc_lines.is_empty() {
            write!(&mut help_line, " - ");

            let alignment = help_line.len();
            let alignment: String = (0..alignment).map(|_| ' ').collect();

            write!(&mut help_line, "{}", &doc_lines[0].value());

            for doc_line in doc_lines[1..].iter() {
                writeln!(&mut help_line);
                let line_value = doc_line.value();
                if !line_value.trim().is_empty() {
                    write!(&mut help_line, "{}{}", alignment, line_value);
                }
            }
        }

        writeln!(&mut help_str, "{}", help_line);
    }

    let help_str_literal = LitStr::new(&help_str, input.enum_token.span());

    parse_quote! {
        fn help() -> &'static str {
            #help_str_literal
        }
    }
}
