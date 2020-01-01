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
        let expecting_body = self.fields.generate_expecting_body();

        quote! {
            impl PegAstNode for #name {
                fn parse(
                    input: &mut impl pegast::input::Input,
                ) -> Result<Self, pegast::ParseError> {
                    todo!();
                }

                fn parsed_string(&self) -> std::borrow::Cow<'_, str> {
                    todo!();
                }

                fn expecting() -> Vec<String> {
                    #expecting_body
                }
            }
        }
    }
}
