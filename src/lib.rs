#![cfg_attr(
    not(any(feature = "tokio", feature = "async-stds", feature = "futures-lite")),
    no_std
)]
//! Wakes current task and returns [`Poll::Pending`] once
//!
//! This function can be used when we want to give
//! task scheduler time to pause before doing some
//! long running task.
//!
//! # Examples
//! ```
//! use yield_now::yield_now;
//!
//! # spin_on::spin_on(async {
//! yield_now().await;
//! # });
//! ```
use core::task::{Context, Poll};

#[allow(unreachable_code)]
/// Wakes current task and returns [`Poll::Pending`] once
///
/// this function will use runtime yield_now depending on
/// which feature is enabled
///
/// see [`tokio::task::yield_now`] for `tokio`
///
/// see [`async_std::task::yield_now`] for `async-std`
///
/// see [`futures_lite::future::yield_now`] for `futures-lite`
pub async fn yield_now() {
    #[cfg(feature = "tokio")]
    if tokio::runtime::Handle::try_current().is_ok() {
        return tokio::task::yield_now().await;
    }

    #[cfg(feature = "async-std")]
    return async_std::task::yield_now().await;

    #[cfg(feature = "futures-lite")]
    return futures_lite::future::yield_now().await;

    YieldNow::Yield.await
}

#[derive(Clone, Copy, Debug)]
pub enum YieldNow {
    Yield,
    Ready,
}

impl core::future::Future for YieldNow {
    type Output = ();
    fn poll(mut self: core::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match *self {
            Self::Yield => {
                *self = Self::Ready;
                cx.waker().wake_by_ref();

                Poll::Pending
            }
            Self::Ready => Poll::Ready(()),
        }
    }
}
