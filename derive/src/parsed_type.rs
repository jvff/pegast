use {
    crate::{ParsedFields, ParsedGenerics, ParsedVariants},
    proc_macro2::TokenStream,
    quote::quote,
    syn::{Data, DeriveInput, Ident},
};

pub struct ParsedType {
    name: Ident,
    generics: ParsedGenerics,
    data: TypeData,
}

impl From<DeriveInput> for ParsedType {
    fn from(input: DeriveInput) -> Self {
        ParsedType {
            name: input.ident,
            generics: ParsedGenerics::from(input.generics),
            data: TypeData::from(input.data),
        }
    }
}

impl ParsedType {
    pub fn generate_peg_ast_node_impl(self) -> TokenStream {
        let name = self.name;
        let impl_generics = self.generics.impl_generics();
        let type_parameters = self.generics.type_parameters();
        let where_clause = self.generics.where_clause();
        let parse_body = self.data.generate_parse_body();
        let parsed_string_body = self.data.generate_parsed_string_body();
        let parsed_string_length_body = self.data.generate_parsed_string_length_body();
        let expecting_body = self.data.generate_expecting_body();

        quote! {
            impl #impl_generics PegAstNode for #name #type_parameters
            #where_clause
            {
                fn parse(
                    input: &mut impl pegast::input::Input,
                ) -> Result<Self, pegast::ParseError> {
                    #parse_body
                }

                fn parsed_string(&self) -> std::borrow::Cow<'_, str> {
                    #parsed_string_body
                }

                fn parsed_string_length(&self) -> usize {
                    #parsed_string_length_body
                }

                fn expecting() -> Vec<String> {
                    #expecting_body
                }
            }
        }
    }

    pub fn generate_set_entries_impl(self) -> TokenStream {
        let name = self.name;
        let entry_id_name = Ident::new(&format!("{}SetEntryId", name), name.span());

        let variants = self
            .data
            .variants()
            .expect("SetEntries can only be derived for enums");
        let variant_names = variants.names().collect::<Vec<_>>();
        let variant_bindings = variants.generate_ignoring_pattern_bindings();
        let variant_min_repetitions = variants.generate_min_repetitions();
        let variant_max_repetitions = variants.generate_max_repetitions();

        quote! {
            impl pegast::rules::sets::SetEntries for #name {
                type EntryId = #entry_id_name;

                fn all_entry_ids() -> &'static [Self::EntryId] {
                    &[
                        #( #entry_id_name :: #variant_names, )*
                    ]
                }

                fn entry_id(&self) -> Self::EntryId {
                    match self {
                        #(
                            Self::#variant_names #variant_bindings => {
                                #entry_id_name :: #variant_names
                            }
                        )*
                    }
                }

                fn min_repetitions(entry_id: Self::EntryId) -> usize {
                    match entry_id {
                        #( #entry_id_name::#variant_names => #variant_min_repetitions, )*
                    }
                }

                fn max_repetitions(entry_id: Self::EntryId) -> Option<usize> {
                    match entry_id {
                        #( #entry_id_name::#variant_names => #variant_max_repetitions, )*
                    }
                }
            }

            #[derive(Clone, Copy, Debug, Eq, PartialEq, std::hash::Hash)]
            pub enum #entry_id_name {
                #( #variant_names, )*
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

    pub fn variants(self) -> Option<ParsedVariants> {
        match self {
            TypeData::Enum(variants) => Some(variants),
            _ => None,
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

    pub fn generate_parsed_string_length_body(&self) -> TokenStream {
        match self {
            TypeData::Enum(variants) => variants.generate_parsed_string_length_body(),
            TypeData::Struct(fields) => fields.generate_parsed_string_length_body_for_structs(),
        }
    }

    pub fn generate_expecting_body(&self) -> TokenStream {
        match self {
            TypeData::Enum(variants) => variants.generate_expecting_body(),
            TypeData::Struct(fields) => fields.generate_expecting_body(),
        }
    }
}
