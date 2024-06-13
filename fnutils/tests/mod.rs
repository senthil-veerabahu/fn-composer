#![cfg(test)]

use fnmacros::composeable;
use futures::{future::BoxFuture, FutureExt};
use fnutils::{BoxedFn1, compose, FnError};

use fnutils::*;
pub fn lifted_fn_add_10<'a, T1, T2, F: Fn(T1) -> Result<T2, FnError> + Send + Sync + 'a>(
    f: F,
) -> BoxedFn1<'a, T1, T2> {
    lift_sync_fn1(f)
}
pub fn async_add_10() -> bool {
    false
}
pub fn add_10(a: i32) -> Result<i32, FnError> {
    Ok(a + 10)
}
use fnutils::*;
pub fn lifted_fn_add_100<
    'a,
    T1,
    T2,
    F: Fn(T1) -> Result<T2, FnError> + Send + Sync + 'a,
>(f: F) -> BoxedFn1<'a, T1, T2> {
    lift_sync_fn1(f)
}
pub fn async_add_100() -> bool {
    false
}
pub fn add_100(a: i32) -> Result<i32, FnError> {
    Ok(a + 100)
}
use fnutils::*;
pub fn lifted_fn_add_async<
    'a,
    T1,
    T2,
    T3,
    F: Fn(T1, T2) -> BoxFuture<'a, Result<T3, FnError>> + 'a + Send + Sync,
>(f: F) -> BoxedAsyncFn2<'a, T1, T2, T3> {
    lift_async_fn2(f)
}
pub fn async_add_async() -> bool {
    true
}
pub fn add_async(a: i32, b: i32) -> BoxFuture<'static, Result<i32, FnError>> {
    async move {
        let r = a + b;
        Ok(r)
    }
        .boxed()
}
use fnutils::*;
pub fn lifted_fn_multiply_async<
    'a,
    T1,
    T2,
    T3,
    F: Fn(T1, T2) -> BoxFuture<'a, Result<T3, FnError>> + 'a + Send + Sync,
>(f: F) -> BoxedAsyncFn2<'a, T1, T2, T3> {
    lift_async_fn2(f)
}
pub fn async_multiply_async() -> bool {
    true
}
pub fn multiply_async(a: i32, b: i32) -> BoxFuture<'static, Result<i32, FnError>> {
    async move {
        let r = a * b;
        Ok(r)
    }
        .boxed()
}
use fnutils::*;
pub fn lifted_fn_add_100_async<
    'a,
    T1,
    T2,
    F: Fn(T1) -> BoxFuture<'a, Result<T2, FnError>> + 'a + Send + Sync,
>(f: F) -> BoxedAsyncFn1<'a, T1, T2> {
    lift_async_fn1(f)
}
pub fn async_add_100_async() -> bool {
    true
}
pub fn add_100_async(a: i32) -> BoxFuture<'static, Result<i32, FnError>> {
    async move {
        let r = a + 100;
        Ok(r)
    }
        .boxed()
}

#[test]
fn test_compose_sync_functions(){
    let x = add_10(100);
    let result = compose!(add_10 -> add_100 -> withArgs(10));
    assert_eq!(120, result.unwrap());
    let result = compose!(add_10 -> add_100 -> add_10 -> add_100 -> withArgs(10));
    assert_eq!(230, result.unwrap())


}

#[tokio::test]
async fn test_compose_async_functions(){
    //Test composing Async function with sync function
    let x = add_100_async(100);
    let result = compose!(add_100_async -> add_100 -> withArgs(10)).await;
    assert_eq!(210, result.unwrap());

    //Test composing Async function with async function
    let result = compose!(add_100_async -> add_100_async -> withArgs(10)).await;
    assert_eq!(210, result.unwrap());

    //Test composing Two arg Async function with async with single arg async function
    let result = compose!(add_async.provide(100) -> add_100_async -> withArgs(10)).await;
    assert_eq!(210, result.unwrap());

    //Test composing single arg sync function with  two arg async function
    //TODO this needs to be fixed
    /*let result = compose!(add_100 -> multiply_async.provide(1) -> withArgs(10)).await;
    assert_eq!(110, result.unwrap());*/
}