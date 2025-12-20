# function-compose ![License: MIT](https://img.shields.io/badge/license-MIT-blue) [![function-compose on crates.io](https://img.shields.io/crates/v/function-compose)](https://crates.io/crates/function-compose) [![function-compose on docs.rs](https://docs.rs/function-compose/badge.svg)](https://docs.rs/function-compose) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/senthil-veerabahu/fn-composer) ![Rust Version: 1.85.0](https://img.shields.io/badge/rustc-1.85.0-orange.svg)

Crate `function-compose` provides utilities for composing functions and way to inject arguments to functions

### Composing functions

#### step 1

Mark a function as composeable as below. Note that the functions must always return Result type

```rust
use function_compose::composeable;
#[composeable()]
pub fn add_10(a: i32) -> Result<i32, String> {
    Ok(a + 10)
}
#[composeable()]
pub fn add_100(a: i32) -> Result<i32, String> {
    Ok(a + 100)
}
```

#### step 2

use compose! macro to compose the above two functions.

```rust
let result = compose!(add_10 -> add_100 -> with_args(10));
assert_eq!(220, result.unwrap());
```

Argument 10(from with_args(10)). is passed to add_10 function and result of add_10 is passed to add_100

### composing Async functions

It is also possible to compose sync and asycn function.

###### <font color="#FFBF00"> **For async function,  return type should be BoxedFuture(futures crate)**</font>

```rust
use function_compose::composeable;
use futures::{future::BoxFuture, FutureExt};
#[composeable()]
pub fn add_async(a: i32, b: i32) -> BoxFuture<'static, Result<i32, String>> {
    async move {
        let r = a + b;
        Ok(r)
    }.boxed()
}
```

#### Composing async and sync functions usage

```rust
 use function_compose::compose;
 use fn_macros::composeable;
 use futures::{future::BoxFuture, FutureExt};
 #[composeable()]
 pub fn add_10_async(a: i32) -> BoxFuture<'static, Result<i32, String>> {
     async move {
         let r = a + 10;
         Ok(r)
     }.boxed()
 }
 #[composeable()]
 pub fn add_10(a: i32) -> Result<i32, String> {
     Ok(a + 10)
 }
 async fn test(){
    let result = compose!(add_async.add_10_async -> add_10 -> with_args(10)).await;
    assert_eq!(30, result.unwrap());
 }
 
```

### Injecting dependencies in multi-args function

For function with multiple arguments(say 2), One of the argument can be injected during composition itself.

##### Function argument injection usage

```rust
 use function_compose::composeable;
 use futures::{future::BoxFuture, FutureExt};
 #[composeable()]
 pub fn add_3_arg_async(a: i32,b: i32, c:i32) -> BoxFuture<'static, Result<i32, String>>{
     async move{
         let  r =   a + b + c;
         Ok(r)
     }.boxed()
 }
 use crate::compose;
 let result = compose!(add_3_arg_async.provide(100).provide(200) -> add_10 -> with_args(10)).await;
 assert_eq!(320, result.unwrap());
```

In the above example function add_3_arg_async, out of three arguments, 2 are injected during composing the function itself (using provide(100)) .
This feature could be used for injecting connection pool or a repository instance(see the example project).

### Retry in Fn Composer

Composeable macro supports retrying a function at specified interval in case of Error returned by the function.
This could be useful when trying make a database call or connect to network endpoint.
Make sure to add https://docs.rs/retry/latest/retry/ to your project before proceeding with retry feature.

Retry mechanism is implemented as part of composeable procedureal macro.
Below is example of  add_10  function configured to be retried 2 times after initial failure.

```rust
use retry::delay::*;
#[composeable(retry = Fixed::from_millis(100).take(2))]
pub fn add_10(a: i32) -> Result<i32, String> {
    Ok(a + 10)
}

```

Retry can be applied to both sync and async functions.

for async functions, <font color="#FFBF00"> **all arguments to the function must be either shared reference or exclusive reference.** </font>

Below is example of  async function with retry.

```rust
#[composeable(retry = Fixed::from_millis(100))]
pub fn add_3_arg_ref__non_copy_async<'a>(
    a: &'a mut Vec<String>,
    b: &'a mut Vec<String>,
    c: &'a Vec<String>,
) -> BoxFuture<'a, Result<i32, String>> {
    async move {
        let r = a.len() + b.len() + c.len();
        Ok(r as i32)
    }
    .boxed()
}
```

Apart from fixed duration retries, it is possible to configure with exponential delay.
Refer to retry documentation for all available delay options https://docs.rs/retry/latest/retry/all.html
