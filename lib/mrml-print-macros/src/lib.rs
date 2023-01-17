extern crate proc_macro;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Data, DataEnum, DataStruct,
    DeriveInput, Field, Fields, FieldsNamed, Path, Type, TypePath,
};

fn as_fields_named(input: &DataStruct) -> Option<&FieldsNamed> {
    if let Fields::Named(inner) = &input.fields {
        Some(inner)
    } else {
        None
    }
}

fn as_data_struct(ast: &DeriveInput) -> Option<&DataStruct> {
    if let Data::Struct(inner) = &(ast.data) {
        Some(inner)
    } else {
        None
    }
}

fn as_data_enum(ast: &DeriveInput) -> Option<&DataEnum> {
    if let Data::Enum(inner) = &(ast.data) {
        Some(inner)
    } else {
        None
    }
}

fn get_fields(ast: &DeriveInput) -> &Punctuated<Field, Comma> {
    as_data_struct(ast)
        .and_then(as_fields_named)
        .map(|f| &f.named)
        .expect("MrmlPrintComponent only supports structs.")
}

fn get_attributes_field(ast: &DeriveInput) -> Option<&Field> {
    get_fields(ast).into_iter().find(|f| {
        f.ident
            .as_ref()
            .map(|id| *id == "attributes")
            .unwrap_or(false)
    })
}

fn get_children_field(ast: &DeriveInput) -> Option<&Field> {
    as_data_struct(ast)
        .and_then(as_fields_named)
        .map(|f| &f.named)
        .expect("MrmlPrintComponent only supports structs.")
        .into_iter()
        .find(|f| {
            f.ident
                .as_ref()
                .map(|id| *id == "children")
                .unwrap_or(false)
        })
}

#[derive(FromDeriveInput)]
#[darling(attributes(mrml_print), forward_attrs(allow, doc, cfg))]
struct Opts {
    tag: Option<String>,
    close_empty: Option<bool>,
    indent_children: Option<bool>,
}

impl Opts {
    fn indent_children(&self) -> bool {
        self.indent_children.unwrap_or(true)
    }
}

fn as_path(field: &Field) -> Option<&Path> {
    match &field.ty {
        Type::Path(TypePath { path, .. }) => Some(path),
        _ => None,
    }
}

fn is_vec(path: &Path) -> bool {
    path.segments
        .first()
        // TODO make sure that it's a Vec<T>
        .map(|s| s.ident == "Vec")
        .unwrap_or(false)
}

fn is_option(path: &Path) -> bool {
    path.segments
        .first()
        // TODO make sure that it's a Option<String>
        .map(|s| s.ident == "Option")
        .unwrap_or(false)
}

fn is_option_string(path: &Path) -> bool {
    path.segments
        .first()
        // TODO make sure that it's a Option<String>
        .map(|s| s.ident == "Option")
        .unwrap_or(false)
}

fn is_map_string(path: &Path) -> bool {
    path.segments
        .first()
        // TODO make sure that it's a Map<String, String>
        .map(|s| s.ident == "Map")
        .unwrap_or(false)
}

fn print_attributes(ast: &DeriveInput) -> proc_macro2::TokenStream {
    if let Some(field) = get_attributes_field(ast) {
        match &field.ty {
            Type::Path(TypePath { path, .. }) if is_map_string(path) => {
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

#[derive(PartialEq, Eq)]
enum ChildrenKind {
    String { indent: bool },
    Single,
    List,
    None,
}

fn get_children_kind(ast: &DeriveInput, opts: &Opts) -> ChildrenKind {
    if let Some(field) = get_children_field(ast) {
        match &field.ty {
            Type::Path(TypePath { path, .. }) if path.is_ident("String") => ChildrenKind::String {
                indent: opts.indent_children(),
            },
            Type::Path(TypePath { path, .. }) if is_vec(path) => ChildrenKind::List,
            _ => ChildrenKind::Single,
        }
    } else {
        ChildrenKind::None
    }
}

fn impl_print(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let opts = Opts::from_derive_input(ast).expect("Wrong options");

    let tag_name = opts.tag.clone().unwrap_or_else(|| "NAME".into());
    let tag_name = Ident::new(tag_name.as_str(), Span::call_site());

    let attrs = print_attributes(ast);

    let printing = match get_children_kind(ast, &opts) {
        ChildrenKind::None => {
            let close_empty = opts.close_empty.unwrap_or(true);
            quote! {
                crate::prelude::print::open(#tag_name, #attrs, #close_empty, pretty, level, indent_size)
            }
        }
        ChildrenKind::String { indent: true } => {
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
        ChildrenKind::String { indent: false } => {
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
        ChildrenKind::Single => {
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
        ChildrenKind::List => {
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
    };

    quote! {
        impl crate::prelude::print::Print for #name {
            fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
                #printing
            }
        }
    }
}

fn impl_display(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                use crate::prelude::print::Print;

                f.write_str(self.dense_print().as_str())
            }
        }
    }
}

#[proc_macro_derive(MrmlPrintComponent, attributes(mrml_print))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    let print_impl = impl_print(&ast);
    let display_impl = impl_display(&ast);

    quote! {
        #print_impl
        #display_impl
    }
    .into()
}

#[proc_macro_derive(MrmlPrintAttributes)]
pub fn derive_attributes(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let fields = get_fields(&ast).iter().filter_map(|f| {
        match (&f.ident, as_path(f).map(is_option_string)) {
            (Some(ident), Some(true)) => Some(quote! {
                if let Some(ref value) = self.#ident {
                    res.insert(stringify!(#ident).to_string(), value.to_string());
                }
            }),
            (Some(ident), Some(false)) => Some(quote! {
                res.insert(stringify!(#ident).to_string(), self.#ident.to_string());
            }),
            _ => None,
        }
    });

    let res = quote! {
        impl #name {
            fn as_map(&self) -> crate::prelude::hash::Map<String, String> {
                let mut res = crate::prelude::hash::Map::new();
                #(#fields)*
                res
            }
        }
    };

    res.into()
}

#[proc_macro_derive(MrmlPrintChildren)]
pub fn derive_children(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    if let Some(data_enum) = as_data_enum(&ast) {
        derive_children_enum(&ast, data_enum).into()
    } else if let Some(data_struct) = as_data_struct(&ast) {
        derive_children_struct(&ast, data_struct).into()
    } else {
        panic!("MrmlPrintChildren only works with enums and structs.")
    }
}

fn derive_children_enum(ast: &DeriveInput, data_enum: &DataEnum) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let fields = data_enum
        .variants
        .iter()
        .map(|v| {
            let variant = &v.ident;
            quote! {
                #name::#variant(elt) => elt.print(pretty, level, indent_size),
            }
        })
        .collect::<proc_macro2::TokenStream>();

    quote! {
        impl crate::prelude::print::Print for #name {
            fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
                match self {
                    #fields
                }
            }
        }
    }
}

fn derive_children_struct(ast: &DeriveInput, data_struct: &DataStruct) -> proc_macro2::TokenStream {
    let name = &ast.ident;

    let fields =
        data_struct
            .fields
            .iter()
            .filter_map(|f| match (&f.ident, as_path(f).map(is_option)) {
                (Some(ident), Some(true)) => Some(quote! {
                    if let Some(ref value) = self.#ident {
                        res.push_str(&value.print(pretty, level, indent_size));
                    }
                }),
                (Some(ident), Some(false)) => Some(quote! {
                    res.push_str(&self.#ident.print(pretty, level, indent_size));
                }),
                _ => None,
            });

    quote! {
        impl crate::prelude::print::Print for #name {
            fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
                let mut res = String::new();
                #(#fields)*
                res
            }
        }
    }
}
