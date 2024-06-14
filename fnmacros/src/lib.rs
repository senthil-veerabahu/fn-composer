use proc_macro::TokenStream;
use std::{borrow::Borrow, fmt::Display, ops::Deref};
use std::fmt::format;

use quote::{quote, ToTokens};
use syn::{ItemFn, parse::Parse};

fn generate_return_type_param(index:u8) -> String{
    format!("T{index}")
}

fn generate_generics_parameters(count:u8)-> String{
    let mut result:String = "".to_owned();
    for i in 1..=count {
        result.push_str(
            format!("T{},", i.to_string().as_str()).as_str()
        );
    }
    result
}

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
    ) = /*if (asyncFn)*/ {
        let returnType = if asyncFn {
            "BoxedAsyncFn".to_owned() + argLength.to_string().as_str()
        }else{
            "BoxedFn".to_owned() + argLength.to_string().as_str()
        };
        let underlyingLiftFnName = if(asyncFn) {
            "lift_async_fn".to_owned() + argLength.to_string().as_str()
        }else{
            "lift_sync_fn".to_owned() + argLength.to_string().as_str()
        };
        let gen_type_params = generate_generics_parameters((argLength + 1) as u8);
        let fun_arg_params = generate_generics_parameters((argLength) as u8);
        let return_type_param = generate_return_type_param((argLength + 1) as u8);
        let funGen = if(asyncFn) {
            syn::parse_str::<syn::Generics>(
                format!("<'a, {gen_type_params} F:Fn({fun_arg_params})->BoxFuture<'a,Result<{return_type_param}, FnError>> + 'a + Send +Sync>", ).as_str()
            ).ok()
                .unwrap()
        }else{
            syn::parse_str::<syn::Generics>(
                format!("<'a, {gen_type_params} F:Fn({fun_arg_params})->Result<{return_type_param}, FnError> + Send +Sync + 'a>").as_str()
            ).ok().unwrap()
        };
            
        let returnTypeIdent = syn::Ident::new(returnType.as_str(), proc_macro2::Span::call_site());
        let underlyingLiftFnNameIdent =
            syn::Ident::new(underlyingLiftFnName.as_str(), proc_macro2::Span::call_site());
        let retGen = syn::parse_str::<syn::Generics>(format!("<'a,{gen_type_params}>").as_str()).ok().unwrap();

        (
            returnType,
            underlyingLiftFnName,
            funGen,
            returnTypeIdent,
            retGen,
            underlyingLiftFnNameIdent,
        )
    };
    
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
