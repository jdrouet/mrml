mod children;

#[proc_macro_derive(MrmlChildren)]
pub fn derive_children(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    crate::children::derive(input)
}
