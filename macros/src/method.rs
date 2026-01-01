use quote::quote;
use syn::{DataEnum, DataStruct, Ident, spanned::Spanned};

use crate::MacroResult;

const NO_METHOD_FIELD: &'static str = "No Method field on this struct";
const METHOD: &'static str = "method";
const UNION: &'static str = "Union Data type is not supported";

pub fn method_trait_derive_struct(name: &Ident, input: &DataStruct) -> MacroResult {
    //check for method field
    let method = input
        .fields
        .iter()
        .any(|field| field.ident == Some(Ident::new(METHOD, field.ident.span())));
    if !method {
        return Err(NO_METHOD_FIELD);
    };
    Ok(quote! {
        use class::GetMethod;
        impl GetMethod for #name{
            fn get_method(&self) -> Method {
                self.method.clone()
            }
        }
    })
}
pub fn method_trait_derive_enum(name: &Ident, _enum: &DataEnum) -> MacroResult {
    let mut match_arms = quote! {};
    for variant in _enum.variants.iter() {
        let v_ident = &variant.ident;
        match_arms.extend(quote! {
            #name::#v_ident(arg) => arg.get_method(),
        });
    }

    Ok(quote! {
        use class::GetMethod;
        impl GetMethod for #name{
            fn get_method(&self) -> Method {
                match self {
                    #match_arms
                }
            }
        }
    })
}

pub fn method_trait_derive_union() -> MacroResult {
    Err(UNION)
}
