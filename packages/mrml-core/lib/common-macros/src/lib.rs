use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Field, Fields, FieldsNamed, FieldsUnnamed,
    GenericArgument, GenericParam, Ident, Path, PathArguments, Type, TypeParam, TypePath, Variant,
};

pub fn as_data_enum(ast: &DeriveInput) -> Option<&DataEnum> {
    if let Data::Enum(inner) = &(ast.data) {
        Some(inner)
    } else {
        None
    }
}

pub fn get_variant_single_field(variant: &Variant) -> Option<&Field> {
    if let Fields::Unnamed(FieldsUnnamed { unnamed, .. }) = &variant.fields {
        Some(unnamed.first().unwrap())
    } else {
        None
    }
}

pub fn is_vec(path: &Path) -> bool {
    path.segments
        .first()
        // TODO make sure that it's a Vec<T>
        .map(|s| s.ident == "Vec")
        .unwrap_or(false)
}

pub fn get_generics(ast: &DeriveInput) -> Option<Ident> {
    ast.generics.params.first().and_then(|p| {
        if let GenericParam::Type(TypeParam { ident, .. }) = p {
            Some(ident.clone())
        } else {
            None
        }
    })
}

fn get_vec_type(path: &Path) -> Type {
    let res = &path.segments.first().unwrap().arguments;
    let res = if let PathArguments::AngleBracketed(arg) = res {
        arg
    } else {
        panic!("expected path arguments of kind angle bracketed");
    };
    let res = res.args.first().unwrap();
    let res = if let GenericArgument::Type(ty) = res {
        ty
    } else {
        panic!("expected generic argument of kind Type");
    };
    res.to_owned()
}

pub fn is_map(path: &Path) -> bool {
    path.segments
        .first()
        // TODO make sure that it's a Vec<String, String>
        .map(|s| s.ident == "Map")
        .unwrap_or(false)
}

pub fn is_option(path: &Path) -> bool {
    path.segments
        .first()
        // TODO make sure that it's a Option<T>
        .map(|s| s.ident == "Option")
        .unwrap_or(false)
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

pub enum ChildrenKind {
    String,
    Single,
    List(Type),
    None,
}

pub fn get_children_kind(ast: &DeriveInput) -> ChildrenKind {
    if let Some(field) = get_children_field(ast) {
        match &field.ty {
            Type::Path(TypePath { path, .. }) if path.is_ident("String") => ChildrenKind::String,
            Type::Path(TypePath { path, .. }) if is_vec(path) => {
                ChildrenKind::List(get_vec_type(path))
            }
            _ => ChildrenKind::Single,
        }
    } else {
        ChildrenKind::None
    }
}

pub fn get_attributes_field(ast: &DeriveInput) -> Option<&Field> {
    get_fields(ast).into_iter().find(|f| {
        f.ident
            .as_ref()
            .map(|id| *id == "attributes")
            .unwrap_or(false)
    })
}

pub fn as_path(field: &Field) -> Option<&Path> {
    match &field.ty {
        Type::Path(TypePath { path, .. }) => Some(path),
        _ => None,
    }
}

pub fn get_attributes_kind(ast: &DeriveInput) -> AttributesKind {
    if let Some(field) = get_attributes_field(ast) {
        match &field.ty {
            Type::Path(TypePath { path, .. }) if is_map(path) => AttributesKind::Map,
            Type::Path(TypePath { path, .. }) => {
                if let Some(ident) = path.get_ident() {
                    AttributesKind::Struct(ident.clone())
                } else {
                    AttributesKind::None
                }
            }
            _ => AttributesKind::None,
        }
    } else {
        AttributesKind::None
    }
}

pub enum AttributesKind {
    Map,
    Struct(Ident),
    None,
}
