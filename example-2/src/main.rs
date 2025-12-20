use function_compose::composeable;
use futures::{FutureExt, future::BoxFuture};
use retry::delay::Fixed;

#[composeable]
fn add_10(a: u32) -> Result<u32, String>{
    return Ok(a+10);
}

#[composeable]
fn add_20(a: u32) -> Result<u32, String>{
    return Ok(a+20);
}

#[composeable]
fn add_20_async(a: u32) -> BoxFuture<'static, Result<u32, String>>{
    async move{
         Ok(a+20)
    }.boxed()
}

#[composeable]
fn add_async(a: u32, b: u32) -> BoxFuture<'static, Result<u32, String>>{
    async move{
         Ok(a+b)
    }.boxed()
}

static mut RETRY_COUNT: i32 = 0;

fn retry_count_lessthan(count:i32) -> bool{
    unsafe{
        RETRY_COUNT < count
    }
}

fn reset_retry_count(){
    unsafe{
        RETRY_COUNT = 0;
    }
}

fn increment_retry_count(){
    unsafe{
        RETRY_COUNT = RETRY_COUNT + 1;
    }
}

#[composeable(retry = Fixed::from_millis(100).take(2))]
pub fn add_3_arg_ref_async<'a>(
    a: &'a u32,
    b: &'a u32,
    c: &'a u32,
) -> BoxFuture<'a, Result<u32, String>> {
    
        async move {
            println!("retrying..");
            if retry_count_lessthan(2) {
                increment_retry_count();                
                Err(
                    "Retry test".to_owned()
                )
            } else {
                let r = a + b + c;
                 reset_retry_count();
                Ok(r)
            }
        }.boxed()
}

#[composeable()]
pub fn add_3_arg_async(a: u32, b: u32, c: u32) -> BoxFuture<'static, Result<u32,String>> {
    async move {
        let r = a + b + c;
        Ok(r)
    }
    .boxed()
}

#[tokio::main]
async fn main() {
    assert_eq!(20, compose!(add_10 -> with_args(10)).unwrap());
    assert_eq!(40, compose!(add_10 -> add_20 -> with_args(10)).unwrap());
    assert_eq!(40, compose!(add_10 -> add_20_async -> with_args(10)).await.unwrap());
    assert_eq!(30, compose!(add_10 -> add_async.provide(10) -> with_args(10)).await.unwrap());
    assert_eq!(40, compose!(add_3_arg_async.provide(10).provide(10) -> add_10 -> with_args(10)).await.unwrap());

    let one = &1;    
    let result = compose!(add_3_arg_ref_async.provide(one).provide(one) -> add_3_arg_async.provide(1).provide(1) -> with_args(&10)).await;    
    assert_eq!(14, result.unwrap());

    let result = compose!(add_3_arg_ref_async.provide(one).provide(one) -> add_10 -> with_args(&10)).await;    
    assert_eq!(22, result.unwrap());
}
