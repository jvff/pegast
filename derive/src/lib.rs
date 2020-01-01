extern crate proc_macro;

mod parsed_fields;
mod parsed_type;

use {
    self::{parsed_fields::ParsedFields, parsed_type::ParsedType},
    proc_macro::TokenStream,
    syn::{parse_macro_input, DeriveInput},
};

#[proc_macro_derive(PegAstNode)]
pub fn derive_peg_ast(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    let parsed_type = ParsedType::from(parsed_input);

    TokenStream::from(parsed_type.generate_peg_ast_node_impl())
}
