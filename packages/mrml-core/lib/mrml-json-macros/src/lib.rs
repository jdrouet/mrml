mod element;
mod helper;

#[proc_macro_derive(MrmlJsonComponent, attributes(mrml_json))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    crate::element::derive(input)
}
