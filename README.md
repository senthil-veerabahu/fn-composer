# fn-composer

fn-composer is a function composition library that allows composition of async and sync functions

There are two macros required for the fn-composer function composition to work.

Proc attribute Macro - composeable

Declarative Macro - compose!

Proc attribute macro composeable is add to any function that can be composed.
e.g below is simple function to add 10 to a given number.
In the current version, only  **single argument sync functions, single arg async function and two arg async functions
are
supported.**

```rust
#[composeable()]
pub fn add_10(a: i32) -> Result<i32, FnError> {
    Ok(a + 10)
}

```

Below snippet is an async example. The async function should return BoxFuture and the error type should be FnError.
Currently only two argument async functions are supported.

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

It is possible to compose single arg sync function with two arg async function and vice versa.

Below are the examples of how to compose async functions

```rust
let result = compose!(add_10 -> add_100 -> withArgs(10));
let result = compose!(add_10 -> add_100 -> add_10 -> add_100 -> withArgs(10));

```

Below are examples of composing async and sync functions

```rust
let result = compose!(add_100_async -> add_100 -> withArgs(10)).await;
assert_eq!(210, result.unwrap());


let result = compose!(add_100_async -> add_100_async -> withArgs(10)).await;
assert_eq!(210, result.unwrap());

```

It is possible to inject the seconds args of async functions using `.provide` method.
This could be useful for injecting database connection or other external service interaction.
See the example below

```rust
// Check the '.provide' method below of injecting second args to add_async function
let result = compose!(add_async.provide(100) -> add_100_async -> withArgs(10)).await;
assert_eq!(210, result.unwrap());
```


