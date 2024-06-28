#![cfg(test)]

use function_compose::*;
use futures::{future::BoxFuture, FutureExt};
use retry::delay::*;

static mut RETRY_COUNT: i32 = 0;

trait TestTrait :Send + Sync{
    fn do_work(&self) -> i32;
}

impl TestTrait for i32{
    fn do_work(&self) -> i32{
        0
    }
}

//#[composeable()]
fn do_work(a:i32, test:impl TestTrait)->Result<i32, FnError>{
    Ok(0)
}

#[composeable()]
fn do_work_with_box(a:i32, test:Box<dyn TestTrait> )->Result<i32, FnError>{
    Ok(0)
}

#[composeable(retry = Fixed::from_millis(100).take(2))]
pub fn add_10(a: i32) -> Result<i32, FnError> {
    Ok(a + 10)
}


#[composeable()]
pub fn add_100(a: i32) -> Result<i32, FnError> {
    Ok(a + 100)
}

#[composeable()]
pub fn add_async(a: i32, b: i32) -> BoxFuture<'static, Result<i32, FnError>> {
    async move {
        let r = a + b;
        Ok(r)
    }
    .boxed()
}

#[composeable()]
pub fn add_3_arg_async(a: i32, b: i32, c: i32) -> BoxFuture<'static, Result<i32, FnError>> {
    async move {
        let r = a + b + c;
        Ok(r)
    }
    .boxed()
}

#[composeable(retry = Fixed::from_millis(100).take(2))]
pub fn add_3_arg_ref_async<'a>(
    a: &'a i32,
    b: &'a i32,
    c: &'a i32,
) -> BoxFuture<'a, Result<i32, FnError>> {
    
        async move {
            if (retry_count_lessthan(2)) {
                increment_retry_count();                
                Err(FnError {
                    description: Some("Retry test".to_owned()),
                    error_code: None,
                    underlying_error: None,
                })
            } else {
                let r = a + b + c;
                 reset_retry_count();
                Ok(r)
            }
        }.boxed()
}

#[composeable(retry = Fixed::from_millis(100).take(2))]
pub fn add_3_arg_ref<'a>(a: &'a i32, b: &'a i32, c: &'a i32) -> Result<i32, FnError> {

    if (retry_count_lessthan(2)) {
        increment_retry_count();
        return Err(FnError {
            description: Some("Retry test".to_owned()),
            error_code: None,
            underlying_error: None,
        });
    }
    let r = a + b + c;
    Ok(r)
}

#[composeable(retry = Fixed::from_millis(100).take(2))]
pub fn add_vec_size_ref__non_copy_sync<'a>(
    a: &'a mut Vec<String>,
    b: &'a mut Vec<String>,
    c: &'a Vec<String>,
) -> Result<i32, FnError> {
    if (retry_count_lessthan(2)) {
        increment_retry_count();
        return Err(FnError {
            description: Some("Retry test".to_owned()),
            error_code: None,
            underlying_error: None,
        });
    }
    let r = a.len() + b.len() + c.len();
    Ok(r as i32)
}

#[composeable(retry = Fixed::from_millis(100).take(2))]
pub fn add_vec_size_ref__non_copy_async<'a>(
    a: &'a mut Vec<String>,
    b: &'a mut Vec<String>,
    c: &'a Vec<String>,
) -> BoxFuture<'a, Result<i32, FnError>> {
    async move {
        if (retry_count_lessthan(2)) {
            increment_retry_count();
            return Err(FnError {
                description: Some("Retry test".to_owned()),
                error_code: None,
                underlying_error: None,
            });
        }
        let r = a.len() + b.len() + c.len();
        Ok(r as i32)
    }
    .boxed()
}

#[composeable()]
pub fn multiply_async(a: i32, b: i32) -> BoxFuture<'static, Result<i32, FnError>> {
    async move {
        let r = a * b;
        Ok(r)
    }
    .boxed()
}

#[composeable]
pub fn add_100_async(a: i32) -> BoxFuture<'static, Result<i32, FnError>> {
    async move {
        let r = a + 100;
        Ok(r)
    }
    .boxed()
}

#[test]
fn test_compose_sync_functions() {
    let result = compose!(add_10 -> add_100 -> with_args(10));
    assert_eq!(120, result.unwrap());
    let result = compose!(add_10 -> add_100 -> add_10 -> add_100 -> with_args(10));
    assert_eq!(230, result.unwrap());
}

#[test]
fn test_box_dyn_trait() {
    let trait_impl = Box::new(20) as Box<dyn TestTrait>;
    let result:Result<i32, FnError> = compose!(do_work_with_box.provide(trait_impl) -> add_100 -> with_args(10));
    assert_eq!(100, result.unwrap());    
}

#[tokio::test]
async fn test_compose_async_functions() {
    let result = compose!(add_100_async -> add_100 -> with_args(10)).await;
    assert_eq!(210, result.unwrap());

    //Test composing Async function with async function
    let result = compose!(add_100_async -> add_100_async -> with_args(10)).await;
    assert_eq!(210, result.unwrap());

    //Test composing Two arg Async function with async with single arg async function
    let result = compose!(add_async.provide(100) -> add_100_async -> with_args(10)).await;
    assert_eq!(210, result.unwrap());

    let result =
        compose!(add_3_arg_async.provide(100).provide(200) -> add_100_async -> with_args(10)).await;
    assert_eq!(410, result.unwrap());

    //Test composing single arg sync function with  two arg async function
    let result: Result<i32, FnError> =
        compose!(add_100 -> add_3_arg_async.provide(1).provide(1) -> with_args(10)).await;
    assert_eq!(112, result.unwrap());

    //Test injecting multiple values to async functions
    let result = compose!(add_3_arg_async.provide(1).provide(1) -> add_3_arg_async.provide(1).provide(1) -> with_args(10)).await;
    assert_eq!(14, result.unwrap());
    reset_retry_count();

    reset_retry_count();
    //Test injecting shared reference to async functions
    let one = &1;
    let result = compose!(add_3_arg_ref_async.provide(one).provide(one) -> add_3_arg_async.provide(1).provide(1) -> with_args(&10)).await;
    assert_eq!(14, result.unwrap());
    reset_retry_count();
}

#[tokio::test]
async fn test_compose_async_retry_test() {
    let one = &1;
    let result = compose!(add_3_arg_ref_async.provide(one).provide(one) -> add_100_async -> with_args(&10)).await;
    assert_eq!(112, result.unwrap());
    update_retry_count_for_failure();
    let result = compose!(add_3_arg_ref_async.provide(one).provide(one) -> add_100_async -> with_args(&10)).await;
    assert_eq!(true, result.is_err());
    reset_retry_count();


  
    let v1 = Box::leak(Box::new(vec!["1".to_owned()]));
    let mut v2 = Box::leak(Box::new(vec!["1".to_owned()]));
    let mut v3 = Box::leak(Box::new(vec!["1".to_owned()]));
    let result = compose!(add_vec_size_ref__non_copy_async.provide(v1).provide( v2) -> add_100_async -> with_args(v3)).await;
    assert_eq!(103, result.unwrap());

    update_retry_count_for_failure();
    let v1 = Box::leak(Box::new(vec!["1".to_owned()]));
    let mut v2 = Box::leak(Box::new(vec!["1".to_owned()]));
    let mut v3 = Box::leak(Box::new(vec!["1".to_owned()]));
    let result = compose!(add_vec_size_ref__non_copy_async.provide(v1).provide( v2) -> add_100_async -> with_args(v3)).await;
    assert_eq!(true, result.is_err());
    reset_retry_count();
}

#[tokio::test]
async fn test_compose_sync_retry_test() {
    let one = &1;
    let result = compose!(add_3_arg_ref.provide(one).provide(one) -> add_100 -> with_args(&10));
    assert_eq!(112, result.unwrap());
    update_retry_count_for_failure();
    let result = compose!(add_3_arg_ref.provide(one).provide(one) -> add_100 -> with_args(&10));
    assert_eq!(true, result.is_err());
    reset_retry_count();



    let v1 = Box::leak(Box::new(vec!["1".to_owned()]));
    let mut v2 = Box::leak(Box::new(vec!["1".to_owned()]));
    let mut v3 = Box::leak(Box::new(vec!["1".to_owned()]));
    let result = compose!(add_vec_size_ref__non_copy_sync.provide(v1).provide( v2) -> add_100 -> with_args(v3));
    assert_eq!(103, result.unwrap());

    update_retry_count_for_failure();
    let v1 = Box::leak(Box::new(vec!["1".to_owned()]));
    let mut v2 = Box::leak(Box::new(vec!["1".to_owned()]));
    let mut v3 = Box::leak(Box::new(vec!["1".to_owned()]));
    let result = compose!(add_vec_size_ref__non_copy_sync.provide(v1).provide( v2) -> add_100 -> with_args  (v3));
    assert_eq!(true, result.is_err());
    reset_retry_count();
}

fn reset_retry_count(){
    unsafe{
        RETRY_COUNT = 0;
    }
}

fn update_retry_count_for_failure(){
    unsafe{
        RETRY_COUNT = -10;
    }
}

fn increment_retry_count(){
    unsafe{
        RETRY_COUNT = RETRY_COUNT + 1;
    }
}

fn retry_count_lessthan(count:i32) -> bool{
    unsafe{
        RETRY_COUNT < count
    }
}