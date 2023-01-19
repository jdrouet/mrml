use darling::FromDeriveInput;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(FromDeriveInput)]
#[darling(attributes(mrml_json), forward_attrs(allow, doc, cfg))]
struct Opts {
    tag: Option<String>,
    tag_field: Option<String>,
}

fn create_serializer(ast: &DeriveInput, opts: &Opts) -> proc_macro2::TokenStream {
    let struct_ident = &ast.ident;
    let struct_generic = common_macros::get_generics(ast);

    let serialize_impl = match struct_generic {
        Some(gen) => quote! {
            impl<#gen: serde::Serialize> serde::Serialize for #struct_ident<#gen>
        },
        None => quote! {
            impl serde::Serialize for #struct_ident
        },
    };

    let tag = if let Some(ref tag) = opts.tag {
        let element_ident = syn::Ident::new(tag, struct_ident.span());
        quote! {
            map.serialize_entry("type", #element_ident)?;
        }
    } else if let Some(ref tag) = opts.tag_field {
        let element_ident = syn::Ident::new(tag, struct_ident.span());
        quote! {
            map.serialize_entry("type", self.#element_ident.as_str())?;
        }
    } else {
        panic!("the tag or tag_field option should be set");
    };

    let mut fields: usize = 1;

    let has_attributes = common_macros::get_attributes_field(ast).is_some();
    let attributes = if has_attributes {
        fields += 1;
        quote! {
            if !self.attributes.is_empty() {
                map.serialize_entry("attributes", &self.attributes)?;
            }
        }
    } else {
        quote! {}
    };

    let has_children = common_macros::get_children_field(ast).is_some();
    let children = if has_children {
        fields += 1;
        quote! {
            if !self.children.is_empty() {
                map.serialize_entry("children", &self.children)?;
            }
        }
    } else {
        quote! {}
    };

    quote! {
        use serde::ser::SerializeMap;

        #serialize_impl {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut map = serializer.serialize_map(Some(#fields))?;
                #tag
                #attributes
                #children
                map.end()
            }
        }
    }
}

fn create_deserialize_visitor_struct(
    visitor_ident: &syn::Ident,
    generic: &Option<syn::Ident>,
) -> proc_macro2::TokenStream {
    match generic {
        Some(gen) => quote! {
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

fn create_deserialize_visitor_impl(
    visitor_ident: &syn::Ident,
    generic: &Option<syn::Ident>,
) -> proc_macro2::TokenStream {
    match generic {
        Some(gen) => quote! {
            impl<'de, #gen: serde::Deserialize<'de>> serde::de::Visitor<'de> for #visitor_ident<#gen>
        },
        None => quote! {
            impl<'de> serde::de::Visitor<'de> for #visitor_ident
        },
    }
}

fn create_deserialize_visitor_type_value(
    struct_ident: &syn::Ident,
    generic: &Option<syn::Ident>,
) -> proc_macro2::TokenStream {
    match generic {
        Some(gen) => quote! {
            type Value = #struct_ident<#gen>;
        },
        None => quote! {
            type Value = #struct_ident;
        },
    }
}

fn create_deserialize_impl(
    struct_ident: &syn::Ident,
    visitor_ident: &syn::Ident,
    generic: &Option<syn::Ident>,
) -> proc_macro2::TokenStream {
    match generic {
        Some(gen) => quote! {
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

fn create_deserialize(ast: &DeriveInput, opts: &Opts) -> proc_macro2::TokenStream {
    let struct_ident = &ast.ident;
    let struct_generic = common_macros::get_generics(ast);
    let visitor_ident = syn::Ident::new(&format!("{struct_ident}Visitor"), struct_ident.span());

    let visitor_struct = create_deserialize_visitor_struct(&visitor_ident, &struct_generic);
    let visitor_impl = create_deserialize_visitor_impl(&visitor_ident, &struct_generic);
    let visitor_type_value = create_deserialize_visitor_type_value(struct_ident, &struct_generic);
    let deserializer_impl = create_deserialize_impl(struct_ident, &visitor_ident, &struct_generic);

    let has_attributes = common_macros::get_attributes_field(ast).is_some();
    let has_children = common_macros::get_children_field(ast).is_some();

    let set_tag = if let Some(ref tag) = opts.tag {
        let element_ident = syn::Ident::new(tag, struct_ident.span());
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
    } else if let Some(ref tag_field) = opts.tag_field {
        let tag_field = syn::Ident::new(tag_field, struct_ident.span());
        quote! {
            result.#tag_field = access.next_value::<String>()?;
        }
    } else {
        panic!("the tag or tag_field option should be set");
    };

    let tag_empty_check = match opts.tag_field {
        Some(ref tag_field) => {
            let tag_field = syn::Ident::new(tag_field, struct_ident.span());
            quote! {
                if result.#tag_field.is_empty() {
                    return Err(M::Error::missing_field("type"));
                }
            }
        }
        None => quote! {},
    };

    let set_attributes = if has_attributes {
        quote! {
            else if key == "attributes" {
                result.attributes = access.next_value()?;
            }
        }
    } else {
        quote! {}
    };
    let set_children = if has_children {
        quote! {
            else if key == "children" {
                result.children = access.next_value()?;
            }
        }
    } else {
        quote! {}
    };

    let (fields, formatter) = if has_attributes && has_children {
        (
            quote! { const FIELDS: [&str; 3] = ["type", "attributes", "children"]; },
            quote! { formatter.write_str("an map with properties type, attributes and children") },
        )
    } else if has_attributes {
        (
            quote! { const FIELDS: [&str; 2] = ["type", "attributes"]; },
            quote! { formatter.write_str("an map with properties type and attributes") },
        )
    } else if has_children {
        (
            quote! { const FIELDS: [&str; 2] = ["type", "children"]; },
            quote! { formatter.write_str("an map with properties type and children") },
        )
    } else {
        (
            quote! { const FIELDS: [&str; 1] = ["type"]; },
            quote! { formatter.write_str("an map with type property") },
        )
    };

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

pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    let opts = Opts::from_derive_input(&ast).expect("Wrong options");

    let serializer = create_serializer(&ast, &opts);
    let deserializer = create_deserialize(&ast, &opts);

    quote! {
        #serializer
        #deserializer
    }
    .into()
}
