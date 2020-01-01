use {
    proc_macro2::TokenStream,
    quote::quote,
    syn::{spanned::Spanned, Field, Fields, Ident, Index, Member, Type},
};

pub struct ParsedFields {
    fields: Vec<ParsedField>,
    field_type: FieldType,
}

impl ParsedFields {
    pub fn new(fields: Fields) -> Self {
        match fields {
            Fields::Unit => panic!("Can't derive(PegAstNode) on a unit struct"),
            Fields::Named(fields) => ParsedFields {
                fields: fields
                    .named
                    .into_iter()
                    .map(ParsedField::from_named_field)
                    .collect(),
                field_type: FieldType::Named,
            },
            Fields::Unnamed(fields) => ParsedFields {
                fields: fields
                    .unnamed
                    .into_iter()
                    .enumerate()
                    .map(|(index, field)| ParsedField::from_unnamed_field(index as u32, field))
                    .collect(),
                field_type: FieldType::Unnamed,
            },
        }
    }

    pub fn generate_parsed_string_body(&self) -> TokenStream {
        if self.fields.len() == 1 {
            let member = &self
                .fields
                .first()
                .expect("Missing first element in a vector of one element")
                .member;

            quote! { self.#member.parsed_string() }
        } else {
            let members = self.fields.iter().map(|field| &field.member);

            quote! {
                let mut string = String::new();

                #( string.push_str(&self.#members.parsed_string()); )*

                std::borrow::Cow::Owned(string)
            }
        }
    }

    pub fn generate_expecting_body(&self) -> TokenStream {
        let field_type = &self
            .fields
            .first()
            .expect("Missing first element in fields")
            .field_type;

        quote! { <#field_type as PegAstNode>::expecting() }
    }
}

enum FieldType {
    Named,
    Unnamed,
}

struct ParsedField {
    name: Ident,
    member: Member,
    field_type: Type,
}

impl ParsedField {
    pub fn from_named_field(field: Field) -> Self {
        let ident = field.ident.expect("Named field is missing the field name");

        ParsedField {
            name: ident.clone(),
            member: Member::Named(ident),
            field_type: field.ty,
        }
    }

    pub fn from_unnamed_field(index: u32, field: Field) -> Self {
        let field_type = field.ty;
        let span = field_type.span();

        ParsedField {
            name: Ident::new(&format!("_{}", index), span),
            member: Member::Unnamed(Index { index, span }),
            field_type,
        }
    }
}
