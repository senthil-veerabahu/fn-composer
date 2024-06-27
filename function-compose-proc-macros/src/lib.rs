use proc_macro::TokenStream;
use std::any::Any;
use std::fmt::Formatter;
use std::{borrow::Borrow, fmt::Display, ops::Deref};

use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::ParseStream;
use syn::token::Token;
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
pub fn retry(attr: TokenStream, item: TokenStream) -> TokenStream {
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
                                    &mut #ident,
                                }
                            } else {
                                quote! {
                                    & #ident,
                                }
                            }
                        }
                        _ => {
                            quote! {
                                 #ident,
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
            let tokeStream = quote! {
                        mut #arg,
            };
            tokens.append_all(tokeStream.into_iter());
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
            input.parse::<keyword::retry>();
            input.parse::<Token![=]>();
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
    use syn::parse::Parser;

    let tokenStreamClone = item.clone();
    let item_fn: ItemFn = syn::parse_macro_input!(tokenStreamClone);

    let fn_gen = item_fn.sig.generics;
    let mut asyncFn = item_fn.sig.asyncness.is_some();
    let inputArgs = item_fn.sig.inputs;
    let arg_tokens: Vec<_> = inputArgs.iter().collect();
    let mut_arg_tokens: Vec<_> = inputArgs.iter().collect();
    let argLength = inputArgs.len();
    let fnIdent = &item_fn.sig.ident;
    let fnName = item_fn.sig.ident.to_string();
    let fnReturnType = &item_fn.sig.output;

    let returnTypeWithoutToken = match fnReturnType {
        ReturnType::Default => None,
        ReturnType::Type(_, returnType) => Some(returnType),
    };

    let retry = syn::parse_macro_input!(attr as OptionalRetry);

    if (!asyncFn) {
        match fnReturnType {
            syn::ReturnType::Default => {}
            syn::ReturnType::Type(_, t) => {
                let x = t.deref();
                asyncFn = x.to_token_stream().to_string().starts_with("BoxFuture");
            }
        }
    }
    let lifted_fn_name = ("lifted_fn_".to_owned() + &fnName);
    let prefixed_lifted_fn_name = &generate_ident_with_prefix(&lifted_fn_name);
    //let lift_retry_fn_name = &generate_ident_with_prefix(&("retry_".to_owned() + &lifted_fn_name));
    let liftFnIdent = syn::Ident::new(prefixed_lifted_fn_name, proc_macro2::Span::call_site());
    let is_retry_fn_name = &generate_ident_with_prefix(&("is_retryable_".to_owned() + &fnName));
    let is_retry_fn_ident = syn::Ident::new(&is_retry_fn_name, proc_macro2::Span::call_site());
    let retryFnIdent = syn::Ident::new(
        &generate_ident_with_prefix(&("retry_".to_owned() + &fnName)),
        proc_macro2::Span::call_site(),
    );

    let async_fn_name = &generate_ident_with_prefix(&("is_async_".to_owned() + fnName.deref()));
    let asyncFnIdent = syn::Ident::new(
        async_fn_name,
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
        } else {
            "BoxedFn".to_owned() + argLength.to_string().as_str()
        };
        let underlyingLiftFnName = if (asyncFn) {
            "lift_async_fn".to_owned() + argLength.to_string().as_str()
        } else {
            "lift_sync_fn".to_owned() + argLength.to_string().as_str()
        };
        let gen_type_params = generate_generics_parameters((argLength + 1) as u8);
        let fun_arg_params = generate_generics_parameters((argLength) as u8);
        let return_type_param = generate_return_type_param((argLength + 1) as u8);
        let funGen = if (asyncFn) {
            syn::parse_str::<syn::Generics>(
                format!("<'a, {gen_type_params} F:Fn({fun_arg_params})->BoxFuture<'a,Result<{return_type_param}, FnError>> + 'a + Send +Sync>", ).as_str()
            ).ok()
                .unwrap()
        } else {
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

    match retry {
        OptionalRetry::NoRetry => {
            let function_mut_args = FunctionMutArgs {
                args: mut_arg_tokens,
            };
            let mut tokens: proc_macro2::TokenStream = quote! {
                use function_compose::*;

                pub fn #liftFnIdent #funGen(f: F)  -> #returnTypeIdent #retGen{
                    #underlyingLiftFnNameIdent(f)
                }

                pub fn #asyncFnIdent ()  -> bool{
                    #asyncFn
                }

                 pub fn #is_retry_fn_ident ()  -> bool{
                         false
                    }

                /**
                * It is only added to keep the compiler happy for non retryable functions
                */
                pub fn #retryFnIdent #fn_gen (#function_mut_args)  #fnReturnType {
                    panic!("Function not to be called");
                }
            };
            let mut toeknStream: proc_macro::TokenStream = tokens.into();
            toeknStream.extend(item.into_iter());
            println!("{}", toeknStream.to_string());
            toeknStream
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
            let retry_tokens: proc_macro2::TokenStream = if (asyncFn) {
                quote! {

                    pub fn #retryFnIdent #fn_gen(#function_mut_args)  #fnReturnType {
                        use function_compose::*;
                        use retry::*;
                        use tokio_retry::Retry as AsyncRetry;
                        use tokio::sync::Mutex;
                        use std::ops::{Deref, DerefMut};
                        async{
                            #( #mutex_tokens )*
                            let result = AsyncRetry::spawn(#strategy_expr, || async{
                                #( #mutex_unlock_tokens )*;
                                let r = #fnIdent(#( #deref_mut_tokens )*);
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

                    pub fn #retryFnIdent #fn_gen (#function_mut_args)  #fnReturnType {
                        use function_compose::*;
                        use retry::*;

                        let result = retry(#strategy_expr, ||{
                            let r:#returnTypeWithoutToken = #fnIdent(#function_args).into();
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

            let mut tokens: proc_macro2::TokenStream = quote! {

                use function_compose::*;
                pub fn #liftFnIdent #funGen(f: F)  -> #returnTypeIdent #retGen{
                    //#lift_retry_fn_ident(#retryFnIdent)
                    #underlyingLiftFnNameIdent(f)
                }

                /*pub fn #lift_retry_fn_ident #funGen(f: F)  -> #returnTypeIdent #retGen{
                    #underlyingLiftFnNameIdent(#retryFnIdent)
                }*/

                 pub fn #is_retry_fn_ident ()  -> bool{
                     true
                }


                pub fn #asyncFnIdent ()  -> bool{
                    #asyncFn
                }
            };
            let mut retry_token_stream: TokenStream = retry_tokens.into();

            let mut toeknStream: proc_macro::TokenStream = tokens.into();

            toeknStream.extend(item.into_iter());
            toeknStream.extend(retry_token_stream.into_iter());
            /*println!("{}", toeknStream.to_string());*/
            toeknStream
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
            return false;
        })
        .collect()
}

fn convert_to_create_mutex_tokens(mutable_args: &Vec<&&FnArg>) -> Vec<proc_macro2::TokenStream> {
    mutable_args
        .iter()
        .map(|i| match i {
            FnArg::Receiver(pat_type) => {
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
            FnArg::Receiver(pat_type) => {
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
                FnArg::Receiver(pat_type) => {
                    return quote! {};
                }
                FnArg::Typed(pat_type) => {
                    let pat = &pat_type.pat;
                    let ty = pat_type.ty.deref();
                    match ty {
                        Type::Reference(ty_ref) => {
                            if (ty_ref.mutability.is_some()) {
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
