#![feature(trace_macros)]

use std::{error::Error, fmt::Display, mem::discriminant, ops::Deref};

use futures::{future::BoxFuture, FutureExt};
use paste::paste;

trace_macros!(true);
#[rustfmt::skip]
macro_rules! all_the_tuples {
    ($name:ident) => {
        $name!([], T1);
        $name!([T1], T2);
        $name!([T1, T2], T3);
        $name!([T1, T2, T3], T4);
        $name!([T1, T2, T3, T4], T5);
        $name!([T1, T2, T3, T4, T5], T6);
        $name!([T1, T2, T3, T4, T5, T6], T7);
        $name!([T1, T2, T3, T4, T5, T6, T7], T8);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8], T9);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9], T10);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10], T11);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11], T12);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12], T13);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13], T14);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14], T15);
        $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15], T16);
    };
}

macro_rules! count_ident_size (
    () => (0usize);
    ( $x:tt, $($xs:tt),* ) => (1usize + count!($($xs)*));
);

macro_rules! sub_one {
    () => {
        const fn sub_one1(a:u32)->u32{
            a - 1
        }
    }
}

sub_one!();

pub fn p2<'a, A: 'a, B: Send + Sync + 'a, C: 'a>(
    fn1: BoxedFn2<'a, A, B, C>,
    b: B,
) -> BoxedFn1<'a, A, C> {
    Box::new(move |a: A| fn1(a, b))
}

pub fn lift<'a, A, B, F: Fn(A) -> Result<B, FnError> + Send + Sync + 'a>(
    f: F,
) -> BoxedAppFn<'a, A, B> {
    Box::new(f)
}

macro_rules! lift_fn_generator {
    ([$($args:ident),*], $return_type:ident, $arg_size:literal) => {
        paste!{
            pub fn [<lift $arg_size>]<'a, $($args),*, $return_type, F: FnOnce($($args),*) -> Result<$return_type, FnError> + Send + Sync + 'a>(f: F,) -> [<BoxedFn $arg_size>]<'a, $($args),*, $return_type> {
                Box::new(f)
            }
        }
    }
}
//[T1][T2], [T2],[T3]
/*impl<'a, T1: 'a + Send, T2: 'a + Send, T3: 'a>
AndThenSync<'a, T1, T2, T3, BoxedFn1<'a, T2, T3>, BoxedFn1<'a, T1, T3>> for BoxedFn1<'a, T1, T2> {
    fn then_sync(self, f: BoxedFn1<'a, T2, T3>) -> BoxedFn1<'a, T1, T3> {
        let r1 = move |x: T1| {
            let b = self(x)?;
            let r = f(b)?;
            Ok(r)
        };
        Box::new(r1)
    }
}
impl<'a, T1: 'a + Send, T2: 'a + Send, T3: 'a>
AndThenAsync<'a, T1, T2, T3, BoxedAsyncFn1<'a, T2, T3>, BoxedAsyncFn1<'a, T1, T3>> for BoxedFn1<'a, T1, T2> {
    fn then_async(self, f: BoxedAsyncFn1<'a, T2, T3>) -> BoxedAsyncFn1<'a, T1, T3> {
        let r1 = |x: T1| {
            async move {
                let b = self(x)?;
                f(b).await
            }.boxed()
        };
        Box::new(r1)
    }
}
impl<'a, T1: 'a + Send, T2: 'a + Send, T3: 'a>
AndThenSync<'a, T1, T2, T3, BoxedFn1<'a, T2, T3>, BoxedAsyncFn1<'a, T1, T3>> for BoxedAsyncFn1<'a, T1, T2> {
    fn then_sync(self, f: BoxedFn1<'a, T2, T3>) -> BoxedAsyncFn1<'a, T1, T3> {
        let r1 = |a: T1| {
            async move {
                let gResult = self(a).await?;
                f(gResult)
            }.boxed()
        };
        let r: BoxedAsyncFn1<'a, T1, T3> = Box::new(r1);
        r
    }
}
impl<'a, T1: 'a + Send, T2: 'a + Send, T3: 'a>
AndThenAsync<'a, T1, T2, T3, BoxedAsyncFn1<'a, T2, T3>, BoxedAsyncFn1<'a, T1, T3>> for BoxedAsyncFn1<'a, T1, T2> {
    fn then_async(self, f: BoxedAsyncFn1<'a, T2, T3>) -> BoxedAsyncFn1<'a, T1, T3> {
        let r1 = |a: T1| {
            async move {
                let gResult = self(a).await?;
                f(gResult).await
            }.boxed()
        };
        let r: BoxedAsyncFn1<'a, T1, T3> = Box::new(r1);
        r
    }
}*/


macro_rules! composer_generator {
    ($arg1:ident, $return_type1:ident, $return_type2:ident) => {
        paste!{
            impl<'a, $arg1: 'a + Send, $return_type1: 'a + Send, $return_type2: 'a>
                AndThenSync<'a, $arg1, $return_type1, $return_type2, BoxedFn1<'a, $return_type1, $return_type2>, BoxedFn1<'a, $arg1, $return_type2>> for BoxedFn1<'a, $arg1, $return_type1>{

                fn then_sync(self, f: BoxedFn1<'a, $return_type1, $return_type2>) -> BoxedFn1<'a, $arg1, $return_type2> {
                    let r1 = move |x: $arg1| {
                        let b = self(x)?;
                        let r = f(b)?;
                        Ok(r)
                    };
                    Box::new(r1)
                }
            }

            impl<'a, $arg1: 'a + Send, $return_type1: 'a + Send, $return_type2: 'a>
                AndThenAsync<'a, $arg1, $return_type1, $return_type2, BoxedAsyncFn1<'a, $return_type1, $return_type2>, BoxedAsyncFn1<'a, $arg1, $return_type2>> for BoxedFn1<'a, $arg1, $return_type1>{

                fn then_async(self, f: BoxedAsyncFn1<'a, $return_type1, $return_type2>) -> BoxedAsyncFn1<'a, $arg1, $return_type2> {
                    let r1 =  |x: $arg1| {
                        async move{
                            let b = self(x)?;
                            f(b).await
                        }.boxed()
                    };
                    Box::new(r1)
                }
            }

            impl<'a, $arg1: 'a + Send, $return_type1: 'a + Send, $return_type2: 'a>
                AndThenSync<'a, $arg1, $return_type1, $return_type2, BoxedFn1<'a, $return_type1, $return_type2>, BoxedAsyncFn1<'a, $arg1, $return_type2>> for BoxedAsyncFn1<'a, $arg1, $return_type1>{

                fn then_sync(self, f: BoxedFn1<'a, $return_type1, $return_type2>) -> BoxedAsyncFn1<'a, $arg1, $return_type2> {
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
            
            

            impl<'a, $arg1: 'a + Send, $return_type1: 'a + Send, $return_type2: 'a>
                AndThenAsync<'a, $arg1, $return_type1, $return_type2, BoxedAsyncFn1<'a, $return_type1, $return_type2>, BoxedAsyncFn1<'a, $arg1, $return_type2>> for BoxedAsyncFn1<'a, $arg1, $return_type1>{

                fn then_async(self, f: BoxedAsyncFn1<'a, $return_type1, $return_type2>) -> BoxedAsyncFn1<'a, $arg1, $return_type2> {
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
            
            impl<'a, $arg1: 'a + Send, $return_type1: 'a + Send, $return_type2: 'a>
                AndThenAsync<'a, $arg1, $return_type1, $return_type2, BoxedFn1<'a, $return_type1, $return_type2>, BoxedAsyncFn1<'a, $arg1, $return_type2>> for BoxedFn1<'a, $arg1, $return_type1>{

                fn then_async(self, f: BoxedFn1<'a, $return_type1, $return_type2>) -> BoxedAsyncFn1<'a, $arg1, $return_type2> {
                    //It is implemented only to keep the compose macro working
                    panic!("code not to be reached")
                }
            }
        }
    }
}

//composer_generator!(T1,T2,T3);
/*impl<'a, T1: 'a + Send, T2: 'a + Send, T3: 'a>
AndThen1<'a, T1, T2, T3, BoxedAsyncFn1<'a, T2, T3>, BoxedAsyncFn1<'a, T1, T3>> for BoxedAsyncFn1<'a, T1, T2> {
    fn then1(self, f: BoxedAsyncFn1<'a, T2, T3>) -> BoxedAsyncFn1<'a, T1, T3> {
        let r1 = |a: T1| {
            async move {
                let gResult = self(a).await?;
                f(gResult).await
            }.boxed()
        };
        let r: BoxedAsyncFn1<'a, T1, T3> = r1.boxed();
        r
    }
}*/

//let x = |a: A| self(a, b);
//         Box::new(x)
macro_rules! impl_injector {
    ([$($args:ident),*], $provided:ident, $return_type:ident,  $arg_size:literal, $return_fn_arg_size:literal) => {

        paste!  {

            pub fn [<provider_f $arg_size>]<'a, $($args),*, $provided, $return_type>(fn1: [<BoxedFn $arg_size>]<'a, $($args),*, $provided, $return_type>,provided_data: $provided,) -> [<BoxedFn $return_fn_arg_size>]<'a, $($args),* , $return_type> where $( $args: 'a ),*, $provided: Send + Sync + 'a, $return_type: 'a{
                    Box::new(move |$( [<$args:lower>]:$args ),*| fn1($( [<$args:lower>]),*,  provided_data))
            }

            pub fn [<provider_async_f $arg_size>]<'a, $($args),*, $provided, $return_type>(fn1: [<BoxedAsyncFn $arg_size>]<'a, $($args),*, $provided, $return_type>,provided_data: $provided,) -> [<BoxedAsyncFn $return_fn_arg_size>]<'a, $($args),* , $return_type> where $( $args: 'a ),*, $provided: Send + Sync + 'a, $return_type: 'a{
                    Box::new(move |$( [<$args:lower>]:$args ),*| fn1($( [<$args:lower>]),*,  provided_data))
            }
        
        }
        paste!{
//where $( $args: 'a ),*, 
            impl<'a, $($args),*, $provided, $return_type> OwnedInjecter<$provided, [<BoxedFn $return_fn_arg_size>]<'a, $($args),*, $return_type>> for [<BoxedFn $arg_size>] <'a, $($args),*, $provided, $return_type>
            where $( $args: 'a ),*, $provided: Send + Sync +'a, $return_type: 'a
            {
                fn provide(self, a: $provided) -> [<BoxedFn $return_fn_arg_size>]<'a, $($args),*, $return_type> {
                    let r = [<provider_f $arg_size>](self, a);
                    r
                }
            }

            impl<'a, $($args),*, $provided, $return_type> OwnedInjecter<$provided, [<BoxedAsyncFn $return_fn_arg_size>]<'a, $($args),*, $return_type>> for [<BoxedAsyncFn $arg_size>] <'a, $($args),*, $provided, $return_type>
            where $( $args: 'a ),*, $provided: Send + Sync +'a, $return_type: 'a
            {
                fn provide(self, a: $provided) -> [<BoxedAsyncFn $return_fn_arg_size>]<'a, $($args),*, $return_type> {
                    let r = [<provider_async_f $arg_size>](self, a);
                    r
                }
            }
        }
    };

    (@gen_type_lifetime_bounds( $first:ident, $($rest:ident),*, $lifetime:lifetime )) => (
        $first: $lifetime, @gen_type_lifetime_bounds($($rest:ident),*, $lifetime)
    );
}

macro_rules! generate_boxed_fn {
    ( [$($args:ident),*], $return_type:ident, $arg_size:expr ) => {

            //let x = count!($($args),*);
            concat_idents::concat_idents!(boxed_fn_name = BoxedFn,$arg_size  {
                pub type boxed_fn_name<'a, $($args),*, $return_type,> = Box<dyn FnOnce($($args),*) -> Result<$return_type, FnError> + Send + Sync + 'a>;
            });

            concat_idents::concat_idents!(boxed_fn_name = BoxedAsyncFn,$arg_size  {
                //Box<dyn FnOnce(A) -> BoxFuture<'a, Result<B, FnError>> + Send + Sync + 'a>;
                    pub type boxed_fn_name<'a, $($args),*, $return_type,> = Box<dyn FnOnce($($args),*) -> BoxFuture<'a, Result<$return_type, FnError>> + Send + Sync + 'a>;
                });
        
            paste!{
                pub fn [<lift_sync_fn $arg_size>]<'a, $($args),*, $return_type, F: FnOnce($($args),*) -> Result<$return_type, FnError> + Send + Sync + 'a>(f: F,) -> [<BoxedFn $arg_size>]<'a, $($args),*, $return_type> {
                    Box::new(f)
                }
                
                pub fn [<lift_async_fn $arg_size>]<'a, $($args),*, $return_type, F: FnOnce($($args),*) -> BoxFuture<'a,Result<$return_type, FnError>> + Send + Sync + 'a>(f: F,) -> [<BoxedAsyncFn $arg_size>]<'a, $($args),*, $return_type> {
                    Box::new(f)
                }
            }
    }
}

/*pub fn provider_f5<'a, T1, T2, T3, T4>(fn1: BoxedFn3<'a, T1, T2, T3,
    T4>, provided_data: T3, ) -> BoxedFn2<'a, T1, T2, T4> where T1: 'a, T2: 'a, T3: Send + Sync + 'a, T4: 'a {
    Box::new(move |t1: T1, t2: T2| fn1(t1, t2, provided_data))
}

pub fn provider_f7<'a, T1, T2, T3>(fn1: BoxedFn2<'a, T1, T2,
    T3>, provided_data: T2, ) -> BoxedFn1<'a, T1, T3> where T1: 'a, T2: Send + Sync + 'a, T3: 'a {
    Box::new(move |t1: T1| fn1(t1, provided_data))
}
impl<'a, T1, T2, T3> OwnedInjecter<T2, BoxedFn1<'a, T1, T3>> for BoxedFn2<'a, T1, T2, T3>
    where T1: 'a, T2: Send + Sync + 'a, T3: 'a
{
    fn provide(self, a: T2) -> BoxedFn1<'a, T1, T3> {
        let r = provider_f7(self, a);
        r
    }
}*/

/*impl<'a, T1: 'a + Send, T2: 'a + Send, T3: 'a>
AndThen1<'a, T1, T2, T3, BoxedFn1<'a, T2, T3>, BoxedFn1<'a, T1, T3>> for BoxedFn1<'a, T1, T2>
{
    fn then1(self, f: BoxedFn1<'a, T2, T3>) -> BoxedFn1<'a, T1, T3> {
        let r1 = move |x: T1| {
            let b = self(x)?;
            let r = f(b)?;
            Ok(r)
        };
        Box::new(r1)
    }
}*/

impl<'a, T1: 'a + Send, T2: 'a + Send, T3: 'a>
AndThen<'a, T1, T2, T3, BoxedFn1<'a, T2, T3>, BoxedFn1<'a, T1, T3>> for BoxedFn1<'a, T1, T2> {
    fn then(self, f: BoxedFn1<'a, T2, T3>) -> BoxedFn1<'a, T1, T3> {
        let r1 = move |x: T1| {
            let b = self(x)?;
            let r = f(b)?;
            Ok(r)
        };
        Box::new(r1)
    }
}
impl<'a, T1: 'a + Send, T2: 'a + Send, T3: 'a>
AndThen<'a, T1, T2, T3, BoxedAsyncFn1<'a, T2, T3>, BoxedAsyncFn1<'a, T1, T3>> for BoxedFn1<'a, T1, T2> {
    fn then(self, f: BoxedAsyncFn1<'a, T2, T3>) -> BoxedAsyncFn1<'a, T1, T3> {
        let r1 = |x: T1| {
            async move {
                let b = self(x)?;
                f(b).await
            }.boxed()
        };
        Box::new(r1)
    }
}


impl<'a, T1: 'a + Send, T2: 'a + Send, T3: 'a>
AndThen<'a, T1, T2, T3, BoxedFn1<'a, T2, T3>, BoxedAsyncFn1<'a, T1, T3>> for BoxedAsyncFn1<'a, T1, T2> {
    fn then(self, f: BoxedFn1<'a, T2, T3>) -> BoxedAsyncFn1<'a, T1, T3> {
        let r1 = |a: T1| {
            async move {
                let gResult = self(a).await?;
                f(gResult)
            }.boxed()
        };
        let r: BoxedAsyncFn1<'a, T1, T3> = Box::new(r1);
        r
    }
}
impl<'a, T1: 'a + Send, T2: 'a + Send, T3: 'a>
AndThen<'a, T1, T2, T3, BoxedAsyncFn1<'a, T2, T3>, BoxedAsyncFn1<'a, T1, T3>> for BoxedAsyncFn1<'a, T1, T2> {
    fn then(self, f: BoxedAsyncFn1<'a, T2, T3>) -> BoxedAsyncFn1<'a, T1, T3> {
        let r1 = |a: T1| {
            async move {
                let gResult = self(a).await?;
                f(gResult).await
            }.boxed()
        };
        let r: BoxedAsyncFn1<'a, T1, T3> = Box::new(r1);
        r
    }
}


generate_boxed_fn! {[T1], T2, 1}
//lift_fn_generator!{[T1], T2, 1}
generate_boxed_fn!([T1, T2], T3, 2);
//lift_fn_generator!([T1, T2], T3, 2);
impl_injector!{[T1],T2, T3, 2, 1}
//composer_generator!(T1,T2,T3);
generate_boxed_fn!([T1, T2, T3], T4, 3);
impl_injector!([T1, T2], T3, T4, 3, 2);
//lift_fn_generator!([T1, T2, T3], T4, 3);
generate_boxed_fn!([T1, T2, T3, T4], T5, 4);
//f(A) for A -> A
//f(B) for A -> A

//test
pub type AppFn<'a, A, B> = (dyn Fn(A) -> Result<B, FnError> + Send);

pub type AppFn2<'a, A, B, C> = &'a dyn Fn(A, B) -> Result<C, FnError>;
pub type AppFn3<'a, A, B, C, D> = &'a dyn Fn(A, B, C) -> Result<D, FnError>;

pub type BoxedFutureAppFn<'a, A, B> =
    Box<dyn FnOnce(A) -> BoxFuture<'a, Result<B, FnError>> + Send + Sync>;

pub type BoxedAppFn<'a, A, B> = Box<dyn Fn(A) -> Result<B, FnError> + Send + Sync + 'a>;
pub type BoxedFutAppFnOnce<'a, A, B> =
    Box<dyn FnOnce(A) -> BoxFuture<'a, Result<B, FnError>> + Send + Sync + 'a>;
pub type BoxedFutAppTwoArgFn<'a, A, B, C> =
    Box<dyn Fn(A, B) -> BoxFuture<'a, Result<C, FnError>> + Send + Sync + 'a>;

pub type BoxedAppFnOnce<'a, A, B> = Box<dyn FnOnce(A) -> Result<B, FnError> + 'a>;

pub type BoxedAppFn2<'a, A, B, C> = Box<dyn Fn(A, B) -> Result<C, FnError> + 'a>;
pub type BoxedAppFn3<'a, A, B, C, D> = Box<dyn Fn(A, B, C) -> Result<D, FnError> + 'a>;

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

impl Into<FnError> for ErrorType {
    fn into(self) -> FnError {
        FnError {
            underlyingError: None,
            errorType: self,
        }
    }
}

#[derive(Debug)]
pub struct FnError {
    pub underlyingError: Option<Box<dyn Error>>,
    pub errorType: ErrorType,
}

unsafe impl Send for FnError {}
unsafe impl Sync for FnError {}

#[derive(Debug)]
struct UnderlyingError {}
impl Display for UnderlyingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self)
    }
}

impl Error for UnderlyingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

pub type AppResult<T> = Result<T, FnError>;
pub trait ToAppResult<T> {
    fn to_app_result(self) -> AppResult<T>;
}

impl<T, E> ToAppResult<T> for Result<T, E>
where
    E: Error + 'static,
{
    fn to_app_result(self) -> AppResult<T> {
        match self {
            Ok(r) => Ok(r),
            Err(e) => Err(FnError {
                errorType: ErrorType::Unknown(e.to_string()),
                underlyingError: Some(Box::new(e)),
            }),
        }
    }
}

pub struct ErrorMapper<K, F> {
    errorMap: Vec<(K, F)>,
}

impl<K, F> ErrorMapper<K, F>
where
    F: FnOnce() -> ErrorType,
{
    pub fn new() -> ErrorMapper<K, F> {
        ErrorMapper {
            errorMap: Vec::new(),
        }
    }

    pub fn add(&mut self, k: K, f: F) -> &mut Self {
        self.errorMap.push((k, f));
        self
    }

    fn getErrorType(&self, k: &K) -> Option<&(K, F)> {
        self.errorMap
            .iter()
            .find(|item| discriminant(&item.0) == discriminant(k))
    }
}

pub fn convertToAppError<K, F>(e: &ErrorMapper<K, F>, err: K) -> FnError
where
    F: Fn() -> ErrorType,
    K: Display,
{
    let matchingErrorType = e.getErrorType(&err);
    if (matchingErrorType.is_some()) {
        let borrow_mut = matchingErrorType.unwrap();
        let var_name = &borrow_mut.1;
        var_name().into()
    } else {
        ErrorType::Unknown(err.to_string()).into()
    }
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for FnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = &self.underlyingError;
        write!(
            f,
            "({}, {})",
            self.errorType,
            match x {
                Some(e) => e.deref().to_string(),
                None => "None".to_string(),
            }
        )
    }
}

impl Error for FnError {}

impl From<String> for FnError {
    fn from(_value: String) -> Self {
        FnError {
            errorType: ErrorType::Unknown(_value),
            underlyingError: None,
        }
    }
}

pub fn compose_fn<'a, A: 'a, B: 'a, C: 'a>(
    f: BoxedAppFn<'a, B, C>,
    g: BoxedAppFn<'a, A, B>,
) -> BoxedAppFn<'a, A, C> {
    let r1 = move |x: A| {
        let b = g(x)?;
        let r = f(b)?;
        Ok(r)
    };

    Box::new(r1)
}

pub fn p1<'a, A: 'a, B: 'a, C: 'a>(
    fn1: BoxedAppFn2<'a, A, B, C>,
    b: B,
) -> BoxedAppFnOnce<'a, A, C> {
    Box::new(move |a: A| fn1(a, b))
}



pub trait OwnedInjecter<I, O> {
    fn provide(self, a: I) -> O;
}

impl<'a, A: 'a, B: 'a, C: 'a> OwnedInjecter<B, BoxedAppFnOnce<'a, A, C>>
    for BoxedAppFn2<'a, A, B, C>
{
    fn provide(self, a: B) -> BoxedAppFnOnce<'a, A, C> {
        let r = p1(self, a);
        r
    }
}


pub fn futf1<A, B>(
    f: &'static (dyn Fn(A) -> BoxFuture<'static, Result<B, FnError>> + Send + Sync),
) -> BoxedFutAppFnOnce<'static, A, B> {
    Box::new(f) as _
}



pub trait AndThen<'a, A, B, C, F, R> {
    fn then(self, f: F) -> R;
}

pub trait AndThenAsync<'a, A, B, C, F, R> {
    fn then_async(self, f: F) -> R;
}



pub trait AsyncIntoBoxedFuture<'a, A, B> {
    fn async_into1<C: 'a>(self, f: BoxedFutAppFnOnce<'a, B, C>) -> BoxedFutAppFnOnce<'a, A, C>;
}

pub trait Lift<'a, I, O, A, B> {
    fn lift(self, i: I) -> O;
    //fn liftAsync(self,i: I)->R;
}

#[macro_use]
pub mod macros {
    use futures::future::BoxFuture;
    use crate::{BoxedAsyncFn1, FnError, lift_async_fn1};
  #[macro_export]
  macro_rules! c1 {
      ($fLeft:ident, $current_f:ident, true, true) =>{
            $fLeft.then_async(current_f)
        };
        
        ($fLeft:ident, $current_f:ident, true, false) =>{
            $fLeft.then_sync(current_f)
        };
        
        ($fLeft:ident, $current_f:ident, false, false) =>{
            $fLeft.then_sync(current_f)
        };
        
        ($fLeft:ident, $current_f:ident, false, true) =>{
            $fLeft.then_async(current_f)
        };
  }

    macro_rules! andthen{
        () => {

        }
    }

    #[macro_export]
    macro_rules! compose {



        ($fnLeft:ident,$isLeftFnAsync:ident,-> withArgs($args:expr) $($others:tt)*) => {
            {
            let r = $fnLeft($args);
            r
            }
        };

        ($fnLeft:ident,$isLeftFnAsync:ident,.provide($p1:expr) $($others:tt)*) => {
            {
                use fnutils::OwnedInjecter;
            let p = $fnLeft.provide($p1);
            let p1 = compose!(p,$isLeftFnAsync,$($others)*);
            p1
            }

            //let p = provide!(f,$p1);
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
                let f4;



                        let fLeft = $fLeft.then($fRight);
                        let isLeftFnAsync = $isRightAsync || $isLeftFnAsync;
                        let f3 = compose!(fLeft,isLeftFnAsync, -> $($others)*);
                        f4 = f3;

                f4
            }
        };

        ($fLeft:ident,$isLeftFnAsync:ident,-> $fn:ident.provide($p:expr) $($others:tt)*) =>{
            {
                let f4;
                concat_idents::concat_idents!(lifted_fn_name = lifted_fn,_, $fn {
                    let current_f = lifted_fn_name($fn);
                    concat_idents::concat_idents!(asynCheckFn = async_, $fn {
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
            concat_idents::concat_idents!(lifted_fn_name = lifted_fn,_, $fn {
                let current_f = lifted_fn_name($fn);
                concat_idents::concat_idents!(asynCheckFn = async_, $fn {
                    let currentAsync = asynCheckFn();
                




                    let _isResultAsync = currentAsync || $isLeftFnAsync;
                    let f3 = $fLeft.then(current_f);
                    let f3 = compose!(f3,_isResultAsync,$($others)*);
                    f4 = f3;


                });
            });
            f4
            }
        };



        ($fn:ident $($others:tt)*) => {
            {
                use paste::paste;
                use fnutils::AndThen;
                let f2;
                paste!{

                    let f = [<lifted_fn_ $fn>]($fn);


                    let isAsync = [<async_ $fn>]();
                    let f1 = compose!(f,isAsync,$($others)*);

                    f2 = f1;
                    //});

                };
                f2
            }
        };
        
       
    }
}


