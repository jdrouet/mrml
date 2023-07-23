extern crate proc_macro;

mod attributes;
mod children;
mod element;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MrmlPrintComponent, attributes(mrml_print))]
pub fn derive_element(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    let opts = element::Opts::from_derive_input(&ast).expect("Wrong options");

    element::Generator::from((ast, opts)).build().into()
}

#[proc_macro_derive(MrmlPrintAttributes)]
pub fn derive_attributes(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    attributes::Generator::from(ast).build().into()
}

#[proc_macro_derive(MrmlPrintChildren)]
pub fn derive_children(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    children::Generator::from(ast).build().into()
}
