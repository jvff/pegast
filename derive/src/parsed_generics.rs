use {
    proc_macro2::TokenStream,
    quote::{quote, ToTokens},
    syn::{Generics, Ident, Lifetime},
};

pub struct ParsedGenerics {
    lifetime_parameters: Vec<Lifetime>,
    type_parameters: Vec<Ident>,
    constraints: Vec<TokenStream>,
}

impl From<Generics> for ParsedGenerics {
    fn from(generics: Generics) -> Self {
        let lifetime_parameters = generics
            .lifetimes()
            .map(|lifetime_definition| lifetime_definition.lifetime.clone())
            .collect();

        let type_parameters = generics
            .type_params()
            .map(|type_parameter| type_parameter.ident.clone())
            .collect();

        let lifetime_constraints = generics
            .lifetimes()
            .filter(|lifetime_definition| lifetime_definition.colon_token.is_some())
            .map(|lifetime_definition| quote! { #lifetime_definition });

        let type_constraints = generics
            .type_params()
            .filter(|type_parameter| type_parameter.colon_token.is_some())
            .map(|type_parameter| quote! { #type_parameter });

        let constraints = lifetime_constraints.chain(type_constraints).collect();

        ParsedGenerics {
            lifetime_parameters,
            type_parameters,
            constraints,
        }
    }
}

impl ParsedGenerics {
    pub fn impl_generics(&self) -> TokenStream {
        self.type_parameters()
    }

    pub fn type_parameters(&self) -> TokenStream {
        let lifetime_parameters = self
            .lifetime_parameters
            .iter()
            .map(|lifetime_parameter| quote! { #lifetime_parameter });

        let type_parameters = self
            .type_parameters
            .iter()
            .map(|type_parameter| quote! { #type_parameter });

        Self::generic_list(lifetime_parameters.chain(type_parameters))
    }

    pub fn where_clause(&self) -> TokenStream {
        if self.constraints.is_empty() {
            quote! {}
        } else {
            let constraints = &self.constraints;

            quote! { where #( #constraints ),* }
        }
    }

    fn generic_list(mut list: impl Iterator<Item = impl ToTokens>) -> TokenStream {
        let first = list.next();

        if first.is_none() {
            quote! {}
        } else {
            quote! { < #first, #( #list ),* > }
        }
    }
}
