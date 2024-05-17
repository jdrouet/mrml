use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, GenericParam, Ident, TypeParam,
};

pub fn get_generics(ast: &DeriveInput) -> Option<Ident> {
    ast.generics.params.first().and_then(|p| {
        if let GenericParam::Type(TypeParam { ident, .. }) = p {
            Some(ident.clone())
        } else {
            None
        }
    })
}

pub fn as_data_struct(ast: &DeriveInput) -> Option<&DataStruct> {
    if let Data::Struct(inner) = &(ast.data) {
        Some(inner)
    } else {
        None
    }
}

pub fn as_fields_named(input: &DataStruct) -> Option<&FieldsNamed> {
    if let Fields::Named(inner) = &input.fields {
        Some(inner)
    } else {
        None
    }
}

pub fn get_fields(ast: &DeriveInput) -> &Punctuated<Field, Comma> {
    as_data_struct(ast)
        .and_then(as_fields_named)
        .map(|f| &f.named)
        .expect("MrmlParseComponent only supports structs.")
}

pub fn get_children_field(ast: &DeriveInput) -> Option<&Field> {
    get_fields(ast).into_iter().find(|f| {
        f.ident
            .as_ref()
            .map(|id| *id == "children")
            .unwrap_or(false)
    })
}

pub fn get_attributes_field(ast: &DeriveInput) -> Option<&Field> {
    get_fields(ast).into_iter().find(|f| {
        f.ident
            .as_ref()
            .map(|id| *id == "attributes")
            .unwrap_or(false)
    })
}
