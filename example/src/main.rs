use futures::future::BoxFuture;
use futures::FutureExt;
use retry::delay::Fixed;
use fn_compose::{composeable};

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


#[tokio::main]
async fn main() {
    let result = compose!(add_10 -> add_100 ->add_async.provide(1000) ->with_args(10000)).await;
    println!("{}", result.unwrap());
}
