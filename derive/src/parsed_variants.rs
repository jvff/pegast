use {
    crate::ParsedFields,
    proc_macro2::TokenStream,
    syn::{punctuated::Punctuated, Ident, Token, Variant},
};

pub struct ParsedVariants {
    names: Vec<Ident>,
    fields: Vec<ParsedFields>,
}

impl ParsedVariants {
    pub fn new(variants: Punctuated<Variant, Token![,]>) -> Self {
        let count = variants.iter().count();
        let mut names = Vec::with_capacity(count);
        let mut fields = Vec::with_capacity(count);

        for variant in variants {
            names.push(variant.ident);
            fields.push(ParsedFields::new(variant.fields));
        }

        ParsedVariants { names, fields }
    }

    pub fn generate_parse_body(&self) -> TokenStream {
        todo!();
    }

    pub fn generate_parsed_string_body(&self) -> TokenStream {
        todo!();
    }

    pub fn generate_expecting_body(&self) -> TokenStream {
        todo!();
    }
}
