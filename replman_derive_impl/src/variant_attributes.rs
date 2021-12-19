use syn::{parse_quote, Attribute, Lit, LitStr};

#[derive(Debug, Default)]
pub struct VariantAttributes {
    pub aliases: Vec<LitStr>,
    pub doc_lines: Vec<LitStr>,
}

impl VariantAttributes {
    pub fn extract(attrs: &[Attribute]) -> Self {
        let mut ret = Self::default();

        for attr in attrs {
            if attr.path == parse_quote!(doc) {
                extract_doc_line(attr, &mut ret.doc_lines);
            }

            if attr.path == parse_quote!(replman) {
                let attr = attr.parse_meta().expect("Must be meta");
                match attr {
                    syn::Meta::List(meta_list) => {
                        for nested in &meta_list.nested {
                            extract_replman(nested, &mut ret);
                        }
                    }
                    _ => panic!("Invalid doc attribute kind"),
                }
            }
        }

        ret
    }
}

fn extract_doc_line(attr: &Attribute, ret: &mut Vec<LitStr>) {
    let attr = attr.parse_meta().expect("Must be meta");
    match attr {
        syn::Meta::NameValue(name_value) => match name_value.lit {
            syn::Lit::Str(lit_str) => {
                // ignore leading space
                let lit_str = if lit_str.value().starts_with(' ') {
                    LitStr::new(&lit_str.value()[1..], lit_str.span())
                } else {
                    lit_str
                };

                ret.push(lit_str);
            }
            _ => panic!("Invalid literal kind for a doc comment"),
        },
        _ => panic!("Invalid doc attribute kind"),
    }
}

fn extract_replman(nested: &syn::NestedMeta, ret: &mut VariantAttributes) {
    match nested {
        syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) => {
            if name_value.path == parse_quote!(alias) {
                if let Lit::Str(lit_str) = &name_value.lit {
                    ret.aliases.push(lit_str.clone());
                } else {
                    panic!("Invalid literal");
                }
            }
        }
        _ => panic!("Unsupported"),
    }
}
