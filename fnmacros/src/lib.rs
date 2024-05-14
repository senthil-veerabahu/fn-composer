use proc_macro::TokenStream;
use std::{
    borrow::Borrow,
    fmt::Display,
    ops::Deref,
};

use quote::{quote, ToTokens};
use syn::{ItemFn, parse::Parse};

#[proc_macro_attribute]
pub fn composeable(attr: TokenStream, item: TokenStream) -> TokenStream {
    let tokenStreamClone = item.clone();
    let item_fn: ItemFn = syn::parse_macro_input!(tokenStreamClone);
    let mut asyncFn = item_fn.sig.asyncness.is_some();
    let inputArgs = item_fn.sig.inputs;
    let argLength = inputArgs.len();
    let fnName = item_fn.sig.ident.to_string();
    let fnReturnType = item_fn.sig.output;
    if (!asyncFn) {
        match fnReturnType {
            syn::ReturnType::Default => {}
            syn::ReturnType::Type(_, t) => {
                let x = t.deref();
                asyncFn = x.to_token_stream().to_string().starts_with("BoxFuture");
            }
        }
    }

    let binding = ("lifted_fn_".to_owned() + &fnName);
    let liftFnName = binding.deref();
    let liftFnIdent = syn::Ident::new(liftFnName, proc_macro2::Span::call_site());
    let asyncPrefix = "async_".to_owned();
    let asyncFnIdent = syn::Ident::new(
        (asyncPrefix + fnName.deref()).deref(),
        proc_macro2::Span::call_site(),
    );
    let (
        returnType,
        underlyingLiftFnName,
        funGen,
        returnTypeIdent,
        retGen,
        underlyingLiftFnNameIdent,
    ) = if (asyncFn) {
        let returnType = if argLength == 2 {
            "BoxedFutAppTwoArgFnOnce"
        } else {
            "BoxedFutAppFnOnce"
        };
        let underlyingLiftFnName = if argLength == 2 {
            "liftTwoArgAsync"
        } else {
            "liftAsync"
        };
        let funGen = if argLength == 2 {
            syn::parse_str::<syn::Generics>(
                "<'a,A,B,C,F:Fn(A,B)->BoxFuture<'a,Result<C, FnError>> + 'a + Send +Sync>",
            )
            .ok()
            .unwrap()
        } else {
            syn::parse_str::<syn::Generics>(
                "<'a,A,B, F:Fn(A)->BoxFuture<'a,Result<B, FnError>> + 'a + Send +Sync>",
            )
            .ok()
            .unwrap()
        };
        let returnTypeIdent = syn::Ident::new(returnType, proc_macro2::Span::call_site());
        let underlyingLiftFnNameIdent =
            syn::Ident::new(underlyingLiftFnName, proc_macro2::Span::call_site());
        let retGen = if argLength == 2 {
            syn::parse_str::<syn::Generics>("<'a,A,B,C>").ok().unwrap()
        } else {
            syn::parse_str::<syn::Generics>("<'a,A,B>").ok().unwrap()
        };

        (
            returnType,
            underlyingLiftFnName,
            funGen,
            returnTypeIdent,
            retGen,
            underlyingLiftFnNameIdent,
        )
    } else {
        let returnType = if argLength == 2 {
            "BoxedFutAppTwoArgFnOnce"
        } else {
            "BoxedAppFn"
        };
        let underlyingLiftFnName = if argLength == 2 {
            "liftTwoArgAsync"
        } else {
            "lift"
        };
        let funGen = if argLength == 2 {
            syn::parse_str::<syn::Generics>(
                "<'a,A,B,C,F:Fn(A,B)->Result<C, FnError> + Send +Sync + 'a>",
            )
            .ok()
            .unwrap()
        } else {
            syn::parse_str::<syn::Generics>(
                "<'a, A,B,F:Fn(A)->Result<B, FnError>  + Send +Sync + 'a>",
            )
            .ok()
            .unwrap()
        };
        let retGen = if argLength == 2 {
            syn::parse_str::<syn::Generics>("<'a, A,B,C>").ok().unwrap()
        } else {
            syn::parse_str::<syn::Generics>("<'a,A,B>").ok().unwrap()
        };
        let returnTypeIdent = syn::Ident::new(returnType, proc_macro2::Span::call_site());
        let underlyingLiftFnNameIdent =
            syn::Ident::new(underlyingLiftFnName, proc_macro2::Span::call_site());
        (
            returnType,
            underlyingLiftFnName,
            funGen,
            returnTypeIdent,
            retGen,
            underlyingLiftFnNameIdent,
        )
    };

    if (asyncFn) {
        let mut tokens: proc_macro2::TokenStream = quote! {
            use fnutils::*;
            pub fn #liftFnIdent #funGen(f: F)  -> #returnTypeIdent #retGen{
                #underlyingLiftFnNameIdent(f)
            }

            pub fn #asyncFnIdent ()  -> bool{
                #asyncFn
            }
        };
        let mut toeknStream: proc_macro::TokenStream = tokens.into();
        toeknStream.extend(item.into_iter());
        toeknStream
    } else {
        let mut tokens: proc_macro2::TokenStream = quote! {
            use fnutils::*;
            pub fn #liftFnIdent #funGen(f: F)  -> #returnTypeIdent #retGen{
                #underlyingLiftFnNameIdent(f)
            }

            pub fn #asyncFnIdent ()  -> bool{
                #asyncFn
            }
        };
        let mut toeknStream: proc_macro::TokenStream = tokens.into();
        toeknStream.extend(item.into_iter());
        toeknStream
    }
}
