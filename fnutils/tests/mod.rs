#![cfg(test)]

use fnmacros::composeable;
use futures::{future::BoxFuture, FutureExt};
use fnutils::{BoxedFn1, compose, FnError};


#[composeable()]
pub fn add_10(a:i32) -> Result<i32, FnError>{
    Ok(a + 10)
}

#[composeable()]
pub fn add_100(a:i32) -> Result<i32, FnError>{
    Ok(a + 100)
}



#[composeable()]
pub fn add_async(a: i32,b: i32) -> BoxFuture<'static, Result<i32, FnError>>{
    async move{
        let  r =   a + b;
        Ok(r)
    }.boxed()
}

#[composeable()]
pub fn add_3_arg_async(a: i32,b: i32, c:i32) -> BoxFuture<'static, Result<i32, FnError>>{
    async move{
        let  r =   a + b + c;
        Ok(r)
    }.boxed()
}

#[composeable()]
pub fn add_3_arg_ref_async<'a>(a: i32,b: &'a i32, c:&'a i32) -> BoxFuture<'a, Result<i32, FnError>>{
    async move{
        let  r =   a + b + c;
        Ok(r)
    }.boxed()
}

#[composeable()]
pub fn multiply_async(a: i32,b: i32) -> BoxFuture<'static, Result<i32, FnError>>{
    async move{
        let  r =   a * b;
        Ok(r)
    }.boxed()
}


#[composeable]
pub fn add_100_async(a: i32) -> BoxFuture<'static, Result<i32, FnError>>{
    async move{
        let  r =   a + 100;
        Ok(r)
    }.boxed()
}

#[test]
fn test_compose_sync_functions(){
    let result = compose!(add_10 -> add_100 -> withArgs(10));
    assert_eq!(120, result.unwrap());
    let result = compose!(add_10 -> add_100 -> add_10 -> add_100 -> withArgs(10));
    assert_eq!(230, result.unwrap());


}

#[tokio::test]
async fn test_compose_async_functions(){
    use fnutils::OwnedInjecter;
    let result = compose!(add_100_async -> add_100 -> withArgs(10)).await;
    assert_eq!(210, result.unwrap());

    //Test composing Async function with async function
    let result = compose!(add_100_async -> add_100_async -> withArgs(10)).await;
    assert_eq!(210, result.unwrap());

    //Test composing Two arg Async function with async with single arg async function
    let result = compose!(add_async.provide(100) -> add_100_async -> withArgs(10)).await;
    assert_eq!(210, result.unwrap());

    let result = compose!(add_3_arg_async.provide(100).provide(200) -> add_100_async -> withArgs(10)).await;
    assert_eq!(410, result.unwrap());


    //Test composing single arg sync function with  two arg async function
    let result = compose!(add_100 -> add_3_arg_async.provide(1).provide(1) -> withArgs(10)).await;
    assert_eq!(112, result.unwrap());

    let result = compose!(add_3_arg_async.provide(1).provide(1) -> add_3_arg_async.provide(1).provide(1) -> withArgs(10)).await;
    assert_eq!(14, result.unwrap());
    
    let one = &1;
    let result = compose!(add_3_arg_ref_async.provide(one).provide(one) -> add_3_arg_async.provide(1).provide(1) -> withArgs(10)).await;
    assert_eq!(14, result.unwrap());
}