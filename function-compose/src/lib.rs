//uncomment below lines to debug macros

/*#![feature(trace_macros)]
trace_macros!(true);*/
//! Crate `function-compose` provides utilities for composing functions and way to inject arguments to functions
//! 
//! ## Composing functions
//! 
//! ### step 1
//!
//!  Mark a function as composeable as below. Note that the functions must always return Result type
//!
//! ```rust
//! use function_compose::composeable;
//! #[composeable()]
//! pub fn add_10(a: i32) -> Result<i32, String> {
//!     Ok(a + 10)
//! }
//! 
//! #[composeable()]
//! pub fn add_100(a: i32) -> Result<i32, String> {
//!     Ok(a + 100)
//! }
//! 
//! ```
//! ### step 2
//! 
//! use compose! macro to compose the above two functions.
//! 
//! ```ignore
//! let result = compose!(add_10 -> add_100 -> with_args(10));
//! assert_eq!(220, result.unwrap());
//! ```
//! Argument 10(from with_args(10)). is passed to add_10 function and result of add_10 is passed to add_100
//! 
//! ## composing Async functions
//! It is also possible to compose sync and asycn function.
//! ##### <font color="#FFBF00"> __For async function,  return type should be BoxedFuture(futures crate)__</font>
//! 
//! ```ignore
//! use function_compose::composeable;
//! use futures::{future::BoxFuture, FutureExt};
//! #[composeable()]
//! pub fn add_async(a: i32, b: i32) -> BoxFuture<'static, Result<i32, String>> {
//!     async move {
//!         let r = a + b;
//!         Ok(r)
//!     }.boxed()
//! }
//! ```
//! 
//! ### Composing async and sync functions usage
//!
//!```ignore
//! use function_compose::compose;
//! use fn_macros::composeable;
//! use futures::{future::BoxFuture, FutureExt};
//! #[composeable()]
//! pub fn add_10_async(a: i32) -> BoxFuture<'static, Result<i32, String>> {
//!     async move {
//!         let r = a + 10;
//!         Ok(r)
//!     }.boxed()
//! }
//! #[composeable()]
//! pub fn add_10(a: i32) -> Result<i32, String> {
//!     Ok(a + 10)
//! }
//! async fn test(){
//!    let result = compose!(add_async.add_10_async -> add_10 -> with_args(10)).await;
//!    assert_eq!(30, result.unwrap());
//! }
//! 
//! ```
//! 
//! ## Injecting dependencies in multi-args function
//! For function with multiple arguments(say 2), One of the argument can be injected during composition itself.
//! 
//! #### Function argument injection usage
//!```ignore
//! use function_compose::composeable;
//! use futures::{future::BoxFuture, FutureExt};
//! #[composeable()]
//! pub fn add_3_arg_async(a: i32,b: i32, c:i32) -> BoxFuture<'static, Result<i32, String>>{
//!     async move{
//!         let  r =   a + b + c;
//!         Ok(r)
//!     }.boxed()
//! }
//! use crate::compose;
//! let result = compose!(add_3_arg_async.provide(100).provide(200) -> add_10 -> with_args(10)).await;
//! assert_eq!(320, result.unwrap());
//!```
//! In the above example function add_3_arg_async, out of three arguments, 2 are injected during composing the function itself (using provide(100)) .
//! This feature could be used for injecting connection pool or a repository instance(see the example project).
//! 
//! ## Retry in Fn Composer

//!Composeable macro supports retrying a function at specified interval in case of Error returned by the function.
//!This could be useful when trying make a database call or connect to network endpoint.
//!Make sure to add https://docs.rs/retry/latest/retry/ to your project before proceeding with retry feature.
//!
//!Retry mechanism is implemented as part of composeable procedureal macro.
//!Below is example of  add_10  function configured to be retried 2 times after initial failure.
//!
//!```ignore
//!use retry::delay::*;
//!#[composeable(retry = Fixed::from_millis(100).take(2))]
//!pub fn add_10(a: i32) -> Result<i32, String> {
//!    Ok(a + 10)
//!}
//!
//!```

//!Retry can be applied to both sync and async functions.
//!
//!for async functions, <font color="#FFBF00"> __all arguments to the function must be either shared reference or exclusive reference.__ </font>
//!
//!Below is example of  async function with retry.
//!
//!```ignore
//!#[composeable(retry = Fixed::from_millis(100))]
//!pub fn add_3_arg_ref__non_copy_async<'a>(
//!    a: &'a mut Vec<String>,
//!    b: &'a mut Vec<String>,
//!    c: &'a Vec<String>,
//!) -> BoxFuture<'a, Result<i32, String>> {
//!    async move {
//!        let r = a.len() + b.len() + c.len();
//!        Ok(r as i32)
//!    }
//!    .boxed()
//!}
//!```
//!
//!Apart from fixed duration retries, it is possible to configure with exponential delay.
//!Refer to retry documentation for all available delay options https://docs.rs/retry/latest/retry/all.html


use futures::{future::BoxFuture, FutureExt};

fn to_fn_error<E1, E2>(error:E1) -> E2 where E2:From<E1>{
    From::from(error)    
}


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
                        let g_result = self(x);
                        match g_result{
                                Ok(inner_result) => f(inner_result),
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
                            let g_result = self(x);
                            match g_result{
                                Ok(inner_result) => f(inner_result).await,
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
                            let g_result = self(a).await;
                            match g_result{
                                Ok(inner_result) => f(inner_result),
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
                            let g_result = self(a).await;
                            match g_result{
                                Ok(inner_result) => f(inner_result).await,
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
                pub type boxed_fn_name<'a, $($args),*, $return_type, $error_type> = Box<dyn FnOnce($($args),*) -> Result<$return_type, $error_type> + Send + Sync + 'a>;
            });

            crate::concat_idents!(boxed_fn_name = BoxedAsyncFn,$arg_size  {
                #[doc = concat!("Type alias  BoxedAsyncFn", stringify!($arg_size), "  for Boxed FnOnce async function" , stringify!($arg_size), " arguments")]
                    pub type boxed_fn_name<'a, $($args),*, $return_type,$error_type> = Box<dyn FnOnce($($args),*) -> BoxFuture<'a, Result<$return_type, $error_type>> + Send + Sync + 'a>;
                });

            paste!{
                #[doc = concat!("Function to box FnOnce sync function with ", stringify!($arg_size), " aguments and coerce it to BoxedFn",stringify!($arg_size))]
                pub fn [<lift_sync_fn $arg_size>]<'a, $($args),*, $return_type, $error_type, F: FnOnce($($args),*) -> Result<$return_type, $error_type> + Send + Sync + 'a>(f: F,) -> [<BoxedFn $arg_size>]<'a, $($args),*, $return_type, $error_type> {
                    Box::new(f)
                }

                #[doc = concat!("Function to box  FnOnce sync function with ", stringify!($arg_size), " aguments and coerce it to BoxedAsyncFn",stringify!($arg_size))]
                pub fn [<lift_async_fn $arg_size>]<'a, $($args),*, $return_type, $error_type, F: FnOnce($($args),*) -> BoxFuture<'a,Result<$return_type, $error_type>> + Send + Sync + 'a>(f: F,) -> [<BoxedAsyncFn $arg_size>]<'a, $($args),*, $return_type, $error_type> {
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
        ($fnLeft:ident,$is_left_fn_async:ident,-> with_args($args:expr) $($others:tt)*) => {
            {
            let r = $fnLeft($args);
            r
            }
        };

        ($fnLeft:ident,$is_left_fn_async:ident,.provide($p1:expr) $($others:tt)*) => {
            {
                use function_compose::Injector;
                let p = $fnLeft.provide($p1);
                let p1 = compose!(p,$is_left_fn_async,$($others)*);
                p1
            }
        };

        ($f_left:ident,$is_left_fn_async:ident,$f_right:ident, $isRightAsync:ident,  .provide($p:expr) $($others:tt)*) =>{
            {
                let f_right = $f_right.provide($p);
                let f3 = compose!($f_left,$is_left_fn_async,f_right,$isRightAsync,$($others)*);
                f3
            }
        };

        ($f_left:ident,$is_left_fn_async:ident,$f_right:ident, $isRightAsync:ident,  ->  $($others:tt)*) =>{
            {
                let f_left = $f_left.then($f_right);
                let is_left_fn_async = $isRightAsync || $is_left_fn_async;
                let f3 = compose!(f_left,is_left_fn_async, -> $($others)*);
                f3
            }
        };

        ($f_left:ident,$is_left_fn_async:ident,-> $fn:ident.provide($p:expr) $($others:tt)*) =>{
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
                    crate::concat_idents!(asyn_check_fn = fn_composer__is_async_, $fn {
                        let is_right_async = asyn_check_fn();
                        let f_right = current_f.provide($p);
                        let f3 = compose!($f_left,$is_left_fn_async,f_right,is_right_async,$($others)*);
                        f4 = f3;
                    });
                });
                f4
            }
        };

        ($f_left:ident,$isLeftFnAsync:ident,-> $fn:ident.provide($p:expr) $($others:tt)*) =>{
            {
                let f4;
                crate::concat_idents!(lifted_fn_name = fn_composer__lifted_fn,_, $fn {
                    let is_retryable = [<fn_composer__is_retryable $fn>]();
                    let current_f = if !is_retryable{
                        [<lifted_fn_name $fn>]($fn)
                    }else {
                        [<lifted_fn_name $fn>]([<fn_composer__ retry_ $fn>])
                    };
                    crate::concat_idents!(asyn_check_fn = fn_composer__is_async_, $fn {
                        let isRightAsync = asyn_check_fn();
                        let f_right = current_f.provide($p);
                        let f3 = compose!($f_left,$is_left_fn_async,f_right,isRightAsync,$($others)*);
                        f4 = f3;
                    });
                });
                f4
            }
        };


        ($f_left:ident,$is_left_fn_async:ident,-> $fn:ident $($others:tt)*) =>{
            {
                let f4;
                paste!{

                    let asyn_check_fn = [<fn_composer__is_async_ $fn>];
                    let current_async = asyn_check_fn();
                    let _is_result_async = current_async || $is_left_fn_async;
                    let is_retryable = [<fn_composer__is_retryable_ $fn>]();
                    let current_f = if !is_retryable{
                        [<fn_composer__lifted_fn_ $fn>]($fn)
                    }else {
                        [<fn_composer__lifted_fn_ $fn>]([<fn_composer__ retry_ $fn>])
                    };
                    let f3 = $f_left.then(current_f);
                    let f3 = compose!(f3,_is_result_async,$($others)*);
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
                    let is_async = [<fn_composer__is_async_ $fn>]();
                    let is_retryable = [<fn_composer__is_retryable_ $fn>]();
                    let f = if !is_retryable{
                        [<fn_composer__lifted_fn_ $fn>]($fn)
                    }else {
                        [<fn_composer__lifted_fn_ $fn>]([<fn_composer__ retry_ $fn>])
                    };
                    let f1 = compose!(f,is_async,$($others)*);
                    f2 = f1;
                };
                f2
            }
        };


    }
}

