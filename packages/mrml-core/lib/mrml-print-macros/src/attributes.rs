use mrml_common_macros::{as_path, get_fields, is_option};
use proc_macro2::Ident;
use quote::quote;
use syn::DeriveInput;

pub(crate) struct Generator {
    struct_ident: Ident,
    struct_generic: Option<Ident>,
    fields: Vec<(Option<Ident>, Option<bool>)>,
}

impl From<DeriveInput> for Generator {
    fn from(ast: DeriveInput) -> Self {
        Self {
            struct_ident: ast.ident.clone(),
            struct_generic: mrml_common_macros::get_generics(&ast),
            fields: get_fields(&ast)
                .into_iter()
                .map(|f| (f.ident.clone(), as_path(f).map(is_option)))
                .collect(),
        }
    }
}

impl Generator {
    fn build_impl_head(&self) -> proc_macro2::TokenStream {
        let name = &self.struct_ident;
        if let Some(ref gen) = self.struct_generic {
            quote! { impl<#gen> #name<#gen> }
        } else {
            quote! { impl #name }
        }
    }

    pub(crate) fn build(self) -> proc_macro2::TokenStream {
        let head = self.build_impl_head();
        let fields = self.fields.into_iter().filter_map(|attr| match attr {
            (Some(ident), Some(true)) => Some(quote! {
                if let Some(ref value) = self.#ident {
                    res.insert(stringify!(#ident).to_string(), value.to_string());
                }
            }),
            (Some(ident), Some(false)) => Some(quote! {
                res.insert(stringify!(#ident).to_string(), self.#ident.to_string());
            }),
            _ => None,
        });

        quote! {
            #head {
                fn as_map(&self) -> crate::prelude::hash::Map<String, String> {
                    let mut res = crate::prelude::hash::Map::new();
                    #(#fields)*
                    res
                }
            }
        }
    }
}
