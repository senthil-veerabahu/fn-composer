use futures::{future::BoxFuture, FutureExt};
use retry::delay::*;


use function_compose::composeable;
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

#[composeable(retry = Fixed::from_millis(100).take(2))]
fn add_retryable(a: &i32, b: &mut i32) -> Result<i32, FnError<String>> {
    Ok(*a + *b)
}

#[composeable()]
fn add(a: &i32, b: &mut i32) -> Result<i32, FnError<String>> {
    Ok(*a + *b)
    //Ok(*a + *b)
}

#[composeable(retry = Fixed::from_millis(100).take(2))]
pub fn add_async_retryable<'l>(
    a: &'l mut i32,
    b: &'l mut i32,
) -> BoxFuture<'l, Result<i32, FnError<String>>> {
    async move {
        let r = *a + *b;
        Ok(r)
    }
    .boxed()
}

#[composeable()]
pub fn add_async<'l>(a: &'l mut i32, b: &'l mut i32) -> BoxFuture<'l, Result<i32, FnError<String>>> {
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
    let _retry_fn = fn_composer__retry_add_retryable;
    assert_eq!(is_add_async, false);
    assert_eq!(is_retryable, true);
    let _  = lifted_add(&10, &mut 11);
}

#[test]
fn composeable_sync_fn_test() {
    let lifted_add = fn_composer__lifted_fn_add_retryable(add);

    let is_add_async = fn_composer__is_async_add();
    let is_retryable = fn_composer__is_retryable_add();
    let _retry_fn = fn_composer__retry_add_retryable;
    assert_eq!(is_add_async, false);
    assert_eq!(is_retryable, false);
    let _ = lifted_add(&10, &mut 11);
}

#[test]
fn composeable_async_fn_test() {
    let _lifted_add = fn_composer__lifted_fn_add_async(add_async);

    let is_add_async = fn_composer__is_async_add_async();
    let is_retryable = fn_composer__is_retryable_add_async();
    let _retry_fn = fn_composer__retry_add_async;
    assert_eq!(is_retryable, false);
    assert_eq!(is_add_async, true);
}

#[test]
fn composeable_async_fn_retryable_test() {
    let _lifted_add = fn_composer__lifted_fn_add_async_retryable(add_async_retryable);

    let is_add_async = fn_composer__is_async_add_async_retryable();
    let is_retryable = fn_composer__is_retryable_add_async_retryable();

    let _retry_fn = fn_composer__retry_add_async_retryable;
    assert_eq!(is_retryable, true);
    assert_eq!(is_add_async, true);
}

#[composeable(retry = Fixed::from_millis(100))]
pub fn add_3_arg_ref_async<'a>(
    a: &'a mut i32,
    b: &'a mut i32,
    c: &'a i32,
) -> BoxFuture<'a, Result<i32, FnError<String>>> {
    async move {
        let r = *a + *b + c;
        Ok(r)
    }
    .boxed()
}

#[composeable(retry = Fixed::from_millis(100))]
pub fn add_3_arg_ref_non_copy_async<'a>(
    a: &'a mut Vec<String>,
    b: &'a mut Vec<String>,
    c: &'a Vec<String>,
) -> BoxFuture<'a, Result<i32, FnError<String>>> {
    async move {
        let r = a.len() + b.len() + c.len();
        Ok(r as i32)
    }
    .boxed()
}
