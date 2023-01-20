use common_macros::{get_attributes_kind, get_children_kind, AttributesKind, ChildrenKind};
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[derive(FromDeriveInput)]
#[darling(attributes(mrml_parse), forward_attrs(allow, doc, cfg))]
struct Opts {
    child_comment: Option<bool>,
    child_text: Option<bool>,
    child_element: Option<bool>,
}

impl Opts {
    fn child_comment(&self) -> bool {
        self.child_comment.unwrap_or(true)
    }
    fn child_text(&self) -> bool {
        self.child_text.unwrap_or(true)
    }
    fn child_element(&self) -> bool {
        self.child_element.unwrap_or(true)
    }
}

fn create_attribute(ast: &DeriveInput) -> proc_macro2::TokenStream {
    match get_attributes_kind(ast) {
        AttributesKind::None => quote! {},
        AttributesKind::Map => quote! {
            attributes: Map<String, String>,
        },
        AttributesKind::Struct(ident) => {
            let ident = Ident::new(&format!("{ident}Builder"), ident.span());
            quote! {
                attributes: #ident,
            }
        }
    }
}

fn create_attribute_new(ast: &DeriveInput) -> proc_macro2::TokenStream {
    match get_attributes_kind(ast) {
        AttributesKind::None => quote! {},
        AttributesKind::Map => quote! {
            attributes: Map::default(),
        },
        AttributesKind::Struct(_) => quote! {
            attributes: Default::default(),
        },
    }
}

fn create_attribute_build(ast: &DeriveInput) -> proc_macro2::TokenStream {
    match get_attributes_kind(ast) {
        AttributesKind::None => quote! {},
        AttributesKind::Map => quote! {
            attributes: self.attributes,
        },
        AttributesKind::Struct(_) => quote! {
            attributes: self.attributes.build()?,
        },
    }
}

fn create_parse_attribute(ast: &DeriveInput) -> proc_macro2::TokenStream {
    match get_attributes_kind(ast) {
        AttributesKind::None => quote! {},
        AttributesKind::Map => quote! {
            fn parse_attribute<'a>(
                &mut self,
                name: xmlparser::StrSpan<'a>,
                value: xmlparser::StrSpan<'a>,
            ) -> Result<(), crate::prelude::parse::Error> {
                self.attributes.insert(name.to_string(), value.to_string());
                Ok(())
            }
        },
        AttributesKind::Struct(_) => quote! {
            fn parse_attribute<'a>(
                &mut self,
                name: xmlparser::StrSpan<'a>,
                value: xmlparser::StrSpan<'a>,
            ) -> Result<(), crate::prelude::parse::Error> {
                self.attributes.insert(name, value)
            }
        },
    }
}

fn create_children(ast: &DeriveInput) -> proc_macro2::TokenStream {
    match get_children_kind(ast) {
        ChildrenKind::List(ty) => quote! {
            children: Vec<#ty>,
        },
        ChildrenKind::String => quote! {
            children: String,
        },
        _ => quote! {},
    }
}

fn create_children_new(ast: &DeriveInput) -> proc_macro2::TokenStream {
    match get_children_kind(ast) {
        ChildrenKind::None => quote! {},
        _ => quote! {
            children: Default::default(),
        },
    }
}

fn create_children_build(ast: &DeriveInput) -> proc_macro2::TokenStream {
    match get_children_kind(ast) {
        ChildrenKind::None => quote! {},
        _ => quote! {
            children: self.children,
        },
    }
}

fn create_parse_child_comment(ast: &DeriveInput, opts: &Opts) -> proc_macro2::TokenStream {
    match get_children_kind(ast) {
        ChildrenKind::List(_) if opts.child_comment() => quote! {
            fn parse_child_comment(&mut self, value: xmlparser::StrSpan) -> Result<(), crate::prelude::parse::Error> {
                self.children
                    .push(crate::comment::Comment::from(value.as_str()).into());
                Ok(())
            }
        },
        _ => quote! {},
    }
}

fn create_parse_child_text(ast: &DeriveInput, opts: &Opts) -> proc_macro2::TokenStream {
    match get_children_kind(ast) {
        ChildrenKind::String if opts.child_text() => quote! {
            fn parse_child_text(&mut self, value: xmlparser::StrSpan) -> Result<(), crate::prelude::parse::Error> {
                self.children = value.to_string();
                Ok(())
            }
        },
        ChildrenKind::List(_) if opts.child_text() => quote! {
            fn parse_child_text(&mut self, value: xmlparser::StrSpan) -> Result<(), crate::prelude::parse::Error> {
                self.children.push(crate::text::Text::from(value.as_str()).into());
                Ok(())
            }
        },
        _ => quote! {},
    }
}

fn create_parse_child_element(ast: &DeriveInput, opts: &Opts) -> proc_macro2::TokenStream {
    match get_children_kind(ast) {
        ChildrenKind::List(ty) if opts.child_element() => quote! {
            fn parse_child_element<'a>(
                &mut self,
                tag: xmlparser::StrSpan<'a>,
                tokenizer: &mut xmlparser::Tokenizer<'a>,
            ) -> Result<(), crate::prelude::parse::Error> {
                use crate::prelude::parse::Parsable;
                self.children.push(<#ty>::parse(tag, tokenizer)?);
                Ok(())
            }
        },
        _ => quote! {},
    }
}

pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    let opts = Opts::from_derive_input(&ast).expect("Wrong options");

    let origin_ident = &ast.ident;
    let parser_name = format!("{origin_ident}Parser");
    let parser_ident = Ident::new(&parser_name, origin_ident.span());

    let attributes = create_attribute(&ast);
    let attributes_build = create_attribute_build(&ast);
    let attributes_new = create_attribute_new(&ast);
    let parse_attribute = create_parse_attribute(&ast);

    let children = create_children(&ast);
    let children_build = create_children_build(&ast);
    let children_new = create_children_new(&ast);
    let parse_child_comment = create_parse_child_comment(&ast, &opts);
    let parse_child_element = create_parse_child_element(&ast, &opts);
    let parse_child_text = create_parse_child_text(&ast, &opts);

    quote! {
        #[derive(Debug)]
        struct #parser_ident {
            #attributes
            #children
        }

        impl #parser_ident {
            fn new() -> Self {
                Self {
                    #attributes_new
                    #children_new
                }
            }
        }

        impl crate::prelude::parse::Parser for #parser_ident {
            type Output = #origin_ident;

            fn build(self) -> Result<Self::Output, crate::prelude::parse::Error> {
                Ok(#origin_ident {
                    #attributes_build
                    #children_build
                })
            }

            #parse_attribute
            #parse_child_comment
            #parse_child_element
            #parse_child_text
        }

        impl crate::prelude::parse::Parsable for #origin_ident {
            fn parse(_tag: xmlparser::StrSpan, tokenizer: &mut xmlparser::Tokenizer) -> Result<Self, crate::prelude::parse::Error> {
                use crate::prelude::parse::Parser;
                #parser_ident::new().parse(tokenizer)?.build()
            }
        }
    }
    .into()
}
