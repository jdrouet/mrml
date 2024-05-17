use darling::FromDeriveInput;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(FromDeriveInput)]
#[darling(attributes(mrml_json), forward_attrs(allow, doc, cfg))]
struct Opts {
    tag: Option<String>,
    tag_field: Option<String>,
}

struct Generator {
    origin_ident: Ident,
    origin_generic: Option<Ident>,
    visitor_ident: Ident,
    has_attributes: bool,
    has_children: bool,
    options: Opts,
}

impl From<(DeriveInput, Opts)> for Generator {
    fn from((ast, options): (DeriveInput, Opts)) -> Self {
        let origin_ident = ast.ident.clone();
        let origin_generic = crate::helper::get_generics(&ast);
        let visitor_ident = syn::Ident::new(&format!("{origin_ident}Visitor"), origin_ident.span());
        let has_attributes = crate::helper::get_attributes_field(&ast).is_some();
        let has_children = crate::helper::get_children_field(&ast).is_some();

        Self {
            origin_ident,
            origin_generic,
            visitor_ident,
            has_attributes,
            has_children,
            options,
        }
    }
}

impl Generator {
    fn build_serializer_trait_impl(&self) -> proc_macro2::TokenStream {
        let origin_ident = &self.origin_ident;
        match self.origin_generic {
            Some(ref gen) => quote! {
                impl<#gen: serde::Serialize> serde::Serialize for #origin_ident<#gen>
            },
            None => quote! {
                impl serde::Serialize for #origin_ident
            },
        }
    }

    fn build_serializer_set_type(&self) -> proc_macro2::TokenStream {
        if let Some(ref tag) = self.options.tag {
            let element_ident = syn::Ident::new(tag, self.origin_ident.span());
            quote! {
                map.serialize_entry("type", #element_ident)?;
            }
        } else if let Some(ref tag) = self.options.tag_field {
            let element_ident = syn::Ident::new(tag, self.origin_ident.span());
            quote! {
                map.serialize_entry("type", self.#element_ident.as_str())?;
            }
        } else {
            panic!("the tag or tag_field option should be set");
        }
    }

    fn build_serializer_fields_count(&self) -> usize {
        if self.has_attributes && self.has_children {
            3
        } else if self.has_attributes || self.has_children {
            2
        } else {
            1
        }
    }

    fn build_serializer_set_attributes(&self) -> proc_macro2::TokenStream {
        if self.has_attributes {
            quote! {
                if !self.attributes.is_empty() {
                    map.serialize_entry("attributes", &self.attributes)?;
                }
            }
        } else {
            quote! {}
        }
    }

    fn build_serializer_set_children(&self) -> proc_macro2::TokenStream {
        if self.has_children {
            quote! {
                if !self.children.is_empty() {
                    map.serialize_entry("children", &self.children)?;
                }
            }
        } else {
            quote! {}
        }
    }

    fn build_serializer(&self) -> proc_macro2::TokenStream {
        let trait_impl = self.build_serializer_trait_impl();
        let set_type = self.build_serializer_set_type();
        let fields = self.build_serializer_fields_count();
        let attributes = self.build_serializer_set_attributes();
        let children = self.build_serializer_set_children();

        quote! {
            use serde::ser::SerializeMap;

            #trait_impl {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    let mut map = serializer.serialize_map(Some(#fields))?;
                    #set_type
                    #attributes
                    #children
                    map.end()
                }
            }
        }
    }

    fn build_deserializer_visitor_struct(&self) -> proc_macro2::TokenStream {
        let visitor_ident = &self.visitor_ident;
        match self.origin_generic {
            Some(ref gen) => quote! {
                struct #visitor_ident<#gen> {
                    _marker: std::marker::PhantomData<#gen>,
                }

                impl<T> Default for #visitor_ident<#gen> {
                    fn default() -> Self {
                        Self {
                            _marker: std::marker::PhantomData,
                        }
                    }
                }
            },
            None => quote! {
                #[derive(Default)]
                struct #visitor_ident;
            },
        }
    }

    fn build_deserializer_visitor_trait(&self) -> proc_macro2::TokenStream {
        let visitor_ident = &self.visitor_ident;
        match self.origin_generic {
            Some(ref gen) => quote! {
                impl<'de, #gen: serde::Deserialize<'de>> serde::de::Visitor<'de> for #visitor_ident<#gen>
            },
            None => quote! {
                impl<'de> serde::de::Visitor<'de> for #visitor_ident
            },
        }
    }

    fn build_deserializer_visitor_type_value(&self) -> proc_macro2::TokenStream {
        let struct_ident = &self.origin_ident;
        match self.origin_generic {
            Some(ref gen) => quote! {
                type Value = #struct_ident<#gen>;
            },
            None => quote! {
                type Value = #struct_ident;
            },
        }
    }

    fn build_deserializer_trait(&self) -> proc_macro2::TokenStream {
        let struct_ident = &self.origin_ident;
        let visitor_ident = &self.visitor_ident;
        match self.origin_generic {
            Some(ref gen) => quote! {
                impl<'de, #gen: serde::Deserialize<'de>> serde::Deserialize<'de> for #struct_ident<#gen> {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        deserializer.deserialize_map(#visitor_ident::<#gen>::default())
                    }
                }
            },
            None => quote! {
                impl<'de> serde::Deserialize<'de> for #struct_ident {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        deserializer.deserialize_map(#visitor_ident::default())
                    }
                }
            },
        }
    }

    fn build_deserializer_set_tag(&self) -> proc_macro2::TokenStream {
        if let Some(ref tag) = self.options.tag {
            let element_ident = syn::Ident::new(tag, self.origin_ident.span());
            quote! {
                let value = access.next_value::<String>()?;
                if value != #element_ident {
                    return Err(M::Error::custom(format!(
                        "expected type to equal {}, found {}",
                        #element_ident,
                        value,
                    )));
                }
            }
        } else if let Some(ref tag_field) = self.options.tag_field {
            let tag_field = syn::Ident::new(tag_field, self.origin_ident.span());
            quote! {
                result.#tag_field = access.next_value::<String>()?;
            }
        } else {
            panic!("the tag or tag_field option should be set");
        }
    }

    fn build_deserializer_check_empty_tag(&self) -> proc_macro2::TokenStream {
        match self.options.tag_field {
            Some(ref tag_field) => {
                let tag_field = syn::Ident::new(tag_field, self.origin_ident.span());
                quote! {
                    if result.#tag_field.is_empty() {
                        return Err(M::Error::missing_field("type"));
                    }
                }
            }
            None => quote! {},
        }
    }

    fn build_deserializer_set_attributes(&self) -> proc_macro2::TokenStream {
        if self.has_attributes {
            quote! {
                else if key == "attributes" {
                    result.attributes = access.next_value()?;
                }
            }
        } else {
            quote! {}
        }
    }

    fn build_deserializer_set_children(&self) -> proc_macro2::TokenStream {
        if self.has_children {
            quote! {
                else if key == "children" {
                    result.children = access.next_value()?;
                }
            }
        } else {
            quote! {}
        }
    }

    fn build_deserializer_fields_and_formatter(
        &self,
    ) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
        if self.has_attributes && self.has_children {
            (
                quote! { const FIELDS: [&str; 3] = ["type", "attributes", "children"]; },
                quote! { formatter.write_str("an map with properties type, attributes and children") },
            )
        } else if self.has_attributes {
            (
                quote! { const FIELDS: [&str; 2] = ["type", "attributes"]; },
                quote! { formatter.write_str("an map with properties type and attributes") },
            )
        } else if self.has_children {
            (
                quote! { const FIELDS: [&str; 2] = ["type", "children"]; },
                quote! { formatter.write_str("an map with properties type and children") },
            )
        } else {
            (
                quote! { const FIELDS: [&str; 1] = ["type"]; },
                quote! { formatter.write_str("an map with type property") },
            )
        }
    }

    fn build_deserializer(&self) -> proc_macro2::TokenStream {
        let visitor_struct = self.build_deserializer_visitor_struct();
        let visitor_impl = self.build_deserializer_visitor_trait();
        let visitor_type_value = self.build_deserializer_visitor_type_value();
        let deserializer_impl = self.build_deserializer_trait();

        let set_tag = self.build_deserializer_set_tag();
        let tag_empty_check = self.build_deserializer_check_empty_tag();

        let set_attributes = self.build_deserializer_set_attributes();
        let set_children = self.build_deserializer_set_children();
        let (fields, formatter) = self.build_deserializer_fields_and_formatter();

        quote! {
            use serde::de::{Error, MapAccess};

            #fields

            #visitor_struct

            #visitor_impl {
                #visitor_type_value

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    #formatter
                }

                fn visit_map<M>(mut self, mut access: M) -> Result<Self::Value, M::Error>
                where
                    M: MapAccess<'de>,
                {
                    let mut result = Self::Value::default();
                    while let Some(key) = access.next_key::<String>()? {
                        if key == "type" {
                            #set_tag
                        }
                        #set_attributes
                        #set_children
                        else {
                            return Err(M::Error::unknown_field(&key, &FIELDS));
                        }
                    }
                    #tag_empty_check
                    Ok(result)
                }
            }

            #deserializer_impl
        }
    }

    fn build(self) -> proc_macro::TokenStream {
        let serializer = self.build_serializer();
        let deserializer = self.build_deserializer();

        quote! {
            #serializer
            #deserializer
        }
        .into()
    }
}

pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    let opts = Opts::from_derive_input(&ast).expect("Wrong options");

    Generator::from((ast, opts)).build()
}
