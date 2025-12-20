//! # function-compose-proc-macros
//! This crate contains proc macro to mark a function as composeable. 
//! composeable  functions can be composed using [compose!](function_compose::compose!) macrof macro
//! 
//! ```
//! use function_compose::composeable;
//!#[composeable()]
//!pub fn add_10(a: i32) -> Result<i32, String> {
//!    Ok(a + 10)
//!}
//! ```
//! 
use proc_macro::TokenStream;

use std::fmt::Formatter;
use std::{fmt::Display, ops::Deref};

use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::ParseStream;
use syn::{parse::Parse, Expr, FnArg, ItemFn, ReturnType, Token, Type};

use crate::OptionalRetry::SomeRetry;

fn generate_return_type_param(index: u8) -> String {
    format!("T{index}")
}

mod keyword {
    syn::custom_keyword!(retry);
}

fn generate_generics_parameters(count: u8) -> String {
    let mut result: String = "".to_owned();
    for i in 1..=count {
        result.push_str(format!("T{},", i.to_string().as_str()).as_str());
    }
    result
}

#[proc_macro_attribute]
pub fn retry(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    panic!()
}

struct Retry {
    strategy: Expr,
}

struct FunctionArgs<'a> {
    args: Vec<&'a FnArg>,
}

struct FunctionMutArgs<'a> {
    args: Vec<&'a FnArg>,
}

impl<'a> ToTokens for FunctionArgs<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.args.iter().for_each(|arg| {
            match *arg {
                FnArg::Receiver(_) => {}
                FnArg::Typed(t) => {
                    let ident = &t.pat;
                    let ty = t.ty.as_ref();
                    let token_stream = match ty {
                        Type::Reference(reference) => {
                            if reference.mutability.is_some() {
                                quote! {
                                   #[allow(unused)] &mut #ident,
                                }
                            } else {
                                quote! {
                                    #[allow(unused)] & #ident,
                                }
                            }
                        }
                        _ => {
                            quote! {
                                 #[allow(unused)] #ident,
                            }
                        }
                    };

                    tokens.append_all(token_stream.into_iter());
                    /*let mut toeknStream: proc_macro::TokenStream = tokeStream.into();
                    toeknStream.extend(tokens.into_iter())*/
                }
            }
        });
    }
}

impl<'a> ToTokens for FunctionMutArgs<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.args.iter().for_each(|arg| {
            let token_stream = quote! {
                       #[allow(unused)] mut #arg,
            };
            tokens.append_all(token_stream.into_iter());
        });
    }
}

enum OptionalRetry {
    SomeRetry(Retry),
    NoRetry,
}

impl Display for Retry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "test")
    }
}

impl Parse for OptionalRetry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(keyword::retry) {
            let _ = input.parse::<keyword::retry>();
            let _ = input.parse::<Token![=]>();
            let expr: Expr = input.parse()?;
            println!("{}", expr.to_token_stream());
            Ok(OptionalRetry::SomeRetry(Retry { strategy: expr }))
        } else {
            Ok(OptionalRetry::NoRetry)
        }
    }
}


fn generate_ident_with_prefix(ident: &str) -> String{
    format!("fn_composer__{}", ident)
}


#[proc_macro_attribute]
pub fn composeable(attr: TokenStream, item: TokenStream) -> TokenStream {
    

    let token_stream_clone = item.clone();
    let item_fn: ItemFn = syn::parse_macro_input!(token_stream_clone);

    let fn_gen = item_fn.sig.generics;
    let mut async_fn = item_fn.sig.asyncness.is_some();
    let input_args = item_fn.sig.inputs;
    let arg_tokens: Vec<_> = input_args.iter().collect();
    let mut_arg_tokens: Vec<_> = input_args.iter().collect();
    let arg_length = input_args.len();
    let fn_ident = &item_fn.sig.ident;
    let fn_name = item_fn.sig.ident.to_string();
    let fn_return_type = &item_fn.sig.output;

    let return_type_without_token = match fn_return_type {
        ReturnType::Default => None,
        ReturnType::Type(_, return_type) => Some(return_type),
    };

    let retry = syn::parse_macro_input!(attr as OptionalRetry);

    if !async_fn {
        match fn_return_type {
            syn::ReturnType::Default => {}
            syn::ReturnType::Type(_, t) => {
                let x = t.deref();
                async_fn = x.to_token_stream().to_string().starts_with("BoxFuture");
            }
        }
    }
    let lifted_fn_name = "lifted_fn_".to_owned() + &fn_name;
    let prefixed_lifted_fn_name = &generate_ident_with_prefix(&lifted_fn_name);
    //let lift_retry_fn_name = &generate_ident_with_prefix(&("retry_".to_owned() + &lifted_fn_name));
    let lift_fn_ident = syn::Ident::new(prefixed_lifted_fn_name, proc_macro2::Span::call_site());
    let is_retry_fn_name = &generate_ident_with_prefix(&("is_retryable_".to_owned() + &fn_name));
    let is_retry_fn_ident = syn::Ident::new(&is_retry_fn_name, proc_macro2::Span::call_site());
    let retry_fn_ident = syn::Ident::new(
        &generate_ident_with_prefix(&("retry_".to_owned() + &fn_name)),
        proc_macro2::Span::call_site(),
    );

    let async_fn_name = &generate_ident_with_prefix(&("is_async_".to_owned() + fn_name.deref()));
    let async_fn_ident = syn::Ident::new(
        async_fn_name,
        proc_macro2::Span::call_site(),
    );
    let (
        _return_type,
        _underlying_lift_fn_name,
        fun_gen,
        return_type_ident,
        ret_gen,
        underlying_lift_fn_name_ident,
    ) = /*if (asyncFn)*/ {
        let return_type = if async_fn {
            "BoxedAsyncFn".to_owned() + arg_length.to_string().as_str()
        } else {
            "BoxedFn".to_owned() + arg_length.to_string().as_str()
        };
        let underlying_lift_fn_name = if async_fn {
            "lift_async_fn".to_owned() + arg_length.to_string().as_str()
        } else {
            "lift_sync_fn".to_owned() + arg_length.to_string().as_str()
        };
        let gen_type_params = generate_generics_parameters((arg_length + 1) as u8);
        let fun_arg_params = generate_generics_parameters((arg_length) as u8);
        let return_type_param = generate_return_type_param((arg_length + 1) as u8);
        let fun_gen = if async_fn {
            let gen_type = format!("<'a, {gen_type_params} E1, F:Fn({fun_arg_params})->BoxFuture<'a,Result<{return_type_param}, E1>> + 'a + Send +Sync>", );            
            syn::parse_str::<syn::Generics>(
                gen_type.as_str()
            ).ok()
                .unwrap()
        } else {
            let gen_type  =format!("<'a, {gen_type_params} E1, F:Fn({fun_arg_params})->Result<{return_type_param}, E1> + Send +Sync + 'a>");            
            syn::parse_str::<syn::Generics>(
                gen_type.as_str()                
            ).ok().unwrap()
        };

        let return_type_ident = syn::Ident::new(return_type.as_str(), proc_macro2::Span::call_site());
        let underlying_lift_fn_name_ident =
            syn::Ident::new(underlying_lift_fn_name.as_str(), proc_macro2::Span::call_site());
        let ret_gen = syn::parse_str::<syn::Generics>(format!("<'a,{gen_type_params} E1>").as_str()).ok().unwrap();

        (
            return_type,
            underlying_lift_fn_name,
            fun_gen,
            return_type_ident,
            ret_gen,
            underlying_lift_fn_name_ident,
        )
    };

    match retry {
        OptionalRetry::NoRetry => {
            let function_mut_args = FunctionMutArgs {
                args: mut_arg_tokens,
            };
            let tokens: proc_macro2::TokenStream = quote! {
                use function_compose::*;

                pub fn #lift_fn_ident #fun_gen(f: F)  -> #return_type_ident #ret_gen{
                    #underlying_lift_fn_name_ident(f)
                }

                pub fn #async_fn_ident ()  -> bool{
                    #async_fn
                }

                 pub fn #is_retry_fn_ident ()  -> bool{
                         false
                    }

                /**
                * It is only added to keep the compiler happy for non retryable functions
                */
                pub fn #retry_fn_ident #fn_gen ( #function_mut_args)  #fn_return_type {
                    panic!("Function not to be called");
                }
            };
            let mut token_stream: proc_macro::TokenStream = tokens.into();
            token_stream.extend(item.into_iter());
            //println!("{}", toekn_stream.to_string());
            token_stream
        }
        SomeRetry(strategy) => {
            let function_args = FunctionArgs { args: arg_tokens };

            let function_mut_args = FunctionMutArgs {
                args: mut_arg_tokens,
            };

            let mutable_args: Vec<_> = filter_mutable_args(&function_args);

            let mutex_tokens: Vec<_> = convert_to_create_mutex_tokens(&mutable_args);

            let mutex_unlock_tokens: Vec<_> = convert_to_mutex_unlock_tokens(mutable_args);

            let deref_mut_tokens: Vec<_> = convert_to_deref_tokens(&function_args);

            let strategy_expr = strategy.strategy;
            let retry_tokens: proc_macro2::TokenStream = if async_fn {
                quote! {

                    pub fn #retry_fn_ident #fn_gen(#function_mut_args)  #fn_return_type {
                        use function_compose::*;
                        use retry::*;
                        use tokio_retry::Retry as AsyncRetry;
                        use tokio::sync::Mutex;
                        use std::ops::{Deref, DerefMut};
                        async{
                            #( #mutex_tokens )*
                            let result = AsyncRetry::spawn(#strategy_expr, || async{
                                #( #mutex_unlock_tokens )*;
                                let r = #fn_ident(#( #deref_mut_tokens )*);
                                //OperationResult::from()
                                r.await
                            });

                            let result = match result.await{
                                    Ok(result) => Ok(result),
                                    Err(e) => Err(e)
                            };
                            result
                        }.boxed()
                    }
                }
            } else {
                quote! {

                    pub fn #retry_fn_ident #fn_gen (#function_mut_args)  #fn_return_type {
                        use function_compose::*;
                        use retry::*;

                        let result = retry(#strategy_expr, ||{
                            let r:#return_type_without_token = #fn_ident(#function_args).into();
                            r
                        });
                        match result{
                            Ok(result) => Ok(result),
                            Err(e) => Err(e.error)
                        }
                    }
                }
            };

            /*println!("#############################");
            println!("{}", retry_tokens);
            println!("#############################");*/

            let tokens: proc_macro2::TokenStream = quote! {

                use function_compose::*;
                pub fn #lift_fn_ident #fun_gen(f: F)  -> #return_type_ident #ret_gen{
                    //#lift_retry_fn_ident(#retryFnIdent)
                    #underlying_lift_fn_name_ident(f)
                }

                /*pub fn #lift_retry_fn_ident #fun_gen(f: F)  -> #return_type_ident #ret_gen{
                    #underlying_lift_fn_name_ident(#retryFnIdent)
                }*/

                 pub fn #is_retry_fn_ident ()  -> bool{
                     true
                }


                pub fn #async_fn_ident ()  -> bool{
                    #async_fn
                }
            };
            let retry_token_stream: TokenStream = retry_tokens.into();

            let mut token_stream: proc_macro::TokenStream = tokens.into();

            token_stream.extend(item.into_iter());
            token_stream.extend(retry_token_stream.into_iter());
            /*println!("{}", toekn_stream.to_string());*/
            token_stream
        }
    }
}

fn filter_mutable_args<'a>(function_args: &'a FunctionArgs) -> Vec<&'a &'a FnArg> {
    function_args
        .args
        .iter()
        .filter(|fn_arg| {
            match fn_arg {
                FnArg::Receiver(_) => return false,
                FnArg::Typed(pat) => {
                    let ty = pat.ty.deref();
                    match ty {
                        Type::Reference(ty_ref) => return ty_ref.mutability.is_some(),

                        _ => {
                            return false;
                        }
                    }
                }
            }
        })
        .collect()
}

fn convert_to_create_mutex_tokens(mutable_args: &Vec<&&FnArg>) -> Vec<proc_macro2::TokenStream> {
    mutable_args
        .iter()
        .map(|i| match i {
            FnArg::Receiver(_pat_type) => {
                panic!();
            }
            FnArg::Typed(pat_type) => {
                let pat = &pat_type.pat;
                quote! {
                    let mut #pat =Mutex::new(#pat);
                }
            }
        })
        .collect()
}

fn convert_to_mutex_unlock_tokens(mutable_args: Vec<&&FnArg>) -> Vec<proc_macro2::TokenStream> {
    mutable_args
        .iter()
        .map(|i| match i {
            FnArg::Receiver(_pat_type) => {
                panic!();
            }
            FnArg::Typed(pat_type) => {
                let pat = &pat_type.pat;
                quote! {
                    let mut #pat = #pat.lock().await;

                }
            }
        })
        .collect()
}

fn convert_to_deref_tokens(function_args: &FunctionArgs) -> Vec<proc_macro2::TokenStream> {
    function_args
        .args
        .iter()
        .map(|i| {
            match i {
                FnArg::Receiver(_pat_type) => {
                    return quote! {};
                }
                FnArg::Typed(pat_type) => {
                    let pat = &pat_type.pat;
                    let ty = pat_type.ty.deref();
                    match ty {
                        Type::Reference(ty_ref) => {
                            if ty_ref.mutability.is_some() {
                                return quote! {
                                    #pat.deref_mut(),
                                };
                            } else {
                                return quote! {
                                    #pat,
                                };
                            }
                        }

                        _ => {
                            return quote! {
                                #pat,
                            };
                        }
                    }
                    /*quote!{
                        let mut #pat = #pat.lock().await;

                    }*/
                }
            }
        })
        .collect()
}
