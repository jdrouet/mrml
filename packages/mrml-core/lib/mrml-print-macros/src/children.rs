use mrml_common_macros::{as_path, is_option};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput};

pub(crate) enum Generator {
    Enum(EnumGenerator),
    Struct(StructGenerator),
}

pub(crate) struct EnumGenerator {
    ident: Ident,
    generic: Option<Ident>,
    variants: Vec<Ident>,
}

impl From<(&DeriveInput, &DataEnum)> for EnumGenerator {
    fn from((ast, data): (&DeriveInput, &DataEnum)) -> Self {
        let ident = ast.ident.clone();
        let generic = mrml_common_macros::get_generics(ast);
        let variants = data.variants.iter().map(|v| v.ident.clone()).collect();

        EnumGenerator {
            ident,
            generic,
            variants,
        }
    }
}

impl EnumGenerator {
    fn build_print_head(&self) -> TokenStream {
        let name = &self.ident;
        if let Some(ref gen) = self.generic {
            quote! { impl<#gen> crate::prelude::print::Print for #name<#gen> }
        } else {
            quote! { impl crate::prelude::print::Print for #name }
        }
    }

    fn build(self) -> TokenStream {
        let head = self.build_print_head();
        let fields = self
            .variants
            .into_iter()
            .map(|variant| {
                quote! {
                    Self::#variant(elt) => elt.print(pretty, level, indent_size),
                }
            })
            .collect::<proc_macro2::TokenStream>();

        quote! {
            #head {
                fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
                    match self {
                        #fields
                    }
                }
            }
        }
    }
}

pub(crate) struct StructGenerator {
    ident: Ident,
    generic: Option<Ident>,
    // (name, is_optional)
    fields: Vec<(Ident, bool)>,
}

impl From<(&DeriveInput, &DataStruct)> for StructGenerator {
    fn from((ast, data): (&DeriveInput, &DataStruct)) -> Self {
        let ident = ast.ident.clone();
        let generic = mrml_common_macros::get_generics(ast);
        let fields = data
            .fields
            .iter()
            .filter_map(|f| match (&f.ident, as_path(f).map(is_option)) {
                (Some(ident), Some(v)) => Some((ident.clone(), v)),
                _ => None,
            })
            .collect();

        StructGenerator {
            ident,
            generic,
            fields,
        }
    }
}

impl StructGenerator {
    fn build_print_head(&self) -> TokenStream {
        let name = &self.ident;
        if let Some(ref gen) = self.generic {
            quote! { impl<#gen> crate::prelude::print::Print for #name<#gen> }
        } else {
            quote! { impl crate::prelude::print::Print for #name }
        }
    }

    fn build(self) -> TokenStream {
        let head = self.build_print_head();
        let fields = self.fields.into_iter().map(|(ident, optional)| {
            if optional {
                quote! {
                    if let Some(ref value) = self.#ident {
                        res.push_str(&value.print(pretty, level, indent_size));
                    }
                }
            } else {
                quote! {
                    res.push_str(&self.#ident.print(pretty, level, indent_size));
                }
            }
        });

        quote! {
            #head {
                fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
                    let mut res = String::new();
                    #(#fields)*
                    res
                }
            }
        }
    }
}

impl From<DeriveInput> for Generator {
    fn from(ast: DeriveInput) -> Self {
        match ast.data {
            Data::Enum(ref inner) => Self::Enum(EnumGenerator::from((&ast, inner))),
            Data::Struct(ref inner) => Self::Struct(StructGenerator::from((&ast, inner))),
            _ => panic!("MrmlPrintChildren only works with enums and structs."),
        }
    }
}

impl Generator {
    pub fn build(self) -> TokenStream {
        match self {
            Self::Enum(inner) => inner.build(),
            Self::Struct(inner) => inner.build(),
        }
    }
}
