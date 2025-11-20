#[macro_use]
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DataEnum, DataStruct, DeriveInput, Ident, parse_macro_input, spanned::Spanned};

#[proc_macro_derive(Span)]
pub fn span_trait_derive(input: TokenStream) -> TokenStream {
    let d_input = parse_macro_input!(input as DeriveInput);
    let name = d_input.ident;
    match d_input.data {
        syn::Data::Struct(struct_data) => span_trait_derive_struct(&struct_data),
        syn::Data::Enum(enum_data) => span_trait_derive_enum(&enum_data),
        syn::Data::Union(_) => span_trait_derive_union(),
    }
}

fn span_trait_derive_struct(input: &DataStruct) -> TokenStream {
    //check for span field
    let span = input
        .fields
        .iter()
        .any(|field| field.ident == Some(Ident::new("span", field.ident.span())));
    if !span {
        return quote! {
            compile_error!("span field doesnt existdddddddddddddddd");
        }
        .into();
    };
    TokenStream::new()
}
fn span_trait_derive_enum(input: &DataEnum) -> TokenStream {
    TokenStream::new()
}
fn span_trait_derive_union() -> TokenStream {
    TokenStream::new()
}
