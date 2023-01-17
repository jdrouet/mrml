use inflector::cases::snakecase::to_snake_case;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DataEnum, DeriveInput, Field, Fields, FieldsUnnamed, Ident, Variant,
};

pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    if let Some(data_enum) = as_data_enum(&ast) {
        derive_children_enum(&ast, data_enum).into()
    } else {
        panic!("MrmlParseChildren only works with enums.")
    }
}

fn derive_children_enum(ast: &DeriveInput, data_enum: &DataEnum) -> proc_macro2::TokenStream {
    let name = &ast.ident;

    let from_variants = data_enum
        .variants
        .iter()
        .map(|v| {
            let variant = &v.ident;
            let field = get_variant_single_field(v).expect("a variant should have a field");
            quote! {
                impl From<#field> for #name {
                    fn from(value: #field) -> Self {
                        Self::#variant(value)
                    }
                }
            }
        })
        .collect::<proc_macro2::TokenStream>();

    let as_variants = data_enum
        .variants
        .iter()
        .map(|v| {
            let variant = &v.ident;
            let fname = Ident::new(
                &format!("as_{}", to_snake_case(&v.ident.to_string())),
                v.ident.span(),
            );
            let field = get_variant_single_field(v).expect("a variant should have a field");
            quote! {
                pub fn #fname(&self) -> Option<&#field> {
                    if let Self::#variant(inner) = self {
                        Some(inner)
                    } else {
                        None
                    }
                }
            }
        })
        .collect::<proc_macro2::TokenStream>();

    quote! {
        #from_variants

        impl #name {
            #as_variants
        }
    }
}

fn as_data_enum(ast: &DeriveInput) -> Option<&DataEnum> {
    if let Data::Enum(inner) = &(ast.data) {
        Some(inner)
    } else {
        None
    }
}

fn get_variant_single_field(variant: &Variant) -> Option<&Field> {
    if let Fields::Unnamed(FieldsUnnamed { unnamed, .. }) = &variant.fields {
        Some(unnamed.first().unwrap())
    } else {
        None
    }
}
