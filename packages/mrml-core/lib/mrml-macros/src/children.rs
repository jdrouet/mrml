use inflector::cases::snakecase::to_snake_case;
use mrml_common_macros::{as_data_enum, get_variant_single_field};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DataEnum, DeriveInput, Ident};

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

    let as_tests = data_enum
        .variants
        .iter()
        // TODO implement a proper test for Node items
        .filter(|v| v.ident != "Node")
        .map(|v| {
            let as_fname = Ident::new(
                &format!("as_{}", to_snake_case(&v.ident.to_string())),
                v.ident.span(),
            );
            let field = get_variant_single_field(v).expect("a variant should have a field");

            quote! {
                #[test]
                fn #as_fname() {
                    let item = #field::default();
                    let child = #name::from(item);
                    assert!(child.#as_fname().is_some());
                }
            }
        })
        .collect::<proc_macro2::TokenStream>();

    quote! {
        #from_variants

        impl #name {
            #as_variants
        }

        #[cfg(test)]
        mod macro_tests {
            use super::*;

            #as_tests
        }
    }
}
