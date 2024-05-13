use std::fs::read_to_string;
//use std::io::read_to_string;
use fnmacros::composeable;
use futures::{future::BoxFuture, FutureExt};

#[composeable]
fn add(a:i32)->Result<i32, FnError>{
    return Ok(a + 0);
}


#[composeable()]
pub fn add_async(a: i32,b: i32) -> BoxFuture<'static, Result<i32, FnError>>{
    async move{
        let  r =   a + b;
        Ok(r)
    }.boxed()
}


#[test]
fn composeable_sync_fn_test() {
    let lifted_add = lifted_fn_add(add);

    let is_add_async = async_add();
    assert_eq!(is_add_async, false);
}

#[test]
fn composeable_async_fn_test() {
    let lifted_add = lifted_fn_add_async(add_async);

    let is_add_async = async_add_async();
    assert_eq!(is_add_async, true);
}