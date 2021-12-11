use syn::{parse_quote, Attribute, Lit};

#[derive(Debug, Clone, Default)]
pub struct FieldAttributes {
    pub default: FieldDefault,
}

#[derive(Debug, Clone)]
pub enum FieldDefault {
    None,
    Some(String),
    Default,
}

impl FieldDefault {
    pub fn is_none(&self) -> bool {
        matches!(self, FieldDefault::None)
    }
}

impl Default for FieldDefault {
    fn default() -> Self {
        Self::None
    }
}

impl FieldAttributes {
    pub fn extract(attrs: &[Attribute]) -> Self {
        let mut ret = Self::default();

        iter_over_attrs(attrs, &mut ret);

        ret
    }
}

fn iter_over_attrs(input_attrs: &[Attribute], ret: &mut FieldAttributes) {
    for attr in input_attrs {
        if &attr.path == &syn::parse_quote!(replman) {
            let meta = attr.parse_meta().expect("Invalid arguments");

            match meta {
                syn::Meta::List(meta_list) => {
                    for nested in &meta_list.nested {
                        try_extract_default(nested, ret);
                    }
                }
                _ => panic!("Invalid first level meta"),
            }
        }
    }
}

fn try_extract_default(nested: &syn::NestedMeta, ret: &mut FieldAttributes) {
    match nested {
        syn::NestedMeta::Meta(nested_meta) => {
            println!("X: {:?}", nested_meta);
            match nested_meta {
                syn::Meta::Path(meta_path) => {
                    if meta_path == &parse_quote!(default) {
                        ret.default = FieldDefault::Default;
                    }
                }
                syn::Meta::NameValue(name_value) => {
                    if &name_value.path == &parse_quote!(default) {
                        if let Lit::Str(lit_str) = &name_value.lit {
                            ret.default = FieldDefault::Some(lit_str.value());
                        } else {
                            panic!("Invalid literal");
                        }
                    }
                }
                _ => panic!("Unsupported"),
            }
        }
        _ => panic!("Unsupported"),
    }
}
