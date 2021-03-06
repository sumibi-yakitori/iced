//! Choose your preferred executor to power a runtime.
mod null;

#[cfg(feature = "thread-pool")]
mod thread_pool;

#[cfg(feature = "tokio")]
mod tokio;

#[cfg(feature = "async-std")]
mod async_std;

#[cfg(target_arch = "wasm32")]
mod wasm_bindgen;

pub use null::Null;

#[cfg(feature = "thread-pool")]
pub use thread_pool::ThreadPool;

#[cfg(feature = "tokio")]
pub use self::tokio::Tokio;

#[cfg(feature = "async-std")]
pub use self::async_std::AsyncStd;

#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen::WasmBindgen;

use futures::Future;

/// A type that can run futures.
pub trait Executor: Sized {
    /// Creates a new [`Executor`].
    ///
    /// [`Executor`]: trait.Executor.html
    fn new() -> Result<Self, futures::io::Error>
    where
        Self: Sized;

    /// Spawns a future in the [`Executor`].
    ///
    /// [`Executor`]: trait.Executor.html
    fn spawn(&self, future: impl Future<Output = ()> + Send + 'static);

    /// Runs the given closure inside the [`Executor`].
    ///
    /// Some executors, like `tokio`, require some global state to be in place
    /// before creating futures. This method can be leveraged to set up this
    /// global state, call a function, restore the state, and obtain the result
    /// of the call.
    fn enter<R>(&self, f: impl FnOnce() -> R) -> R {
        f()
    }
}
