use std::{ops::Deref, any::{Any, type_name, TypeId}, borrow::{Borrow, BorrowMut}, fmt::Display};

use fnutils::AsyncIntoBoxedFuture;
use proc_macro::{TokenStream, Span, Ident};
use proc_macro2::Delimiter;
use quote::{quote, ToTokens};
use syn::{ItemFn, Ident as SynIdent, parse::{ParseStream, Parse,Result, self}, Token, Expr, parse_macro_input, parenthesized, token::{Paren, Token}, Type, punctuated::Punctuated, Error};

use futures::{FutureExt, future::BoxFuture};

use paste::paste;




#[derive(Clone)]
struct FnComposerData{
    initial: bool,
    fnName: SynIdent,
    liftFnName: SynIdent,
    asyncFnIdent: SynIdent,
    injectableArgs:Option<Punctuated<Expr, Token![,]>>,
    next: Option<Box<FnComposerData>>,
    finalArgs:Option<Punctuated<Expr, Token![,]>>,
}

#[non_exhaustive]
enum ParseResult<'a> {
    Handled,
    HandledWithCurrentResult(&'a mut FnComposerData),
    HandledWithNewResult(FnComposerData),
    UnhandledIdentifier(SynIdent)
    
}
fn parseWithArgs<'a>(input: ParseStream,identifierOption: Option<SynIdent>, currentFnComposeData: &'a mut FnComposerData) ->Result<ParseResult<'a>>{
    let fnName:SynIdent = match identifierOption {
        Some(identifier) => identifier,
        None => input.parse()?,
    };    
    if(fnName.to_string() == "withArgs"){
        let content;
        let e:Paren = parenthesized!(content in input);
        let args = content.parse_terminated(Expr::parse, Token![,])?;
        //println!("expecting expression for provide {:?}",&args);
        currentFnComposeData.finalArgs = Some(args);
        return Ok(ParseResult::Handled);
    }else{
        Ok(ParseResult::UnhandledIdentifier(fnName))
    }
}

fn parseFnComposer<'a>(input: ParseStream, currentIdentifierOption: Option<SynIdent>)->Result<ParseResult<'a>>{
    let fnName = match currentIdentifierOption {
        Some(identifier) => identifier,
        None => input.parse()?,
    };    
    let asyncPrefix = "async_".to_owned();
    let fnNameString = fnName.to_string();
    let liftFnIdent = syn::Ident::new(("lifted_fn_".to_owned() + (fnNameString.deref()).deref()).deref(), 
            proc_macro2::Span::call_site());
                
    let asyncFnIdent = syn::Ident::new(
        (asyncPrefix+fnNameString.deref()).deref(), 
            proc_macro2::Span::call_site());

    //println!("First fn data composed {}" , fnName);

    let mut currentFnMetaData = FnComposerData{
        fnName: fnName,
        liftFnName: liftFnIdent,
        asyncFnIdent,
        injectableArgs: None,
        next: None,
        finalArgs: None,
        initial: false,
    };
    Ok(ParseResult::HandledWithNewResult(currentFnMetaData))
}

fn parseProvide<'a>(input: ParseStream, fnComposeData: &'a mut FnComposerData) ->Result<ParseResult<'a>>{
    let hasMethodCall = input.peek(Token![.]);
    let hasIdentifier = input.peek2(SynIdent);
    
    if (!hasIdentifier || !hasMethodCall){
        return Ok(ParseResult::Handled);
    }
    let dotSeparator = input.parse::<Token![.]>()?;
    let provideFn: SynIdent = input.parse()?;
    println!("expecting identifier provide");
    if(provideFn.to_string() == "provide"){
        println!("expecting expression for provide");
        let content;
        let e:Paren = parenthesized!(content in input);
        let args = content.parse_terminated(Expr::parse, Token![,])?;
        //println!("expecting expression for provide {:?}",&args);
        fnComposeData.injectableArgs = Some(args);
        /*println!("before next token parse");
        let testFnName: Result<SynIdent> = input.parse();
        if(testFnName.is_err()){
            println!("parse error is {:?}", testFnName.clone().err());
        }
        println!("after next token parse");
        println!("next token is {:?}", testFnName);*/
        //let nextComposeableFn = Self::parse(input)?;
        return Ok(ParseResult::Handled);
    }else{
        Ok(ParseResult::UnhandledIdentifier(provideFn))
    }        
}

impl Parse for FnComposerData {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut firstFnMetaData =  parseFnCall(input, None)?;
        //let firstFn: &mut FnComposerData = &mut currentFnMetaData;
        

        let result = parseProvide(input, &mut firstFnMetaData)?;
        let mut currentFnMetaData=  &mut firstFnMetaData;
        while(input.peek(Token![->])){

            input.parse::<Token![->]>()?;
            let withArgsResult = parseWithArgs(
                input,
                 None,
                   &mut currentFnMetaData)?;
            match withArgsResult{
                ParseResult::Handled => {
                    return Ok(firstFnMetaData.clone());
                }
                ParseResult::HandledWithCurrentResult(_) => {
                    return Ok(firstFnMetaData.clone());
                }
                ParseResult::HandledWithNewResult(_) => {
                    return Ok(firstFnMetaData.clone());
                },
                ParseResult::UnhandledIdentifier(identifier) => {
                    //println!("identifier is {:?}", identifier);
                    let  mut newFnMetaData = parseFnCall(input, Some(identifier))?;
                    currentFnMetaData.next = Some(Box::new(newFnMetaData));
                    let y = currentFnMetaData.next.as_deref_mut();
                    let mut x = y.unwrap();
                    currentFnMetaData = x;
                }
            }
        }
        firstFnMetaData.initial = true;
        Ok(firstFnMetaData)
    }
}

fn parseFnCall(input: &parse::ParseBuffer<'_>, currentIdentifierOption: Option<SynIdent>) -> Result<FnComposerData> {        
    let parseResult = parseFnComposer(input, currentIdentifierOption)?;
    let currentFnMetaData = match  parseResult {
        ParseResult::Handled => {
            return Err(Error::new(input.span(), "Expecting Handle with new Result"));
        }
        ParseResult::HandledWithCurrentResult(_) => {
            return Err(Error::new(input.span(), "Expecting Handle with new Result"));
        }
        ParseResult::HandledWithNewResult(fnComposer) => fnComposer,
        ParseResult::UnhandledIdentifier(_) => {
            return Err(Error::new(input.span(), "Expecting Handle with new Result"));
        }
    };
    Ok(currentFnMetaData)
}
/* 
#[proc_macro]
pub fn compose(input: TokenStream) -> TokenStream {
    let fnComposer:FnComposerData = parse_macro_input!(input as FnComposerData);
    //println!("fncomposer is {:#?}", fnComposer);
    let tokens = quote!{
        use fnutils::BoxedFutAppTwoArgFnOnce;
        #fnComposer
    };
    tokens.into()
}

macro_rules! compose1 {
    ($fn_name:ident) =>{
        {
            paste!{
                let is_async = <async_ $fn_name>();
                let <lifted_fn_ $fn_name> = <async_ $fn_name>();
            }
        }
    }
}*/

impl  ToTokens for FnComposerData{
    fn to_tokens(&self, inputTokens: &mut proc_macro2::TokenStream) {
        
        let finalFnName = syn::Ident::new(("final_fn".to_owned()).deref(), proc_macro2::Span::call_site());
        
        println!("Created first set of tokens");
        let mut intermediateTokenStream = proc_macro2::TokenStream::new();
        let tokens = self.intoTokens(&mut intermediateTokenStream, finalFnName);
        let mut startToken: proc_macro2::TokenStream = quote!{
            
                {
                    #intermediateTokenStream
                }
        };
        println!("{}", startToken);
        inputTokens.extend(startToken);
    }
}

impl FnComposerData {
    fn fnComposerIntoIdentifiers(&self) -> (&SynIdent, &SynIdent, &SynIdent, bool, 
        Punctuated<Expr, syn::token::Comma>, Punctuated<Expr,
        syn::token::Comma>, bool, SynIdent, SynIdent) {
        let liftFnName = &self.liftFnName;
        let fnName = &self.fnName;
        let asyncFnName = &self.asyncFnIdent;
        let mut hasInjectedArgs = self.injectableArgs.is_some();
        let mut injected_args = Punctuated::<Expr, Token![,]>::new();
        let mut final_args = Punctuated::<Expr, Token![,]>::new();
        let mut hasFinalArgs = self.finalArgs.is_some();
        let asyncVarIdent = syn::Ident::new(("async_fn_".to_owned() + fnName.to_string().deref()).deref(), proc_macro2::Span::call_site());
        println!("has final args {} , {}", hasFinalArgs, fnName );
        if(hasFinalArgs){ 
            let punctuated = (&self.finalArgs).as_ref().unwrap().clone();
            hasFinalArgs = !punctuated.is_empty();
            final_args = punctuated;
            println!(" final args is empty {} , {}", hasFinalArgs, fnName );
        }
        if(hasInjectedArgs){
            hasInjectedArgs = !(&self.injectableArgs.as_ref().unwrap().is_empty());
            injected_args = (&self.injectableArgs).as_ref().unwrap().clone();
        }
        let liftedFnIdent = syn::Ident::new(("lifted_fn".to_owned() + fnName.to_string().deref()).deref(), proc_macro2::Span::call_site());
        (fnName, liftFnName, asyncFnName, hasInjectedArgs, injected_args, final_args, hasFinalArgs, liftedFnIdent, asyncVarIdent)
    }

    fn intoTokens(&self,  tokens:&mut proc_macro2::TokenStream,finalFnName: SynIdent) {
        let (fnName, liftFnName, asyncFnName, hasInjectedArgs, injected_args, final_args, hasFinalArgs, liftedFnIdent, asyncVarIdent) = self.fnComposerIntoIdentifiers();
        let finalAsyncMethod = syn::Ident::new(("final_fn_async".to_owned()).deref(), proc_macro2::Span::call_site());
        let isHeadIdent = syn::Ident::new(self.initial.to_string().as_str(), proc_macro2::Span::call_site());
        let mut firstToken: proc_macro2::TokenStream = quote!{
    
            let #liftedFnIdent = #liftFnName(#fnName);
            let #asyncVarIdent:bool = #asyncFnName();
            if(#hasInjectedArgs){
                #liftedFnIdent = #liftedFnIdent.provide(#injected_args);
            }

            


            if(#isHeadIdent){
                
                let #finalFnName = #liftedFnIdent;
                let #finalAsyncMethod=#asyncVarIdent;
            }else{
                if(!#asyncVarIdent && !#finalAsyncMethod ){
                    #finalFnName = #finalFnName.then(#liftedFnIdent);
                    #finalAsyncMethod=false;
                }else if(#asyncVarIdent && !#finalAsyncMethod ){
                    #finalFnName = #finalFnName.async_out(#liftedFnIdent);
                    #finalAsyncMethod=true;
                }
                else if(!#asyncVarIdent && #finalAsyncMethod ){
                    #finalFnName = #finalFnName.async_into(#liftedFnIdent);
                    #finalAsyncMethod=true;
                }else if(#asyncVarIdent && #finalAsyncMethod ){
                    #finalFnName = #finalFnName.async_into(#liftedFnIdent);
                    #finalAsyncMethod=true;
                }
            }

            if(#hasFinalArgs){
                let result = #finalFnName(#final_args);
                return result;
            }

    
        };

        tokens.extend(firstToken);
        println!("Created next set of tokens");
        if(self.next.is_some()){
            let fn_composer_data = (&self.next);
            let (fnNameNext, liftFnNameNext, asyncFnNameNext, hasInjectedArgsNext, 
                injected_args_next, final_args_next, 
                hasFinalArgsNext, liftedFnIdentNext, asyncVarIdentNext) = fn_composer_data.as_ref().unwrap().fnComposerIntoIdentifiers();
    
            let mut nextToken: proc_macro2::TokenStream = quote!{
        
        
                let #liftedFnIdentNext = #liftFnNameNext(#fnNameNext);
                let asyncFn:bool = #asyncFnNameNext();
                if(#hasInjectedArgsNext){
                    #liftedFnIdentNext = #liftedFnIdent.provide(#injected_args_next);
                }

                
                if(!#asyncVarIdent && !#asyncVarIdentNext ){
                    #finalFnName = #finalFnName.then(#liftFnNameNext);
                    #finalAsyncMethod=false;
                }else if(#asyncVarIdent && !#asyncVarIdentNext ){
                    #finalFnName = #finalFnName.async_out(#liftFnNameNext);
                    #finalAsyncMethod=true;
                }
                else if(!#asyncVarIdent && #asyncVarIdentNext ){
                    #finalFnName = #finalFnName.async_into(#liftFnNameNext);
                    #finalAsyncMethod=true;
                }else if(#asyncVarIdent && #asyncVarIdentNext ){
                    #finalFnName = #finalFnName.async_into(#liftFnNameNext);
                    #finalAsyncMethod=true;
                }                

                if(#hasFinalArgsNext){
                    let result = #finalFnName(#final_args_next);
                    return result;
                }
            };
            tokens.extend(nextToken);
            if(self.next.as_ref().unwrap().next.is_some()){
                FnComposerData::intoTokens(&self.next.as_ref().unwrap().next.as_ref().unwrap(), tokens, finalFnName);
            }
        }
    }
}

#[proc_macro_attribute]
pub fn composeable(attr: TokenStream,item: TokenStream) -> TokenStream {
    let tokenStreamClone = item.clone();
    let item_fn: ItemFn = syn::parse_macro_input!(tokenStreamClone);
    let mut asyncFn = item_fn.sig.asyncness.is_some();
    let inputArgs = item_fn.sig.inputs;
    let argLength = inputArgs.len();
    let fnName = item_fn.sig.ident.to_string();
    let fnReturnType = item_fn.sig.output;
    if(!asyncFn){
        match fnReturnType {
            syn::ReturnType::Default => { 
                
            }
            syn::ReturnType::Type(_, t) => {
                let x= t.deref();
                
                asyncFn = x.to_token_stream().to_string().starts_with("BoxFuture");
                
                //let x =t.to_owned().borrow(); 
                //assert_eq!(TypeId::of::<BoxFuture>(), x.type_id());
            },
        }
    }

    let binding = ("lifted_fn_".to_owned()+&fnName);
    let liftFnName = binding.deref();
    let liftFnIdent = syn::Ident::new(liftFnName, proc_macro2::Span::call_site());

    let liftFnIdent = syn::Ident::new(liftFnName, proc_macro2::Span::call_site());
    let asyncPrefix = "async_".to_owned();
    let asyncFnIdent = syn::Ident::new(
        (asyncPrefix+fnName.deref()).deref(), 
         proc_macro2::Span::call_site());
    let (returnType, underlyingLiftFnName, funGen, returnTypeIdent,retGen, underlyingLiftFnNameIdent) = 
    if(asyncFn){  

         
        let returnType = if argLength == 2 { "BoxedFutAppTwoArgFnOnce" }  else { "BoxedFutAppFnOnce" };
        let underlyingLiftFnName = if argLength == 2 { "liftTwoArgAsync" }  else { "liftAsync" };
        let funGen = if argLength == 2  { syn::parse_str::<syn::Generics>("<'a,A,B,C,F:Fn(A,B)->BoxFuture<'a,Result<C, FnError>> + 'a + Send +Sync>").ok().unwrap() } 
        else 
            {syn::parse_str::<syn::Generics>("<'a,A,B, F:Fn(A)->BoxFuture<'a,Result<B, FnError>> + 'a + Send +Sync>").ok().unwrap()};
        let returnTypeIdent = syn::Ident::new(returnType, proc_macro2::Span::call_site());
        let underlyingLiftFnNameIdent = syn::Ident::new(underlyingLiftFnName, proc_macro2::Span::call_site());
        let retGen = if argLength == 2  { syn::parse_str::<syn::Generics>("<'a,A,B,C>").ok().unwrap() } else {syn::parse_str::<syn::Generics>("<'a,A,B>").ok().unwrap()};

        
        (returnType, underlyingLiftFnName, funGen, returnTypeIdent,retGen, underlyingLiftFnNameIdent)
       
    }else {
        let returnType = if argLength == 2 { "BoxedFutAppTwoArgFnOnce" }  else { "BoxedAppFn" };
        let underlyingLiftFnName = if argLength == 2 { "liftTwoArgAsync" }  else { "lift" };
        println!("before fn gen");
        let funGen = if argLength == 2  { syn::parse_str::<syn::Generics>("<'a,A,B,C,F:Fn(A,B)->Result<C, FnError> + Send +Sync + 'a>").ok().unwrap() }
        else 
            {syn::parse_str::<syn::Generics>("<'a, A,B,F:Fn(A)->Result<B, FnError>  + Send +Sync + 'a>").ok().unwrap()};
            println!("after fn gen");
        let retGen = if argLength == 2  { syn::parse_str::<syn::Generics>("<'a, A,B,C>").ok().unwrap() } else {syn::parse_str::<syn::Generics>("<'a,A,B>").ok().unwrap()};
        let returnTypeIdent = syn::Ident::new(returnType, proc_macro2::Span::call_site());
        let underlyingLiftFnNameIdent = syn::Ident::new(underlyingLiftFnName, proc_macro2::Span::call_site());
        (returnType, underlyingLiftFnName, funGen, returnTypeIdent,retGen , underlyingLiftFnNameIdent)
        
    };

    if(asyncFn){
        let mut tokens: proc_macro2::TokenStream = quote!{
            use fnutils::*;
            pub fn #liftFnIdent #funGen(f: F)  -> #returnTypeIdent #retGen{
                println!("test");
                #underlyingLiftFnNameIdent(f)
            }

            pub fn #asyncFnIdent ()  -> bool{            
                #asyncFn
            }
        };
        let mut toeknStream:proc_macro::TokenStream = tokens.into();
        toeknStream.extend(item.into_iter());
        toeknStream
    }else{
        let mut tokens: proc_macro2::TokenStream = quote!{
            use fnutils::*;
            pub fn #liftFnIdent #funGen(f: F)  -> #returnTypeIdent #retGen{
                println!("test11");
                #underlyingLiftFnNameIdent(f)
            }

            pub fn #asyncFnIdent ()  -> bool{            
                #asyncFn
            }
        };
        let mut toeknStream:proc_macro::TokenStream = tokens.into();
        toeknStream.extend(item.into_iter());
        toeknStream
    }
    //println!("{}",toeknStream.to_string());
    
}