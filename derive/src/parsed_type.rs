use {
    crate::{ParsedFields, ParsedVariants},
    proc_macro2::TokenStream,
    quote::quote,
    syn::{Data, DeriveInput, Ident},
};

pub struct ParsedType {
    name: Ident,
    data: TypeData,
}

impl From<DeriveInput> for ParsedType {
    fn from(input: DeriveInput) -> Self {
        ParsedType {
            name: input.ident,
            data: TypeData::from(input.data),
        }
    }
}

impl ParsedType {
    pub fn generate_peg_ast_node_impl(self) -> TokenStream {
        let name = self.name;
        let parse_body = self.data.generate_parse_body();
        let parsed_string_body = self.data.generate_parsed_string_body();
        let expecting_body = self.data.generate_expecting_body();

        quote! {
            impl PegAstNode for #name {
                fn parse(
                    input: &mut impl pegast::input::Input,
                ) -> Result<Self, pegast::ParseError> {
                    #parse_body
                }

                fn parsed_string(&self) -> std::borrow::Cow<'_, str> {
                    #parsed_string_body
                }

                fn expecting() -> Vec<String> {
                    #expecting_body
                }
            }
        }
    }
}

enum TypeData {
    Enum(ParsedVariants),
    Struct(ParsedFields),
}

impl TypeData {
    pub fn from(input_data: Data) -> Self {
        match input_data {
            Data::Enum(data) => TypeData::Enum(ParsedVariants::new(data.variants)),
            Data::Struct(data) => TypeData::Struct(ParsedFields::new(data.fields)),
            Data::Union(_) => panic!("Derive(PegAstNode) not supported on unions"),
        }
    }

    pub fn generate_parse_body(&self) -> TokenStream {
        match self {
            TypeData::Enum(variants) => variants.generate_parse_body(),
            TypeData::Struct(fields) => fields.generate_parse_body(quote! { Self }),
        }
    }

    pub fn generate_parsed_string_body(&self) -> TokenStream {
        match self {
            TypeData::Enum(variants) => variants.generate_parsed_string_body(),
            TypeData::Struct(fields) => fields.generate_parsed_string_body_for_structs(),
        }
    }

    pub fn generate_expecting_body(&self) -> TokenStream {
        match self {
            TypeData::Enum(variants) => variants.generate_expecting_body(),
            TypeData::Struct(fields) => fields.generate_expecting_body(),
        }
    }
}
