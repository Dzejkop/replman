use convert_case::Casing;
use syn::Variant;

use crate::enum_attributes::EnumAttributes;

pub fn effective_variant_name(
    variant: &Variant,
    attrs: &EnumAttributes,
) -> String {
    let variant_name = &variant.ident;
    let mut variant_name_str = variant_name.to_string();
    if let Some(rename_all) = attrs.rename_all.as_ref() {
        variant_name_str = variant_name_str.to_case(*rename_all);
    }

    variant_name_str
}
