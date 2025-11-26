use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

use crate::span::{span_trait_derive_enum, span_trait_derive_struct, span_trait_derive_union};

mod span;

pub(crate) type MacroResult = Result<proc_macro2::TokenStream, &'static str>;

///Derive macro generating an impl of trait SpanData
#[proc_macro_derive(Span)]
pub fn span_data_trait_derive(input: TokenStream) -> TokenStream {
    let d_input = parse_macro_input!(input as DeriveInput);
    let name = d_input.ident;
    let impl_ = match d_input.data {
        syn::Data::Struct(struct_data) => span_trait_derive_struct(&name, &struct_data),
        syn::Data::Enum(enum_data) => span_trait_derive_enum(&name, &enum_data),
        syn::Data::Union(_) => span_trait_derive_union(),
    };
    if let Err(e) = impl_ {
        return quote! {compile_error!(#e);}.into();
    }
    impl_.unwrap().into()
}
