use std::ops::DerefMut;

use futures::{future::BoxFuture, FutureExt};
use retry::delay::*;
use tokio::sync::Mutex;

use fn_macros::composeable;
use fn_compose::{FnError};

#[composeable(retry = Fixed::from_millis(100).take(2))]
fn add_retryable(a: &i32, b: &mut i32) -> Result<i32, FnError> {
    Ok(*a + *b)
}

#[composeable()]
fn add(a: &i32, b: &mut i32) -> Result<i32, FnError> {
    Ok(*a + *b)
    //Ok(*a + *b)
}

#[composeable(retry = Fixed::from_millis(100).take(2))]
pub fn add_async_retryable<'l>(
    a: &'l mut i32,
    b: &'l mut i32,
) -> BoxFuture<'l, Result<i32, FnError>> {
    async move {
        let r = *a + *b;
        Ok(r)
    }
    .boxed()
}

#[composeable()]
pub fn add_async<'l>(a: &'l mut i32, b: &'l mut i32) -> BoxFuture<'l, Result<i32, FnError>> {
    async move {
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
