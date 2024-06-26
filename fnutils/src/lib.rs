//un comment below lines to debug macros

/*#![feature(trace_macros)]
trace_macros!(true);*/


use std::{error::Error, fmt::Display, ops::Deref};
use std::env::Args;


use futures::{future::BoxFuture, FutureExt};
use paste::paste;


//pub type FnError = Box<dyn Error>;

#[derive(Debug)]
pub struct FnError{
    pub underlying_error: Option<Box<dyn Error + Send>>,
    pub error_code:Option<String>,
    pub description: Option<String>
}
macro_rules! composer_generator {
    ($arg1:ident, $return_type1:ident, $return_type2:ident) => {
        paste!{
            #[doc = concat!("Then implementation for composing sync function (BoxedFn1) with another sync function(BoxedFn1) ")]
            impl<'a, $arg1: 'a + Send, $return_type1: 'a + Send, $return_type2: 'a>
                Then<'a, $arg1, $return_type1, $return_type2, BoxedFn1<'a, $return_type1, $return_type2>, BoxedFn1<'a, $arg1, $return_type2>> for BoxedFn1<'a, $arg1, $return_type1>{

                fn then(self, f: BoxedFn1<'a, $return_type1, $return_type2>) -> BoxedFn1<'a, $arg1, $return_type2> {
                    let r1 = move |x: $arg1| {
                        let b = self(x)?;
                        let r = f(b)?;
                        Ok(r)
                    };
                    Box::new(r1)
                }
            }

            #[doc = concat!("Then implementation for composing sync function(BoxedFn1) with another async function(BoxedAsyncFn1) ")]
            impl<'a, $arg1: 'a + Send, $return_type1: 'a + Send, $return_type2: 'a>
                Then<'a, $arg1, $return_type1, $return_type2, BoxedAsyncFn1<'a, $return_type1, $return_type2>, BoxedAsyncFn1<'a, $arg1, $return_type2>> for BoxedFn1<'a, $arg1, $return_type1>{

                fn then(self, f: BoxedAsyncFn1<'a, $return_type1, $return_type2>) -> BoxedAsyncFn1<'a, $arg1, $return_type2> {
                    let r1 =  |x: $arg1| {
                        async move{
                            let b = self(x)?;
                            f(b).await
                        }.boxed()
                    };
                    Box::new(r1)
                }
            }

            #[doc = concat!("Then implementation for composing async function(BoxedAsyncFn1) with another sync function(BoxedFn1) ")]

            impl<'a, $arg1: 'a + Send, $return_type1: 'a + Send, $return_type2: 'a>
                Then<'a, $arg1, $return_type1, $return_type2, BoxedFn1<'a, $return_type1, $return_type2>, BoxedAsyncFn1<'a, $arg1, $return_type2>> for BoxedAsyncFn1<'a, $arg1, $return_type1>{

                fn then(self, f: BoxedFn1<'a, $return_type1, $return_type2>) -> BoxedAsyncFn1<'a, $arg1, $return_type2> {
                    let r1 = |a: $arg1| {
                        async move {
                            let gResult = self(a).await?;
                            f(gResult)
                        }.boxed()
                    };
                    let r: BoxedAsyncFn1<'a,$arg1, $return_type2> = Box::new(r1);
                    r
                }
            }


            #[doc = concat!("Then implementation for composing async function(BoxedAsyncFn1) with another async function(BoxedAsyncFn1) ")]
            impl<'a, $arg1: 'a + Send, $return_type1: 'a + Send, $return_type2: 'a>
                Then<'a, $arg1, $return_type1, $return_type2, BoxedAsyncFn1<'a, $return_type1, $return_type2>, BoxedAsyncFn1<'a, $arg1, $return_type2>> for BoxedAsyncFn1<'a, $arg1, $return_type1>{

                fn then(self, f: BoxedAsyncFn1<'a, $return_type1, $return_type2>) -> BoxedAsyncFn1<'a, $arg1, $return_type2> {
                    let r1 = |a: $arg1| {
                        async move {
                            let gResult = self(a).await?;
                            f(gResult).await
                        }.boxed()
                    };
                    let r: BoxedAsyncFn1<'a,$arg1, $return_type2> = Box::new(r1);
                    r
                }
            }
        }
    }
}
macro_rules! impl_injector {
    ([$($args:ident),*], $provided:ident, $return_type:ident,  $arg_size:literal, $return_fn_arg_size:literal) => {

        paste!  {
            #[doc = concat!("dependency injection function provide_f", stringify!($arg_size), " for injecting the last argument of a given sync function")]
            pub fn [<provider_f $arg_size>]<'a, $($args),*, $provided, $return_type>(fn1: [<BoxedFn $arg_size>]<'a, $($args),*, $provided, $return_type>,provided_data: $provided,) -> [<BoxedFn $return_fn_arg_size>]<'a, $($args),* , $return_type> where $( $args: 'a ),*, $provided: Send + Sync + 'a, $return_type: 'a{
                    Box::new(move |$( [<$args:lower>]:$args ),*| fn1($( [<$args:lower>]),*,  provided_data))
            }

            #[doc = concat!("dependency injection function provider_async_f", stringify!($arg_size), " for injecting the last argument of a given async function")]
            pub fn [<provider_async_f $arg_size>]<'a, $($args),*, $provided, $return_type>(fn1: [<BoxedAsyncFn $arg_size>]<'a, $($args),*, $provided, $return_type>,provided_data: $provided,) -> [<BoxedAsyncFn $return_fn_arg_size>]<'a, $($args),* , $return_type> where $( $args: 'a ),*, $provided: Send + Sync + 'a, $return_type: 'a{
                    Box::new(move |$( [<$args:lower>]:$args ),*| fn1($( [<$args:lower>]),*,  provided_data))
            }

        }
        paste!{

            #[doc = concat!("Injector implementation for a given sync function that accepts " , stringify!($return_fn_arg_size+1), " arguments and returns a function with ", stringify!($return_fn_arg_size), " arguments")]
            impl<'a, $($args),*, $provided, $return_type> Injector<$provided, [<BoxedFn $return_fn_arg_size>]<'a, $($args),*, $return_type>> for [<BoxedFn $arg_size>] <'a, $($args),*, $provided, $return_type>
            where $( $args: 'a ),*, $provided: Send + Sync +'a, $return_type: 'a
            {
                fn provide(self, a: $provided) -> [<BoxedFn $return_fn_arg_size>]<'a, $($args),*, $return_type> {
                    let r = [<provider_f $arg_size>](self, a);
                    r
                }
            }

            #[doc = concat!("Injector implementation for a given async function that accepts " , stringify!($return_fn_arg_size+1), " arguments  and returns a function with ", stringify!($return_fn_arg_size), " arguments")]
            impl<'a, $($args),*, $provided, $return_type> Injector<$provided, [<BoxedAsyncFn $return_fn_arg_size>]<'a, $($args),*, $return_type>> for [<BoxedAsyncFn $arg_size>] <'a, $($args),*, $provided, $return_type>
            where $( $args: 'a ),*, $provided: Send + Sync +'a, $return_type: 'a
            {
                fn provide(self, a: $provided) -> [<BoxedAsyncFn $return_fn_arg_size>]<'a, $($args),*, $return_type> {
                    let r = [<provider_async_f $arg_size>](self, a);
                    r
                }
            }
        }
    };
}

macro_rules! generate_boxed_fn {
    ( [$($args:ident),*], $return_type:ident, $arg_size:expr ) => {

            //let x = count!($($args),*);
            concat_idents::concat_idents!(boxed_fn_name = BoxedFn,$arg_size  {
                #[doc = concat!("Type alias  BoxedFn", stringify!($arg_size), "  for Boxed FnOnce sync function with ", stringify!($arg_size), " arguments")]
                pub type boxed_fn_name<'a, $($args),*, $return_type,> = Box<dyn FnOnce($($args),*) -> Result<$return_type, FnError> + Send + Sync + 'a>;
            });

            concat_idents::concat_idents!(boxed_fn_name = BoxedAsyncFn,$arg_size  {
                #[doc = concat!("Type alias  BoxedAsyncFn", stringify!($arg_size), "  for Boxed FnOnce async function" , stringify!($arg_size), " arguments")]
                    pub type boxed_fn_name<'a, $($args),*, $return_type,> = Box<dyn FnOnce($($args),*) -> BoxFuture<'a, Result<$return_type, FnError>> + Send + Sync + 'a>;
                });

            paste!{
                #[doc = concat!("Function to box FnOnce sync function with ", stringify!($arg_size), " aguments and coerce it to BoxedFn",stringify!($arg_size))]
                pub fn [<lift_sync_fn $arg_size>]<'a, $($args),*, $return_type, F: FnOnce($($args),*) -> Result<$return_type, FnError> + Send + Sync + 'a>(f: F,) -> [<BoxedFn $arg_size>]<'a, $($args),*, $return_type> {
                    Box::new(f)
                }

                #[doc = concat!("Function to box  FnOnce sync function with ", stringify!($arg_size), " aguments and coerce it to BoxedAsyncFn",stringify!($arg_size))]
                pub fn [<lift_async_fn $arg_size>]<'a, $($args),*, $return_type, F: FnOnce($($args),*) -> BoxFuture<'a,Result<$return_type, FnError>> + Send + Sync + 'a>(f: F,) -> [<BoxedAsyncFn $arg_size>]<'a, $($args),*, $return_type> {
                    Box::new(f)
                }
            }
    }
}

generate_boxed_fn! {[T1], T2, 1}

generate_boxed_fn!([T1, T2], T3, 2);
impl_injector! {[T1],T2, T3, 2, 1}

generate_boxed_fn!([T1, T2, T3], T4, 3);
impl_injector!([T1, T2], T3, T4, 3, 2);

generate_boxed_fn!([T1, T2, T3, T4], T5, 4);
impl_injector!([T1, T2, T3], T4, T5, 4, 3);

generate_boxed_fn!([T1, T2, T3, T4, T5], T6, 5);
impl_injector!([T1, T2, T3, T4], T5, T6, 5, 4);

generate_boxed_fn!([T1, T2, T3, T4, T5, T6], T7, 6);
impl_injector!([T1, T2, T3, T4, T5], T6, T7, 6, 5);

generate_boxed_fn!([T1, T2, T3, T4, T5, T6, T7], T8, 7);
impl_injector!([T1, T2, T3, T4, T5, T6], T7, T8, 7, 6);

generate_boxed_fn!([T1, T2, T3, T4, T5, T6, T7, T8], T9, 8);
impl_injector!([T1, T2, T3, T4, T5, T6, T7], T8, T9, 8, 7);

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
composer_generator!(T1, T2, T3);

#[derive(Debug)]
pub enum ErrorType {
    //User Id
    UserNotFound(String),
    AuthError(String),
    // Role ID
    RoleNotFound(String),
    Unknown(String),
    DBInitError,
    DBError(String),
    InvalidInput(String),
    EmailAlreadyTaken(String),
}




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
    macro_rules! c1 {
        ($fLeft:ident, $current_f:ident, true, true) => {
            $fLeft.then_async(current_f)
        };

        ($fLeft:ident, $current_f:ident, true, false) => {
            $fLeft.then_sync(current_f)
        };

        ($fLeft:ident, $current_f:ident, false, false) => {
            $fLeft.then_sync(current_f)
        };

        ($fLeft:ident, $current_f:ident, false, true) => {
            $fLeft.then_async(current_f)
        };
    }

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
                use fnutils::Injector;
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
                
                concat_idents::concat_idents!(lifted_fn_name = fn_composer__lifted_fn,_, $fn {
                    paste!{
                    let is_retryable = [< fn_composer__is_retryable_ $fn >]();
                    let current_f = if !is_retryable{
                        [<fn_composer__lifted_fn_ $fn>]($fn)
                    }else {
                        [<fn_composer__lifted_fn_ $fn>]([< fn_composer__ retry_ $fn>])
                    };
                    }
                    concat_idents::concat_idents!(asynCheckFn = fn_composer__is_async_, $fn {
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
                concat_idents::concat_idents!(lifted_fn_name = fn_composer__lifted_fn,_, $fn {
                    let is_retryable = [<fn_composer__is_retryable $fn>]();
                    let current_f = if !is_retryable{
                        [<lifted_fn_name $fn>]($fn)
                    }else {
                        [<lifted_fn_name $fn>]([<fn_composer__ retry_ $fn>])
                    };
                    concat_idents::concat_idents!(asynCheckFn = fn_composer__is_async_, $fn {
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
                use paste::paste;
                use fnutils::Then;
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
