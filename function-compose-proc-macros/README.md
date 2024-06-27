### Composeable macro Usage

##### Sync function

```rust
#[composeable()]
pub fn add_10(a: i32) -> Result<i32, FnError> {
    Ok(a + 10)
}

```

##### Async Function
The async function should return BoxFuture and the error type should be FnError.



```rust
#[composeable()]
pub fn add_async(a: i32, b: i32) -> BoxFuture<'static, Result<i32, FnError>> {
    async move {
        let r = a + b;
        Ok(r)
    }.boxed()
}
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