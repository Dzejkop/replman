use convert_case::Case;
use syn::{Attribute, Lit};

#[derive(Debug, Default)]
pub struct EnumAttributes {
    pub rename_all: Option<Case>,
}

impl EnumAttributes {
    pub fn extract(input_attrs: &[Attribute]) -> Self {
        let mut ret = Self::default();

        iter_over_attrs(input_attrs, &mut ret);

        ret
    }
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

fn iter_over_attrs(input_attrs: &[Attribute], ret: &mut EnumAttributes) {
    for attr in input_attrs {
        if attr.path == syn::parse_quote!(replman) {
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

fn try_extract_rename_all(nested: &syn::NestedMeta, ret: &mut EnumAttributes) {
    match nested {
        syn::NestedMeta::Meta(syn::Meta::NameValue(meta_name_value)) => {
            if meta_name_value.path == syn::parse_quote!(rename_all) {
                if let Lit::Str(lit_str) = &meta_name_value.lit {
                    ret.rename_all = Some(str_to_case(&lit_str.value()));
                } else {
                    panic!("Invalid");
                }
            }
        }
        _ => panic!("Unsupported"),
    }
}
