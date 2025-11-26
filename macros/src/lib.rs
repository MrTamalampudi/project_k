use proc_macro::TokenStream;
use quote::quote;
use syn::{
    DeriveInput, Ident, ItemFn, Stmt, Token, parse::Parse, parse_macro_input,
    punctuated::Punctuated,
};

use crate::span::{span_trait_derive_enum, span_trait_derive_struct, span_trait_derive_union};

mod span;

pub(crate) type MacroResult = Result<proc_macro2::TokenStream, &'static str>;

struct Attributes {
    identifiers: Vec<Ident>,
}

impl Parse for Attributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let idents: Punctuated<Ident, Token![,]> =
            input.parse_terminated(Ident::parse, Token![,])?;
        Ok(Attributes {
            identifiers: idents.into_iter().collect(),
        })
    }
}

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

#[proc_macro_attribute]
pub fn pop_token(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let attributes = parse_macro_input!(attrs as Attributes);
    let mut func = parse_macro_input!(item as ItemFn);
    for attr in attributes.identifiers.iter().rev() {
        let stmt_token_stream = quote! {let #attr = _token_stack.pop().unwrap();}.into();
        let stmt = parse_macro_input!(stmt_token_stream as Stmt);
        func.block.stmts.insert(0, stmt);
    }
    return quote! {#func}.into();
}
