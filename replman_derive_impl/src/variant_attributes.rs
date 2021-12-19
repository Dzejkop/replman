use syn::Attribute;

#[derive(Debug, Default)]
pub struct VariantAttributes {}

impl VariantAttributes {
    pub fn _extract(_input_attrs: &[Attribute]) -> Self {
        let ret = Self::default();

        ret
    }
}
