use std::fmt;

use convert_case::Casing;
use syn::{LitStr, Variant};

use crate::enum_attributes::EnumAttributes;
use crate::variant_attributes::VariantAttributes;

pub struct EffectiveVariantName {
    pub main_name: LitStr,
    pub aliases: Vec<LitStr>,
}

impl fmt::Display for EffectiveVariantName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.main_name.value())?;
        for alias in &self.aliases {
            write!(f, "|{}", alias.value())?;
        }

        Ok(())
    }
}

pub fn effective_variant_name(
    variant: &Variant,
    enum_attrs: &EnumAttributes,
    variant_attrs: &VariantAttributes,
) -> EffectiveVariantName {
    let variant_name = &variant.ident;
    let mut main_name = variant_name.to_string();
    if let Some(rename_all) = enum_attrs.rename_all.as_ref() {
        main_name = main_name.to_case(*rename_all);
    }

    let main_name = LitStr::new(&main_name, variant.ident.span());

    EffectiveVariantName {
        main_name,
        aliases: variant_attrs.aliases.clone(),
    }
}
