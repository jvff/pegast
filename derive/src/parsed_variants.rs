use {
    crate::ParsedFields,
    proc_macro2::TokenStream,
    quote::quote,
    syn::{punctuated::Punctuated, Ident, Token, Variant},
};

pub struct ParsedVariant {
    pub name: Ident,
    pub fields: ParsedFields,
}

impl From<Variant> for ParsedVariant {
    fn from(variant: Variant) -> Self {
        ParsedVariant {
            name: variant.ident,
            fields: ParsedFields::new(variant.fields),
        }
    }
}

pub struct ParsedVariants {
    variants: Vec<ParsedVariant>,
}

impl ParsedVariants {
    pub fn new(variants: Punctuated<Variant, Token![,]>) -> Self {
        let variants = variants.into_iter().map(ParsedVariant::from).collect();

        ParsedVariants { variants }
    }

    pub fn names(&self) -> impl Iterator<Item = &Ident> + '_ {
        self.variants.iter().map(|variant| &variant.name)
    }

    pub fn generate_pattern_bindings(&self) -> impl Iterator<Item = TokenStream> + '_ {
        self.variants
            .iter()
            .map(|variant| variant.fields.generate_pattern_bindings())
    }

    pub fn generate_parse_body(&self) -> TokenStream {
        let variant_parsers = self.variants.iter().map(|variant| {
            let variant_name = &variant.name;

            variant
                .fields
                .generate_parse_body(quote! { Self::#variant_name })
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
        let variant_names = self.names();
        let bindings = self.generate_pattern_bindings();
        let variant_parsed_strings = self.variants.iter().map(|variant| {
            variant
                .fields
                .generate_parsed_string_body_for_enum_variants()
        });

        quote! {
            match self {
                #( Self::#variant_names #bindings => { #variant_parsed_strings } )*
            }
        }
    }

    pub fn generate_parsed_string_length_body(&self) -> TokenStream {
        let variant_names = self.names();
        let bindings = self.generate_pattern_bindings();
        let variant_parsed_string_lengths = self.variants.iter().map(|variant| {
            variant
                .fields
                .generate_parsed_string_length_body_for_enum_variants()
        });

        quote! {
            match self {
                #( Self::#variant_names #bindings => { #variant_parsed_string_lengths } )*
            }
        }
    }

    pub fn generate_expecting_body(&self) -> TokenStream {
        let variant_expecting = self
            .variants
            .iter()
            .map(|variant| variant.fields.generate_expecting_body());

        quote! {
            let mut expecting = Vec::new();

            #( expecting.extend(#variant_expecting); )*

            expecting
        }
    }
}
