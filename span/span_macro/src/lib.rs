use proc_macro::TokenStream;
use quote::quote;
use syn::{DataEnum, DataStruct, DeriveInput, Ident, parse_macro_input, spanned::Spanned};

const NO_SPAN_FIELD: &'static str = "No Span field on this struct";
const SPAN: &'static str = "span";
const UNION: &'static str = "Union Data type is not supported";

type MacroResult = Result<proc_macro2::TokenStream, &'static str>;

#[proc_macro_derive(Span)]
pub fn span_data_trait_derive(input: TokenStream) -> TokenStream {
    let d_input = parse_macro_input!(input as DeriveInput);
    let name = d_input.ident;
    let impl_ = match d_input.data {
        syn::Data::Struct(struct_data) => span_trait_derive_struct(&name, &struct_data),
        syn::Data::Enum(enum_data) => span_trait_derive_enum(&name, &enum_data),
        syn::Data::Union(_) => Err(UNION),
    };
    if let Err(e) = impl_ {
        return quote! {compile_error!(#e);}.into();
    }
    impl_.unwrap().into()
}

fn span_trait_derive_struct(name: &Ident, input: &DataStruct) -> MacroResult {
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
fn span_trait_derive_enum(name: &Ident, _enum: &DataEnum) -> MacroResult {
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
