extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(PegAstNode)]
pub fn derive_peg_ast(input: TokenStream) -> TokenStream {
    todo!();
}
