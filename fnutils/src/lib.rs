use std::{error::Error, fmt::Display, mem::discriminant, ops::Deref, thread};

use futures::{future::BoxFuture, FutureExt};

//test
pub type AppFn<'a, A, B> = (dyn Fn(A) -> Result<B, FnError> + Send);

pub type AppFn2<'a, A, B, C> = &'a dyn Fn(A, B) -> Result<C, FnError>;
pub type AppFn3<'a, A, B, C, D> = &'a dyn Fn(A, B, C) -> Result<D, FnError>;

pub type FutureAppFn<'a, A, B> = Box<dyn Fn(A) -> BoxFuture<'a, Result<B, FnError>> + Send + Sync>;
pub type FutureAppFnOnce<'a, A, B> = Box<dyn FnOnce(A) -> BoxFuture<'a, Result<B, FnError>>>;
pub type FutureAppFn2<'a, A, B, C> = Box<dyn Fn(A, B) -> BoxFuture<'a, Result<C, FnError>>>;

pub type BoxedAppFn<'a, A, B> = Box<dyn Fn(A) -> Result<B, FnError> + Send + Sync + 'a>;
pub type BoxedFutAppFnOnce<'a, A, B> =
    Box<dyn FnOnce(A) -> BoxFuture<'a, Result<B, FnError>> + Send + Sync + 'a>;
pub type BoxedFutAppTwoArgFnOnce<'a, A, B, C> =
    Box<dyn Fn(A, B) -> BoxFuture<'a, Result<C, FnError>> + Send + Sync + 'a>;

pub type BoxedAppFnOnce<'a, A, B> = Box<dyn FnOnce(A) -> Result<B, FnError> + 'a>;

pub type Boxed1AppFnOnce<'a, A, B, C> = Box<dyn FnOnce(A, B) -> Result<C, FnError> + 'a>;

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

pub fn composeWithFut<'a, A: Send + 'a, B: Send + 'a, C: 'a>(
    f: BoxedAppFn<'a, B, C>,
    g: BoxedFutAppFnOnce<'a, A, B>,
) -> BoxedFutAppFnOnce<'a, A, C> {
    let r1 = |x: A| {
        async move {
            let b = g(x).await?;

            f(b)
        }
        .boxed()
    };

    Box::new(r1) as _
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

pub fn composeWithFut2<'a, A: Send + 'a, B: Send + 'a, C: 'a>(
    f: BoxedFutAppFnOnce<'a, B, C>,
    g: BoxedAppFn<'a, A, B>,
) -> BoxedFutAppFnOnce<'a, A, C> {
    let r1 = |a: A| {
        async move {
            let gResult = (g)(a);

            match gResult {
                Ok(b) => {
                    let f = f(b).await;
                    let fResult = f?;
                    //sendFn(fResult);
                    Ok(fResult)
                    //Ok(Default::default())
                }
                Err(e) => Err(e),
            }
        }
        .boxed()
    };
    Box::new(r1)
}

pub fn composeWithFut3<'a, A: Send + 'a, B: Send + 'a, C: 'a>(
    f: BoxedFutAppFnOnce<'a, B, C>,
    g: BoxedFutAppFnOnce<'a, A, B>,
) -> BoxedFutAppFnOnce<'a, A, C> {
    let r1 = |a: A| {
        async move {
            let gResult = g(a).await?;
            f(gResult).await
        }
        .boxed()
    };

    let r: BoxedFutAppFnOnce<'a, A, C> = Box::new(r1);
    r
}

pub fn p1<'a, A: 'a, B: 'a, C: 'a>(
    fn1: BoxedAppFn2<'a, A, B, C>,
    b: B,
) -> BoxedAppFnOnce<'a, A, C> {
    Box::new(move |a: A| fn1(a, b))
}

pub fn futp1<'a, A: Send + 'a, B: Send + Sync + 'a, C: Send + 'a>(
    fn1: BoxedFutAppTwoArgFnOnce<'a, A, B, C>,
    b: B,
) -> BoxedFutAppFnOnce<'a, A, C> {
    let x = move |a: A| fn1(a, b);
    Box::new(x)
}

fn run_fn1<F: Fn() + Send + 'static>(f: F) {
    let _hand = thread::spawn(f);
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

impl<'a, A: Send + Sync + 'a, B: Send + Sync + 'a, C: Send + 'a>
    OwnedInjecter<B, BoxedFutAppFnOnce<'a, A, C>> for BoxedFutAppTwoArgFnOnce<'a, A, B, C>
{
    fn provide(self, a: B) -> BoxedFutAppFnOnce<'a, A, C> {
        let r = futp1(self, a);
        r
    }
}

pub fn futf1<A, B>(
    f: &'static (dyn Fn(A) -> BoxFuture<'static, Result<B, FnError>> + Send + Sync),
) -> BoxedFutAppFnOnce<'static, A, B> {
    Box::new(f) as _
}

pub trait AndThen<'a, A, B> {
    fn then<C: 'a>(self, f: BoxedAppFn<'a, B, C>) -> BoxedAppFn<'a, A, C>;
}

pub trait AndThen1<'a, A, B, C, F, R> {
    fn then1(self, f: F) -> R;
}

impl<'a, A: 'a, B: 'a, C: 'a> AndThen1<'a, A, B, C, BoxedAppFn<'a, B, C>, BoxedAppFn<'a, A, C>>
    for BoxedAppFn<'a, A, B>
{
    fn then1(self, f: BoxedAppFn<'a, B, C>) -> BoxedAppFn<'a, A, C> {
        compose_fn(f, self)
    }
}

impl<'a, A: 'a + Send, B: 'a + Send, C: 'a>
    AndThen1<'a, A, B, C, BoxedFutAppFnOnce<'a, B, C>, BoxedFutAppFnOnce<'a, A, C>>
    for BoxedFutAppFnOnce<'a, A, B>
{
    fn then1(self, f: BoxedFutAppFnOnce<'a, B, C>) -> BoxedFutAppFnOnce<'a, A, C> {
        composeWithFut3(f, self)
    }
}

impl<'a, A: 'a + Send, B: 'a + Send, C: 'a>
    AndThen1<'a, A, B, C, BoxedAppFn<'a, B, C>, BoxedFutAppFnOnce<'a, A, C>>
    for BoxedFutAppFnOnce<'a, A, B>
{
    fn then1(self, f: BoxedAppFn<'a, B, C>) -> BoxedFutAppFnOnce<'a, A, C> {
        composeWithFut(f, self)
    }
}

impl<'a, A: 'a + Send, B: 'a + Send, C: 'a>
    AndThen1<'a, A, B, C, BoxedFutAppFnOnce<'a, B, C>, BoxedFutAppFnOnce<'a, A, C>>
    for BoxedAppFn<'a, A, B>
{
    fn then1(self, f: BoxedFutAppFnOnce<'a, B, C>) -> BoxedFutAppFnOnce<'a, A, C> {
        composeWithFut2(f, self)
    }
}

impl<'a, A: 'a, B: 'a> AndThen<'a, A, B> for BoxedAppFn<'a, A, B> {
    fn then<C: 'a>(self, f: BoxedAppFn<'a, B, C>) -> BoxedAppFn<'a, A, C> {
        compose_fn(f, self)
    }
}

pub trait AsyncInto<'a, A, B> {
    fn async_into<C: 'a>(self, f: BoxedFutAppFnOnce<'a, B, C>) -> BoxedFutAppFnOnce<'a, A, C>;
}

pub trait AsyncFrom<'a, A, B> {
    fn async_out<C: 'a>(self, f: BoxedAppFn<'a, B, C>) -> BoxedFutAppFnOnce<'a, A, C>;
}

impl<'a, A: 'a + Send, B: 'a + Send> AsyncFrom<'a, A, B> for BoxedFutAppFnOnce<'a, A, B> {
    fn async_out<C: 'a>(self, g: BoxedAppFn<'a, B, C>) -> BoxedFutAppFnOnce<'a, A, C> {
        //let a = self;
        let r = composeWithFut(g, self);
        r
    }
}

impl<'a, A: 'a + Send, B: 'a + Send> AsyncInto<'a, A, B> for BoxedAppFn<'a, A, B> {
    fn async_into<C: 'a>(self, g: BoxedFutAppFnOnce<'a, B, C>) -> BoxedFutAppFnOnce<'a, A, C> {
        //let a = self;
        let r = composeWithFut2(g, self);
        r
    }
}

impl<'a, A: 'a + Send, B: 'a + Send> AsyncInto<'a, A, B> for BoxedFutAppFnOnce<'a, A, B> {
    fn async_into<C: 'a>(self, g: BoxedFutAppFnOnce<'a, B, C>) -> BoxedFutAppFnOnce<'a, A, C> {
        let r = composeWithFut3(g, self);
        r
    }
}
pub trait AsyncIntoBoxedFuture<'a, A, B> {
    fn async_into1<C: 'a>(self, f: BoxedFutAppFnOnce<'a, B, C>) -> BoxedFutAppFnOnce<'a, A, C>;
}

fn futf2<'a, A, B, C, F: Fn(A, B) -> BoxFuture<'static, Result<C, FnError>> + 'static>(
    f: F,
) -> FutureAppFn2<'static, A, B, C> {
    let f1: FutureAppFn2<'static, A, B, C> = Box::new(f);
    f1
}

pub trait Lift<'a, I, O, A, B> {
    fn lift(self, i: I) -> O;
    //fn liftAsync(self,i: I)->R;
}

struct Lifter;

pub fn liftTwoArgAsync<
    'a,
    A,
    B,
    C,
    F: Fn(A, B) -> BoxFuture<'a, Result<C, FnError>> + 'a + Send + Sync,
>(
    f: F,
) -> BoxedFutAppTwoArgFnOnce<'a, A, B, C> {
    Box::new(f)
}

pub fn liftAsync<'a, A, B, F: Fn(A) -> BoxFuture<'a, Result<B, FnError>> + 'a + Send + Sync>(
    f: F,
) -> BoxedFutAppFnOnce<'a, A, B> {
    Box::new(f)
}

pub fn lift<'a, A, B, F: Fn(A) -> Result<B, FnError> + Send + Sync + 'a>(
    f: F,
) -> BoxedAppFn<'a, A, B> {
    Box::new(f)
}

#[macro_use]
pub mod macros {

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
            let p = $fnLeft.provide($p1);
            let p1 = compose!(p,$isLeftFnAsync,$($others)*);
            p1
            }

            //let p = provide!(f,$p1);
        };

        ($fLeft:ident,$isLeftFnAsync:ident,-> $fn:ident $($others:tt)*) =>{
            {
            let f4;
            concat_idents::concat_idents!(lifted_fn_name = lifted_fn,_, $fn {
                let current_f = lifted_fn_name($fn);
                concat_idents::concat_idents!(asynCheckFn = async_, $fn {
                    let currentAsync = asynCheckFn();

                    let f3 = {

                        $fLeft.then1(current_f)
                    };

                    let _isResultAsync = currentAsync || $isLeftFnAsync;

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
                use fnutils::AndThen1;
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
