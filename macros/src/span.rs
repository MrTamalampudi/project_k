use quote::quote;
use syn::{DataEnum, DataStruct, Ident, spanned::Spanned};

use crate::MacroResult;

const NO_SPAN_FIELD: &'static str = "No Span field on this struct";
const SPAN: &'static str = "span";
const UNION: &'static str = "Union Data type is not supported";

pub fn span_trait_derive_struct(name: &Ident, input: &DataStruct) -> MacroResult {
    //check for span field
    let span = input
        .fields
        .iter()
        .any(|field| field.ident == Some(Ident::new(SPAN, field.ident.span())));
    if !span {
        return Err(NO_SPAN_FIELD);
    };
    Ok(quote! {
        impl SpanData for #name{
            fn get_span(&self) -> Span {
                self.span.clone()
            }
            fn set_span(&mut self,span:Span) {
                self.span = span;
            }
        }
    })
}
pub fn span_trait_derive_enum(name: &Ident, _enum: &DataEnum) -> MacroResult {
    let mut match_arms = quote! {};
    let mut match_set_arms = quote! {};
    for variant in _enum.variants.iter() {
        let v_ident = &variant.ident;
        match_arms.extend(quote! {
            #name::#v_ident(arg) => arg.get_span(),
        });
        match_set_arms.extend(quote! {
            #name::#v_ident(arg) => arg.set_span(span),
        });
    }

    Ok(quote! {
        impl SpanData for #name{
            fn get_span(&self) -> Span {
                match self {
                    #match_arms
                }
            }
            fn set_span(&mut self,span:Span){
                match self {
                    #match_set_arms
                }
            }
        }
    })
}

pub fn span_trait_derive_union() -> MacroResult {
    Err(UNION)
}
