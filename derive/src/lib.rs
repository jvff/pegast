extern crate proc_macro;

mod parsed_attributes;
mod parsed_fields;
mod parsed_generics;
mod parsed_type;
mod parsed_variants;

use {
    self::{
        parsed_attributes::ParsedAttributes, parsed_fields::ParsedFields,
        parsed_generics::ParsedGenerics, parsed_type::ParsedType, parsed_variants::ParsedVariants,
    },
    proc_macro::TokenStream,
    syn::{parse_macro_input, DeriveInput},
};

#[proc_macro_derive(PegAstNode)]
pub fn derive_peg_ast(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    let parsed_type = ParsedType::from(parsed_input);

    TokenStream::from(parsed_type.generate_peg_ast_node_impl())
}

#[proc_macro_derive(SetEntries, attributes(pegast))]
pub fn derive_set_entries(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    let parsed_type = ParsedType::from(parsed_input);

    TokenStream::from(parsed_type.generate_set_entries_impl())
}

#[proc_macro_derive(FromStr)]
pub fn derive_from_str(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    let parsed_type = ParsedType::from(parsed_input);

    TokenStream::from(parsed_type.generate_from_str_impl())
}
