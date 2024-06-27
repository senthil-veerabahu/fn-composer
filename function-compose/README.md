# function-compose

function-compose is a function composition library that allows composition of async and sync functions

There are two macros required for the fn-composer function composition to work.

Proc attribute Macro - composeable

Declarative Macro - compose!

Proc attribute macro composeable is add to any function that can be composed.
e.g below is simple function to add 10 to a given number.


 [Composeable macro documentation](./funciton-compose-proc-macros/README.MD)
```rust
#[composeable()]
pub fn add_10(a: i32) -> Result<i32, FnError> {
    Ok(a + 10)
}

```

Below snippet is an async example. The async function should return BoxFuture and the error type should be FnError.


```rust
#[composeable()]
pub fn add_async(a: i32, b: i32) -> BoxFuture<'static, Result<i32, FnError>> {
    async move {
        let r = a + b;
        Ok(r)
    }.boxed()
}
```

Below is example of async multiplication and add_100 to a number asynchronously

```rust
#[composeable()]
pub fn multiply_async(a: i32, b: i32) -> BoxFuture<'static, Result<i32, FnError>> {
    async move {
        let r = a * b;
        Ok(r)
    }.boxed()
}

#[composeable()]
pub fn add_100_async(a: i32) -> BoxFuture<'static, Result<i32, FnError>> {
    async move {
        let r = a + 100;
        Ok(r)
    }.boxed()
}
```

Below is example of async addition of 3 values

```rust
#[composeable()]
pub fn add_3_arg_async(a: i32,b: i32, c:i32) -> BoxFuture<'static, Result<i32, FnError>>{
    async move{
        let  r =   a + b + c;
        Ok(r)
    }.boxed()
}

```

Below is example of async addition of 3 values with last parameter accepting reference

```rust
#[composeable()]
pub fn add_3_arg_ref_async<'a>(a: i32,b: &'a i32, c:&'a i32) -> BoxFuture<'a, Result<i32, FnError>>{
    async move{
        let  r =   a + b + c;
        Ok(r)
    }.boxed()
}

```

It is possible to compose single arg sync function with two arg async function and vice versa.

Below are the examples of how to compose async functions

```rust
let result = compose!(add_10 -> add_100 -> with_args(10));
let result = compose!(add_10 -> add_100 -> add_10 -> add_100 -> with_args(10));

```

Below are examples of composing async and sync functions

```rust
let result = compose!(add_100_async -> add_100 -> with_args(10)).await;
assert_eq!(210, result.unwrap());


let result = compose!(add_100_async -> add_100_async -> with_args(10)).await;
assert_eq!(210, result.unwrap());

```

It is possible to inject the seconds args of async functions using `.provide` method.
This could be useful for injecting database connection or other external service interaction.
See the example below

```rust
// Check the '.provide' method below of injecting second args to add_async function
let result = compose!(add_async.provide(100) -> add_100_async -> with_args(10)).await;
assert_eq!(210, result.unwrap());
```

Example of multiple injection to async function  
```rust
let result = compose!(add_3_arg_async.provide(100).provide(200) -> add_100_async -> with_args(10)).await;
assert_eq!(410, result.unwrap());
```

Example of multiple injection to async function in the second position
```rust

let result = compose!(add_100 -> add_3_arg_async.provide(1).provide(1) -> with_args(10)).await;
assert_eq!(112, result.unwrap());
```

Example of multiple injection to shared reference to async function

```rust
let one = &1;
let result = compose!(add_3_arg_ref_async.provide(one).provide(one) -> add_3_arg_async.provide(1).provide(1) -> with_args(10)).await;
assert_eq!(14, result.unwrap());
```


### Retry in Fn Composer

Composeable macro supports retrying a function at specified interval in case of Error returned by the function.
This could be useful when trying make a database call or connect to network endpoint.
Make sure to install https://docs.rs/retry/latest/retry/ before proceeding with retry feature.

Retry mechanism is implemented as part of composeable procedureal macro.
Below is example of  add_10  function configured to be retried 2 times after initial failure. 

```rust
use retry::delay::*;
#[composeable(retry = Fixed::from_millis(100).take(2))]
pub fn add_10(a: i32) -> Result<i32, FnError> {
    Ok(a + 10)
}

```

Retry can be applied to both sync and async functions.

for async functions, <font color="#FFBF00"> __all arguments to the function must be either shared reference or exclusive reference.__ </font> 

Below is example of  async function with retry.

```rust
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
```

Apart from fixed duration retries, it is possible to configure with exponential delay. 
Refer to retry documentation for all available delay options https://docs.rs/retry/latest/retry/all.html