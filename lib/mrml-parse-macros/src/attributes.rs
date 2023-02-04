use common_macros::{as_path, get_fields, is_option};
use inflector::cases::kebabcase::to_kebab_case;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field, Ident};

fn create_builder_struct_field(field: &Field) -> Option<proc_macro2::TokenStream> {
    if let Some(ref field_ident) = field.ident {
        if as_path(field).map(is_option).unwrap_or(false) {
            let ty = &field.ty;
            Some(quote! {
                #field_ident: #ty,
            })
        } else {
            let ty = &field.ty;
            Some(quote! {
                #field_ident: Option<#ty>,
            })
        }
    } else {
        None
    }
}

fn create_builder_build_field(field: &Field) -> Option<proc_macro2::TokenStream> {
    if let Some(ref field_ident) = field.ident {
        if as_path(field).map(is_option).unwrap_or(false) {
            Some(quote! {
                #field_ident: self.#field_ident,
            })
        } else {
            Some(quote! {
                #field_ident: self.#field_ident
                    .ok_or_else(|| crate::prelude::parse::Error::MissingAttribute(stringify!(#field_ident)))?,
            })
        }
    } else {
        None
    }
}

fn create_builder_insert_field(field: &Field) -> Option<proc_macro2::TokenStream> {
    if let Some(ref field_ident) = field.ident {
        let attribute = match field_ident.to_string().as_str() {
            // TODO make this a macro like #[mrml_parse(rename = "type")]
            "kind" => "type".to_string(),
            other => to_kebab_case(other),
        };
        Some(quote! {
            #attribute => {
                self.#field_ident = Some(value.to_string());
            }
        })
    } else {
        None
    }
}

fn create_builder(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let ident = &ast.ident;

    let builder_name = format!("{ident}Builder");
    let builder_ident = Ident::new(&builder_name, ident.span());

    let struct_fields = get_fields(ast)
        .iter()
        .filter_map(create_builder_struct_field);

    let build_fields = get_fields(ast)
        .iter()
        .filter_map(create_builder_build_field);

    let insert_fields = get_fields(ast)
        .iter()
        .filter_map(create_builder_insert_field);

    quote! {
        #[derive(Debug, Default)]
        struct #builder_ident {
            #(#struct_fields)*
        }

        impl #builder_ident {
            fn build(self) -> Result<#ident, crate::prelude::parse::Error> {
                Ok(#ident {
                    #(#build_fields)*
                })
            }

            fn insert<'a>(&mut self, name: xmlparser::StrSpan<'a>, value: xmlparser::StrSpan<'a>) -> Result<(), crate::prelude::parse::Error> {
                match name.as_str() {
                    #(#insert_fields)*
                    _ => return Err(crate::prelude::parse::Error::UnexpectedAttribute(name.start())),
                };
                Ok(())
            }
        }
    }
}

pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    create_builder(&ast).into()
}
