use {
    crate::ParsedFields,
    proc_macro2::TokenStream,
    quote::quote,
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
        let variant_parsers =
            self.fields
                .iter()
                .zip(&self.names)
                .map(|(variant_fields, variant_name)| {
                    variant_fields.generate_parse_body(quote! { Self::#variant_name })
                });

        quote! {
            Err(())
                #( .or_else(|_| -> Result<_, pegast::ParseError> { #variant_parsers }) )*
                .or_else(|_| Err(pegast::ParseError {
                    expected: Self::expecting(),
                    position: input.position(),
                }))
        }
    }

    pub fn generate_parsed_string_body(&self) -> TokenStream {
        let variant_names = &self.names;
        let bindings = self
            .fields
            .iter()
            .map(|variant_fields| variant_fields.generate_pattern_bindings());
        let variant_parsed_strings = self
            .fields
            .iter()
            .map(|variant_fields| variant_fields.generate_parsed_string_body_for_enum_variants());

        quote! {
            match self {
                #( Self::#variant_names #bindings => { #variant_parsed_strings } )*
            }
        }
    }

    pub fn generate_expecting_body(&self) -> TokenStream {
        let variant_expecting = self
            .fields
            .iter()
            .map(|variant_fields| variant_fields.generate_expecting_body());

        quote! {
            let mut expecting = Vec::new();

            #( expecting.extend(#variant_expecting); )*

            expecting
        }
    }
}
