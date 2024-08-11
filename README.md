# yield-now

Wakes current task and returns [`Poll::Pending`] once

This function can be used when we want to give
task scheduler time to pause before doing some
long running task.

this function will use runtime yield_now depending on
which feature is enabled

see [`tokio::task::yield_now`] for `tokio`

see [`async_std::task::yield_now`] for `async-std`

see [`futures_lite::future::yield_now`] for `futures-lite`
## Examples
```rust
use yield_now::yield_now;

yield_now().await;
```

License: MIT
