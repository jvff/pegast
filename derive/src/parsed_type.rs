use {
    proc_macro2::TokenStream,
    quote::quote,
    syn::{DeriveInput, Ident},
};

pub struct ParsedType {
    name: Ident,
}

impl From<DeriveInput> for ParsedType {
    fn from(input: DeriveInput) -> Self {
        ParsedType { name: input.ident }
    }
}

impl ParsedType {
    pub fn generate_peg_ast_node_impl(self) -> TokenStream {
        let name = self.name;

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
                    todo!();
                }
            }
        }
    }
}
