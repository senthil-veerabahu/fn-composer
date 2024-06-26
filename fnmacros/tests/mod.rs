use std::fs::read_to_string;
use std::future;
use std::future::Future;
use std::ops::{Deref, DerefMut};
//use std::sync::Mutex;
use std::time::Duration;
use tokio::sync::Mutex;

use futures::{future::BoxFuture, FutureExt};
use quote::spanned::Spanned;

use fnmacros::composeable;

use fnutils::FnError;
use retry::delay::*;

pub fn retry_add_async99<'l>(
    mut a: &'l mut i32,
    mut b: &'l mut i32,
) -> BoxFuture<'l, Result<i32, FnError>> {
    use fnutils::*;
    use retry::*;
    use tokio::sync::Mutex;
    use tokio_retry::Retry as AsyncRetry;
    async {
        let mut a = Mutex::new(a);
        let mut b = Mutex::new(b);
        let result = AsyncRetry::spawn(Fixed::from_millis(100).take(2), || async {
            let mut a = a.lock().await;
            let mut b = b.lock().await;
            let r = add_async(a.deref_mut(), b.deref_mut());
            r.await
        });
        let result = match result.await {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        };
        result
    }
    .boxed()
}

/*pub fn lifted_fn_add < 'a, T1, T2, T3, F : Fn(T1, T2,) -> Result < T3, FnError
> + Send + Sync + 'a > (f : F) -> BoxedFn2 < 'a, T1, T2, T3, >
{ lift_sync_fn2(f) } pub fn async_add() -> bool { false }  use fnutils :: * ; use retry :: * ;
pub fn retry_add(mut a : & i32, mut b : & mut i32,) -> Result < i32, FnError >
{
    let result =
        retry(Fixed :: from_millis(100).take(2), ||
            { let r : Result < i32, FnError > = add(& a, & mut b,).into() ; r }) ;
    match result { Ok(result) => Ok(result), Err(e) => Err(e.error) }
}*/

#[composeable(retry = Fixed::from_millis(100).take(2))]
fn add_retryable(a: &i32, b: &mut i32) -> Result<i32, FnError> {
    Ok(*a + *b)
    //Ok(*a + *b)
}

#[composeable()]
fn add(a: &i32, b: &mut i32) -> Result<i32, FnError> {
    Ok(*a + *b)
    //Ok(*a + *b)
}

pub fn retry_add_async5<'a>(
    mut a: &'a mut i32,
    mut b: &'a mut i32,
) -> BoxFuture<'a, Result<i32, FnError>>
//impl Future<Output = Result<i32, FnError>> + Send
{
    async {
        use fnutils::*;
        use retry::*;
        use tokio_retry::Retry as AsyncRetry;
        let mut a = Mutex::new(a);
        let mut b = Mutex::new(b);
        /*
         * {a:mutext, b:mutex}
         */

        let result = AsyncRetry::spawn(Fixed::from_millis(100).take(2), || async {
            /*
             * {a:mutextguard, b:mutexguard}
             */
            //                    let a= a.lock().await;
            let mut a = a.lock().await;
            let mut b = b.lock().await;
            let r = add_async(a.deref_mut(), b.deref_mut());
            r.await
        });
        //async{
        let r = match result.await {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        };

        r
    }
    .boxed()
    //}.boxed()
}

pub fn retry_add_async11<'l>(
    mut a: &'l mut i32,
    mut b: &'l mut i32,
) -> BoxFuture<'l, Result<i32, FnError>> {
    use fnutils::*;
    use retry::*;
    use tokio::sync::Mutex;
    use tokio_retry::Retry as AsyncRetry;
    async {
        let mut a = Mutex::new(a);
        let mut b = Mutex::new(b);
        let result = AsyncRetry::spawn(Fixed::from_millis(100).take(2), || async {
            let mut a = a.lock().await;
            let mut b = b.lock().await;
            let r = add_async(a.deref_mut(), b.deref_mut());
            r.await
        });
        let result = match result.await {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        };
        result
    }
    .boxed()
}

#[composeable(retry = Fixed::from_millis(100).take(2))]
pub fn add_async_retryable<'l>(
    a: &'l mut i32,
    b: &'l mut i32,
) -> BoxFuture<'l, Result<i32, FnError>> {
    async move {
        let r = *a + *b;
        Ok(r)
    }.boxed()
}

#[composeable()]
pub fn add_async<'l>(a: &'l mut i32, b: &'l mut i32) -> BoxFuture<'l, Result<i32, FnError>> {
    async move {
        /*let retry_strategy = Fixed::from_millis(100).take(2);

        let result = tokio_retry::Retry::spawn(retry_strategy, || async{1}).await?;*/

        let r = *a + *b;
        Ok(r)
    }
    .boxed()
}

#[test]
fn composeable_sync_fn_retryable_test() {
    let lifted_add = fn_composer__lifted_fn_add_retryable(add_retryable);

    let is_add_async = fn_composer__is_async_add_retryable();
    let is_retryable = fn_composer__is_retryable_add_retryable();
    let retry_fn = fn_composer__retry_add_retryable;
    assert_eq!(is_add_async, false);
    assert_eq!(is_retryable, true);
    lifted_add(&10, &mut 11);
}

#[test]
fn composeable_sync_fn_test() {
    let lifted_add = fn_composer__lifted_fn_add_retryable(add);

    let is_add_async = fn_composer__is_async_add();
    let is_retryable = fn_composer__is_retryable_add();
    let retry_fn = fn_composer__retry_add_retryable;
    assert_eq!(is_add_async, false);
    assert_eq!(is_retryable, false);
    lifted_add(&10, &mut 11);
}

#[test]
fn composeable_async_fn_test() {
    let lifted_add = fn_composer__lifted_fn_add_async(add_async);

    let is_add_async = fn_composer__is_async_add_async();
    let is_retryable = fn_composer__is_retryable_add_async();
    let retry_fn = fn_composer__retry_add_async;
    assert_eq!(is_retryable, false);
    assert_eq!(is_add_async, true);
}

#[test]
fn composeable_async_fn_retryable_test() {
    let lifted_add = fn_composer__lifted_fn_add_async_retryable(add_async_retryable);

    let is_add_async = fn_composer__is_async_add_async_retryable();
    let is_retryable = fn_composer__is_retryable_add_async_retryable();
    
    let retry_fn = fn_composer__retry_add_async_retryable;
    assert_eq!(is_retryable, true);
    assert_eq!(is_add_async, true);
}

#[composeable(retry = Fixed::from_millis(100))]
pub fn add_3_arg_ref_async<'a>(
    a: &'a mut i32,
    b: &'a mut i32,
    c: &'a i32,
) -> BoxFuture<'a, Result<i32, FnError>> {
    async move {
        let r = *a + *b + c;
        Ok(r)
    }
    .boxed()
}

#[composeable(retry = Fixed::from_millis(100))]
pub fn add_3_arg_ref__non_copy_async<'a>(
    a: &'a mut Vec<String>,
    b: &'a mut Vec<String>,
    c: &'a Vec<String>,
) -> BoxFuture<'a, Result<i32, FnError>> {
    async move {
        let r = a.len() + b.len() + c.len();
        Ok(r as i32)
    }
        .boxed()
}

/*pub fn _retry_add_3_arg_ref_async1099<'a>(
    mut a: &'a mut i32,
    mut b: &'a i32,
    mut c: &'a i32,
) -> BoxFuture<'a, Result<i32, FnError>> {
    use fnutils::*;
    use retry::*;
    use tokio_retry::Retry as AsyncRetry;
    use tokio::sync::Mutex;
    async {
        let mut a = Mutex::new(a);
        let result = AsyncRetry::spawn(
            Fixed::from_millis(100),
            || async {
                let mut a = a.lock().await;
                let r = add_3_arg_ref_async(a.deref_mut(), b, c);
                r.await
            },
        );
        let result = match result.await {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        };
        result
    }
        .boxed()
}
*/

pub fn _retry_add_async_retryable00<'l>(
    mut a: &'l mut i32,
    mut b: &'l mut i32,
) -> BoxFuture<'l, Result<i32, FnError>> {
    use fnutils::*;
    use retry::*;
    use tokio_retry::Retry   as AsyncRetry;
    use tokio::sync::Mutex;
    async {
        let mut a = Mutex::new(a);
        let mut b = Mutex::new(b);
        let result = AsyncRetry::spawn(Fixed::from_millis(100).take(2), || async {
            let mut a = a.lock().await;
            let mut b = b.lock().await;
            ;
            let r = add_async_retryable(a.deref_mut(), b.deref_mut());
            r.await
        });
        let result = match result.await {
            Ok(result) => Ok(result),
            Err(e) => Err(e)
        };
        result
    }.boxed()
}



/*pub fn _retry_add_3_arg_ref_async9090<'a>(
    mut a: &'a i32,
    mut b: &'a i32,
    mut c: &'a i32,
) -> BoxFuture<'a, Result<i32, FnError>> {
    use fnutils::*;
    use retry::*;
    use tokio_retry::Retry as AsyncRetry;
    use tokio::sync::Mutex;
    async {
        let result = AsyncRetry::spawn(
            Fixed::from_millis(100),
            || async {
                let r = add_3_arg_ref_async(a, b, c);
                r.await
            },
        );
        let result = match result.await {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        };
        result
    }
        .boxed()
}
*/

/*
def test_answer():
    assert inc(-3) == -2
    assert inc(-3) == -2
    assert inc(3) == 5
 */
