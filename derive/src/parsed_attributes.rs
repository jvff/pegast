use {
    proc_macro2::Span,
    std::collections::HashMap,
    syn::{Attribute, Ident, Lit, MetaNameValue},
};

pub struct ParsedAttributes {
    key_value_pairs: HashMap<String, Lit>,
}

impl<T> From<T> for ParsedAttributes
where
    T: IntoIterator<Item = Attribute>,
{
    fn from(attributes: T) -> Self {
        let mut key_value_pairs = HashMap::new();
        let pegast_ident = Ident::new("pegast", Span::call_site());
        let pegast_attributes = attributes
            .into_iter()
            .filter(|attribute| attribute.path.is_ident(&pegast_ident));

        for attribute in pegast_attributes {
            if let Ok(meta) = attribute.parse_args::<MetaNameValue>() {
                let key = meta
                    .path
                    .get_ident()
                    .expect("Invalid pegast attribute")
                    .to_string();

                key_value_pairs.insert(key, meta.lit);
            } else {
                panic!("Invalid pegast attribute");
            }
        }

        ParsedAttributes { key_value_pairs }
    }
}

impl ParsedAttributes {
    pub fn get_value(&self, key: &str) -> Option<&Lit> {
        self.key_value_pairs.get(key)
    }
}
