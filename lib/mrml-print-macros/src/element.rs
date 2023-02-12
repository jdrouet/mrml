use darling::FromDeriveInput;
use mrml_common_macros::{get_attributes_field, get_children_kind, is_map, ChildrenKind};
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{DeriveInput, Type, TypePath};

#[derive(FromDeriveInput)]
#[darling(attributes(mrml_print), forward_attrs(allow, doc, cfg))]
pub(super) struct Opts {
    tag: Option<String>,
    close_empty: Option<bool>,
    indent_children: Option<bool>,
    children: Option<bool>,
}

impl Opts {
    fn children(&self) -> bool {
        self.children.unwrap_or(true)
    }

    fn indent_children(&self) -> bool {
        self.indent_children.unwrap_or(true)
    }
}

pub(super) struct Generator {
    struct_ident: Ident,
    struct_generic: Option<Ident>,
    attributes_field: Option<syn::Field>,
    children_kind: ChildrenKind,
    options: Opts,
}

impl From<(DeriveInput, Opts)> for Generator {
    fn from((ast, options): (DeriveInput, Opts)) -> Self {
        Self {
            struct_ident: ast.ident.clone(),
            struct_generic: mrml_common_macros::get_generics(&ast),
            attributes_field: get_attributes_field(&ast).cloned(),
            children_kind: get_children_kind(&ast),
            options,
        }
    }
}

impl Generator {
    fn tag_name(&self) -> Ident {
        let tag_name = self.options.tag.clone().unwrap_or_else(|| "NAME".into());
        Ident::new(tag_name.as_str(), Span::call_site())
    }

    fn build_print_trait_attributes(&self) -> proc_macro2::TokenStream {
        if let Some(ref field) = self.attributes_field {
            match &field.ty {
                Type::Path(TypePath { path, .. }) if is_map(path) => {
                    quote! { Some(&self.attributes) }
                }
                _ => {
                    quote! { Some(&self.attributes.as_map()) }
                }
            }
        } else {
            quote! { None }
        }
    }

    fn build_print_trait_body(&self) -> proc_macro2::TokenStream {
        let tag_name = self.tag_name();
        let attrs = self.build_print_trait_attributes();

        match self.children_kind {
            ChildrenKind::String if self.options.children() && self.options.indent_children() => {
                quote! {
                    if self.children.is_empty() {
                        crate::prelude::print::open(#tag_name, #attrs, true, pretty, level, indent_size)
                    } else {
                        let mut res = crate::prelude::print::open(#tag_name, #attrs, false, pretty, level, indent_size);
                        res.push_str(&self.children);
                        res.push_str(&crate::prelude::print::close(#tag_name, pretty, level, indent_size));
                        res
                    }
                }
            }
            ChildrenKind::String if self.options.children() => {
                quote! {
                    if self.children.is_empty() {
                        crate::prelude::print::open(#tag_name, #attrs, true, pretty, level, indent_size)
                    } else {
                        let mut res = crate::prelude::print::open(#tag_name, #attrs, false, false, level, indent_size);
                        res.push_str(&self.children);
                        res.push_str(&crate::prelude::print::close(#tag_name, false, level, indent_size));
                        if pretty {
                            crate::prelude::print::indent(level, indent_size, res)
                        } else {
                            res
                        }
                    }
                }
            }
            ChildrenKind::Single if self.options.children() => {
                quote! {
                    let content = self.children.print(pretty, level + 1, indent_size);
                    if content.is_empty() {
                        crate::prelude::print::open(#tag_name, #attrs, true, pretty, level, indent_size)
                    } else {
                        let mut res = crate::prelude::print::open(#tag_name, #attrs, false, pretty, level, indent_size);
                        res.push_str(&content);
                        res.push_str(&crate::prelude::print::close(#tag_name, pretty, level, indent_size));
                        res
                    }
                }
            }
            ChildrenKind::List(_) if self.options.children() => {
                quote! {
                    if self.children.is_empty() {
                        crate::prelude::print::open(#tag_name, #attrs, true, pretty, level, indent_size)
                    } else {
                        let mut res = crate::prelude::print::open(#tag_name, #attrs, false, pretty, level, indent_size);
                        for child in self.children.iter() {
                            res.push_str(&child.print(pretty, level + 1, indent_size));
                        }
                        res.push_str(&crate::prelude::print::close(#tag_name, pretty, level, indent_size));
                        res
                    }
                }
            }
            _ => {
                let close_empty = self.options.close_empty.unwrap_or(true);
                quote! {
                    crate::prelude::print::open(#tag_name, #attrs, #close_empty, pretty, level, indent_size)
                }
            }
        }
    }

    fn build_print_trait_head(&self) -> proc_macro2::TokenStream {
        let struct_ident = &self.struct_ident;
        match self.struct_generic {
            Some(ref gen) => quote! {
                impl<#gen> crate::prelude::print::Print for #struct_ident<#gen>
            },
            None => quote! {
                impl crate::prelude::print::Print for #struct_ident
            },
        }
    }

    fn build_print_trait(&self) -> proc_macro2::TokenStream {
        let head = self.build_print_trait_head();
        let printing = self.build_print_trait_body();

        quote! {
            #head {
                fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
                    #printing
                }
            }
        }
    }

    fn build_display_trait_head(&self) -> proc_macro2::TokenStream {
        let struct_ident = &self.struct_ident;
        match self.struct_generic {
            Some(ref gen) => quote! {
                impl<#gen> std::fmt::Display for #struct_ident<#gen>
            },
            None => quote! {
                impl std::fmt::Display for #struct_ident
            },
        }
    }

    fn build_display_trait(&self) -> proc_macro2::TokenStream {
        let head = self.build_display_trait_head();

        quote! {
            #head {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    use crate::prelude::print::Print;

                    f.write_str(self.dense_print().as_str())
                }
            }
        }
    }

    pub(crate) fn build(self) -> proc_macro2::TokenStream {
        let print_trait = self.build_print_trait();
        let display_trait = self.build_display_trait();

        quote! {
            #print_trait
            #display_trait
        }
    }
}
