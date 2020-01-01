use {
    crate::ParsedFields,
    proc_macro2::TokenStream,
    quote::quote,
    syn::{Data, DeriveInput, Ident},
};

pub struct ParsedType {
    name: Ident,
    fields: ParsedFields,
}

impl From<DeriveInput> for ParsedType {
    fn from(input: DeriveInput) -> Self {
        let fields = match input.data {
            Data::Struct(data) => ParsedFields::new(data.fields),
            _ => panic!("Currently only structs can have PegAstNode derived"),
        };

        ParsedType {
            name: input.ident,
            fields,
        }
    }
}

impl ParsedType {
    pub fn generate_peg_ast_node_impl(self) -> TokenStream {
        let name = self.name;
        let parse_body = self.fields.generate_parse_body();
        let parsed_string_body = self.fields.generate_parsed_string_body();
        let expecting_body = self.fields.generate_expecting_body();

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
