//un comment below lines to debug macros

/*#![feature(trace_macros)]
trace_macros!(true);*/
//! Crate `function-compose` provides utilities for composing functions and way to inject arguments to functions
//! The composeable functions should return rust Result type with FnError as Err type
//!
//!
//! ### Usage
//! ```rust
//! use function_compose::composeable;
//! #[composeable()]
//! pub fn add_10(a: i32) -> Result<i32, FnError<String>> {
//!     Ok(a + 10)
//! }
//! 
//! ```
//! 
//! ##### The async function should return BoxFuture and the error type should be FnError.
//! 
//! ```rust
//! use function_compose::composeable;
//! use futures::{future::BoxFuture, FutureExt};
//! #[composeable()]
//! pub fn add_async(a: i32, b: i32) -> BoxFuture<'static, Result<i32, FnError<String>>> {
//!     async move {
//!         let r = a + b;
//!         Ok(r)
//!     }.boxed()
//! }
//! ```
//! 
//! ##### Composing async and sync functions usage
//!
//!```ignore
//! use function_compose::compose;
//! use fn_macros::composeable;
//! use futures::{future::BoxFuture, FutureExt};
//! #[composeable()]
//! pub fn add_async(a: i32, b: i32) -> BoxFuture<'static, Result<i32, FnError<String>>> {
//!     async move {
//!         let r = a + b;
//!         Ok(r)
//!     }.boxed()
//! }
//! #[composeable()]
//! pub fn add_10(a: i32) -> Result<i32, FnError<String>> {
//!     Ok(a + 10)
//! }
//! async fn test(){
//!    let result = compose!(add_async.provide(10) -> add_100 -> with_args(10)).await;
//!    assert_eq!(210, result.unwrap());
//! }
//! ```
//! ##### Function argument injection usage
//!```rust
//! use function_compose::composeable;
//! use futures::{future::BoxFuture, FutureExt};
//! #[composeable()]
//! pub fn add_3_arg_async(a: i32,b: i32, c:i32) -> BoxFuture<'static, Result<i32, FnError<String>>>{
//!     async move{
//!         let  r =   a + b + c;
//!         Ok(r)
//!     }.boxed()
//! }
//! ```
//! ##### Example of multiple injection to async function
//!
//!```ignore
//! use crate::compose;
//! let result = compose!(add_3_arg_async.provide(100).provide(200) -> add_10 -> with_args(10)).await;
//! assert_eq!(220, result.unwrap());
//!```


use std::{error::Error};



use futures::{future::BoxFuture, FutureExt};
//use paste::paste;


//pub type FnError = Box<dyn Error>;

#[derive(Debug)]
pub struct FnError<E>{
    pub underlying_error: Option<E>,
    pub error_code:Option<String>,
    pub description: Option<String>
}

impl From<String> for FnError<String>{
    fn from(value: String) -> Self {
        return FnError::<String>{
            underlying_error: Some(value.clone()),
            error_code:None,
            description: Some(value)
        };
    }
}
fn to_fn_error<E1, E2>(fn_error:FnError<E1>) -> FnError<E2> where E2:From<E1>{
        let underlying_error = match fn_error.underlying_error{
            None => {None}
            Some(error) => {Some(From::from(error))}
        };

        FnError {
            description: fn_error.description,
            error_code: fn_error.error_code,
            underlying_error: underlying_error
        }
}

/*impl<E1, E2> From<FnError<E1>> for FnError<E2> where E2:From<E1>{
    fn from(value: FnError<E1>) -> Self {
        todo!()
    }
}*/

pub use function_compose_proc_macros::*;
pub use paste::*;
pub use concat_idents::concat_idents;

macro_rules! composer_generator {
    ($arg1:ident, $return_type1:ident, $return_type2:ident, $error_type1:ident, $error_type2:ident) => {
        paste!{
            #[doc = concat!("Then implementation for composing sync function (BoxedFn1) with another sync function(BoxedFn1) ")]
            impl<'a, $arg1: 'a + Send, $return_type1: 'a + Send, $return_type2: 'a, $error_type1: Send + 'a, $error_type2: Send + 'a>
                Then<'a, $arg1, $return_type1, $return_type2, BoxedFn1<'a, $return_type1, $return_type2, $error_type2>, BoxedFn1<'a, $arg1, $return_type2, $error_type2>> for BoxedFn1<'a, $arg1, $return_type1, $error_type1> where E2:From<E1>{

                fn then(self, f: BoxedFn1<'a, $return_type1, $return_type2, $error_type2>) -> BoxedFn1<'a, $arg1, $return_type2, $error_type2> {
                    let r1 = move |x: $arg1| {
                        let gResult = self(x);
                        match gResult{
                                Ok(innerResult) => f(innerResult),
                                Err(error) =>   Err(to_fn_error(error)),
                            }
                    };
                    Box::new(r1)
                }
            }

            #[doc = concat!("Then implementation for composing sync function(BoxedFn1) with another async function(BoxedAsyncFn1) ")]
            impl<'a, $arg1: 'a + Send, $return_type1: 'a + Send, $return_type2: 'a, $error_type1: Send + 'a, $error_type2: Send + 'a>
                Then<'a, $arg1, $return_type1, $return_type2, BoxedAsyncFn1<'a, $return_type1, $return_type2, $error_type2>, BoxedAsyncFn1<'a, $arg1, $return_type2, $error_type2>> for BoxedFn1<'a, $arg1, $return_type1, $error_type1> where E2:From<E1>{

                fn then(self, f: BoxedAsyncFn1<'a, $return_type1, $return_type2, $error_type2>) -> BoxedAsyncFn1<'a, $arg1, $return_type2, $error_type2> {
                    let r1 =  |x: $arg1| {
                        async move{
                            let gResult = self(x);
                            match gResult{
                                Ok(innerResult) => f(innerResult).await,
                                Err(error) =>   Err(to_fn_error(error)),
                            }
                            //f(b).await
                        }.boxed()
                    };
                    Box::new(r1)
                }
            }

            #[doc = concat!("Then implementation for composing async function(BoxedAsyncFn1) with another sync function(BoxedFn1) ")]

            impl<'a, $arg1: 'a + Send, $return_type1: 'a + Send, $return_type2: 'a, $error_type1:Send +  'a, $error_type2:Send +  'a>
                Then<'a, $arg1, $return_type1, $return_type2, BoxedFn1<'a, $return_type1, $return_type2, $error_type2>, BoxedAsyncFn1<'a, $arg1, $return_type2, $error_type2>> for BoxedAsyncFn1<'a, $arg1, $return_type1, $error_type1> where E2:From<E1>{

                fn then(self, f: BoxedFn1<'a, $return_type1, $return_type2, $error_type2>) -> BoxedAsyncFn1<'a, $arg1, $return_type2, $error_type2> {
                    let r1 = |a: $arg1| {
                        async move {
                            let gResult = self(a).await;
                            match gResult{
                                Ok(innerResult) => f(innerResult),
                                Err(error) =>   Err(to_fn_error(error)),
                            }
                        }.boxed()
                    };
                    let r: BoxedAsyncFn1<'a,$arg1, $return_type2, $error_type2> = Box::new(r1);
                    r
                }
            }


            #[doc = concat!("Then implementation for composing async function(BoxedAsyncFn1) with another async function(BoxedAsyncFn1) ")]
            impl<'a, $arg1: 'a + Send, $return_type1: 'a + Send, $return_type2: 'a, $error_type1:Send +  'a, $error_type2:Send + 'a>
                Then<'a, $arg1, $return_type1, $return_type2, BoxedAsyncFn1<'a, $return_type1, $return_type2, $error_type2>, BoxedAsyncFn1<'a, $arg1, $return_type2, $error_type2>> for BoxedAsyncFn1<'a, $arg1, $return_type1, $error_type1> where E2:From<E1>{

                fn then(self, f: BoxedAsyncFn1<'a, $return_type1, $return_type2, $error_type2>) -> BoxedAsyncFn1<'a, $arg1, $return_type2, $error_type2> {
                    let r1 = |a: $arg1| {
                        async move {
                            let gResult = self(a).await;
                            match gResult{
                                Ok(innerResult) => f(innerResult).await,
                                Err(error) =>   Err(to_fn_error(error)),
                            }
                        }.boxed()
                    };
                    let r: BoxedAsyncFn1<'a,$arg1, $return_type2, $error_type2> = Box::new(r1);
                    r
                }
            }
        }
    }
}
macro_rules! impl_injector {
    ([$($args:ident),*], $provided:ident, $return_type:ident, $error_type:ident, $arg_size:literal, $return_fn_arg_size:literal) => {

        paste!  {
            #[doc = concat!("dependency injection function provide_f", stringify!($arg_size), " for injecting the last argument of a given sync function")]
            pub fn [<provider_f $arg_size>]<'a, $($args),*, $provided, $return_type, $error_type>(fn1: [<BoxedFn $arg_size>]<'a, $($args),*, $provided, $return_type, $error_type>,provided_data: $provided,) -> [<BoxedFn $return_fn_arg_size>]<'a, $($args),* , $return_type, $error_type> where $( $args: 'a ),*, $provided: Send + Sync + 'a, $return_type: 'a, $error_type: 'a{
                    Box::new(move |$( [<$args:lower>]:$args ),*| fn1($( [<$args:lower>]),*,  provided_data))
            }

            #[doc = concat!("dependency injection function provider_async_f", stringify!($arg_size), " for injecting the last argument of a given async function")]
            pub fn [<provider_async_f $arg_size>]<'a, $($args),*, $provided, $return_type, $error_type>(fn1: [<BoxedAsyncFn $arg_size>]<'a, $($args),*, $provided, $return_type, $error_type>,provided_data: $provided,) -> [<BoxedAsyncFn $return_fn_arg_size>]<'a, $($args),* , $return_type, $error_type> where $( $args: 'a ),*, $provided: Send + Sync + 'a, $return_type: 'a, $error_type: 'a{
                    Box::new(move |$( [<$args:lower>]:$args ),*| fn1($( [<$args:lower>]),*,  provided_data))
            }

        }
        paste!{

            #[doc = concat!("Injector implementation for a given sync function that accepts " , stringify!($return_fn_arg_size+1), " arguments and returns a function with ", stringify!($return_fn_arg_size), " arguments")]
            impl<'a, $($args),*, $provided, $return_type, $error_type> Injector<$provided, [<BoxedFn $return_fn_arg_size>]<'a, $($args),*, $return_type, $error_type>> for [<BoxedFn $arg_size>] <'a, $($args),*, $provided, $return_type, $error_type>
            where $( $args: 'a ),*, $provided: Send + Sync +'a, $return_type: 'a, $error_type: 'a
            {
                fn provide(self, a: $provided) -> [<BoxedFn $return_fn_arg_size>]<'a, $($args),*, $return_type, $error_type> {
                    let r = [<provider_f $arg_size>](self, a);
                    r
                }
            }

            #[doc = concat!("Injector implementation for a given async function that accepts " , stringify!($return_fn_arg_size+1), " arguments  and returns a function with ", stringify!($return_fn_arg_size), " arguments")]
            impl<'a, $($args),*, $provided, $return_type, $error_type> Injector<$provided, [<BoxedAsyncFn $return_fn_arg_size>]<'a, $($args),*, $return_type, $error_type>> for [<BoxedAsyncFn $arg_size>] <'a, $($args),*, $provided, $return_type, $error_type>
            where $( $args: 'a ),*, $provided: Send + Sync +'a, $return_type: 'a, $error_type: 'a
            {
                fn provide(self, a: $provided) -> [<BoxedAsyncFn $return_fn_arg_size>]<'a, $($args),*, $return_type, $error_type> {
                    let r = [<provider_async_f $arg_size>](self, a);
                    r
                }
            }
        }
    };
}

macro_rules! generate_boxed_fn {
    ( [$($args:ident),*], $return_type:ident,$error_type:ident, $arg_size:expr ) => {

            //let x = count!($($args),*);
            crate::concat_idents!(boxed_fn_name = BoxedFn,$arg_size  {
                #[doc = concat!("Type alias  BoxedFn", stringify!($arg_size), "  for Boxed FnOnce sync function with ", stringify!($arg_size), " arguments")]
                pub type boxed_fn_name<'a, $($args),*, $return_type, $error_type> = Box<dyn FnOnce($($args),*) -> Result<$return_type, FnError<$error_type>> + Send + Sync + 'a>;
            });

            crate::concat_idents!(boxed_fn_name = BoxedAsyncFn,$arg_size  {
                #[doc = concat!("Type alias  BoxedAsyncFn", stringify!($arg_size), "  for Boxed FnOnce async function" , stringify!($arg_size), " arguments")]
                    pub type boxed_fn_name<'a, $($args),*, $return_type,$error_type> = Box<dyn FnOnce($($args),*) -> BoxFuture<'a, Result<$return_type, FnError<$error_type>>> + Send + Sync + 'a>;
                });

            paste!{
                #[doc = concat!("Function to box FnOnce sync function with ", stringify!($arg_size), " aguments and coerce it to BoxedFn",stringify!($arg_size))]
                pub fn [<lift_sync_fn $arg_size>]<'a, $($args),*, $return_type, $error_type, F: FnOnce($($args),*) -> Result<$return_type, FnError<$error_type>> + Send + Sync + 'a>(f: F,) -> [<BoxedFn $arg_size>]<'a, $($args),*, $return_type, $error_type> {
                    Box::new(f)
                }

                #[doc = concat!("Function to box  FnOnce sync function with ", stringify!($arg_size), " aguments and coerce it to BoxedAsyncFn",stringify!($arg_size))]
                pub fn [<lift_async_fn $arg_size>]<'a, $($args),*, $return_type, $error_type, F: FnOnce($($args),*) -> BoxFuture<'a,Result<$return_type, FnError<$error_type>>> + Send + Sync + 'a>(f: F,) -> [<BoxedAsyncFn $arg_size>]<'a, $($args),*, $return_type, $error_type> {
                    Box::new(f)
                }
            }
    }
}

generate_boxed_fn! {[T1], T2, E1, 1}

generate_boxed_fn!([T1, T2], T3, E1,  2);
impl_injector! {[T1],T2, T3, E1, 2, 1}

generate_boxed_fn!([T1, T2, T3], T4, E1, 3);
impl_injector!([T1, T2], T3, T4, E1, 3, 2);

generate_boxed_fn!([T1, T2, T3, T4], T5, E1, 4);
impl_injector!([T1, T2, T3], T4, T5, E1, 4, 3);

generate_boxed_fn!([T1, T2, T3, T4, T5], T6, E1,  5);
impl_injector!([T1, T2, T3, T4], T5, T6, E1, 5, 4);

generate_boxed_fn!([T1, T2, T3, T4, T5, T6], T7, E1, 6);
impl_injector!([T1, T2, T3, T4, T5], T6, T7, E1, 6, 5);

generate_boxed_fn!([T1, T2, T3, T4, T5, T6, T7], T8, E1, 7);
impl_injector!([T1, T2, T3, T4, T5, T6], T7, T8,E1, 7, 6);

generate_boxed_fn!([T1, T2, T3, T4, T5, T6, T7, T8], T9, E1, 8);
impl_injector!([T1, T2, T3, T4, T5, T6, T7], T8, T9, E1, 8, 7);

//Generates a function composition for BoxedFn1 as below. The below is example of composing sync with sync function.
//Similar code is generated for composing sync with async function, async with sync function and async with async function.
// impl<'a, T1: 'a + Send, T2: 'a + Send, T3: 'a>
// Then<'a, T1, T2, T3, BoxedFn1<'a, T2, T3>, BoxedFn1<'a, T1, T3>> for BoxedFn1<'a, T1, T2> {
//     fn then(self, f: BoxedFn1<'a, T2, T3>) -> BoxedFn1<'a, T1, T3> {
//         let r1 = move |x: T1| {
//             let b = self(x)?;
//             let r = f(b)?;
//             Ok(r)
//         };
//         Box::new(r1)
//     }
// }
composer_generator!(T1, T2, T3, E1, E2);





pub trait Injector<I, O> {
    fn provide(self, a: I) -> O;
}

///trait Then allows you to compose functions.
/// Type param A represents the function arg of Self
///
/// Type param B represents the return type of Self
///
///Type B also acts as the input arg of function f
///
/// Type C represents the return type of  function f

pub trait Then<'a, A, B, C, F, R> {

    /// Compose self with function f
    ///
    /// self:function(A)->B
    ///
    /// f:function(B)->C
    ///
    /// returns function(A)->C
    fn then(self, f: F) -> R;
}

#[macro_use]
pub mod macros {


    #[macro_export]
    macro_rules! compose {
        ($fnLeft:ident,$isLeftFnAsync:ident,-> with_args($args:expr) $($others:tt)*) => {
            {
            let r = $fnLeft($args);
            r
            }
        };

        ($fnLeft:ident,$isLeftFnAsync:ident,.provide($p1:expr) $($others:tt)*) => {
            {
                use function_compose::Injector;
                let p = $fnLeft.provide($p1);
                let p1 = compose!(p,$isLeftFnAsync,$($others)*);
                p1
            }
        };

        ($fLeft:ident,$isLeftFnAsync:ident,$fRight:ident, $isRightAsync:ident,  .provide($p:expr) $($others:tt)*) =>{
            {
                let fRight = $fRight.provide($p);
                let f3 = compose!($fLeft,$isLeftFnAsync,fRight,$isRightAsync,$($others)*);
                f3
            }
        };

        ($fLeft:ident,$isLeftFnAsync:ident,$fRight:ident, $isRightAsync:ident,  ->  $($others:tt)*) =>{
            {
                let fLeft = $fLeft.then($fRight);
                let isLeftFnAsync = $isRightAsync || $isLeftFnAsync;
                let f3 = compose!(fLeft,isLeftFnAsync, -> $($others)*);
                f3
            }
        };

        ($fLeft:ident,$isLeftFnAsync:ident,-> $fn:ident.provide($p:expr) $($others:tt)*) =>{
            {
                let f4;
                
                crate::concat_idents!(lifted_fn_name = fn_composer__lifted_fn,_, $fn {
                    paste!{
                    let is_retryable = [< fn_composer__is_retryable_ $fn >]();
                    let current_f = if !is_retryable{
                        [<fn_composer__lifted_fn_ $fn>]($fn)
                    }else {
                        [<fn_composer__lifted_fn_ $fn>]([< fn_composer__ retry_ $fn>])
                    };
                    }
                    crate::concat_idents!(asynCheckFn = fn_composer__is_async_, $fn {
                        let isRightAsync = asynCheckFn();
                        let fRight = current_f.provide($p);
                        let f3 = compose!($fLeft,$isLeftFnAsync,fRight,isRightAsync,$($others)*);
                        f4 = f3;
                    });
                });
                f4
            }
        };

        ($fLeft:ident,$isLeftFnAsync:ident,-> $fn:ident.provide($p:expr) $($others:tt)*) =>{
            {
                let f4;
                crate::concat_idents!(lifted_fn_name = fn_composer__lifted_fn,_, $fn {
                    let is_retryable = [<fn_composer__is_retryable $fn>]();
                    let current_f = if !is_retryable{
                        [<lifted_fn_name $fn>]($fn)
                    }else {
                        [<lifted_fn_name $fn>]([<fn_composer__ retry_ $fn>])
                    };
                    crate::concat_idents!(asynCheckFn = fn_composer__is_async_, $fn {
                        let isRightAsync = asynCheckFn();
                        let fRight = current_f.provide($p);
                        let f3 = compose!($fLeft,$isLeftFnAsync,fRight,isRightAsync,$($others)*);
                        f4 = f3;
                    });
                });
                f4
            }
        };


        ($fLeft:ident,$isLeftFnAsync:ident,-> $fn:ident $($others:tt)*) =>{
            {
                let f4;
                paste!{

                    let asynCheckFn = [<fn_composer__is_async_ $fn>];
                    let currentAsync = asynCheckFn();
                    let _isResultAsync = currentAsync || $isLeftFnAsync;
                    let is_retryable = [<fn_composer__is_retryable_ $fn>]();
                    let current_f = if !is_retryable{
                        [<fn_composer__lifted_fn_ $fn>]($fn)
                    }else {
                        [<fn_composer__lifted_fn_ $fn>]([<fn_composer__ retry_ $fn>])
                    };
                    let f3 = $fLeft.then(current_f);
                    let f3 = compose!(f3,_isResultAsync,$($others)*);
                    f4 = f3;
                }
                f4
            }
        };



        ($fn:ident $($others:tt)*) => {
            {

                use Then;
                let f2;
                paste!{
                    let f = [<fn_composer__lifted_fn_ $fn>]($fn);
                    let isAsync = [<fn_composer__is_async_ $fn>]();
                    let is_retryable = [<fn_composer__is_retryable_ $fn>]();
                    let f = if !is_retryable{
                        [<fn_composer__lifted_fn_ $fn>]($fn)
                    }else {
                        [<fn_composer__lifted_fn_ $fn>]([<fn_composer__ retry_ $fn>])
                    };
                    let f1 = compose!(f,isAsync,$($others)*);
                    f2 = f1;
                };
                f2
            }
        };


    }
}

