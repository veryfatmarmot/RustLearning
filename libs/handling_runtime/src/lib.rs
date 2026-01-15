use anyhow::Result;

pub trait HandlingRuntime {
    fn execute(&self, job: Box<dyn FnOnce() -> Result<()> + Send + 'static>) -> Result<()>;
}

#[cfg(all(feature = "thread_pool_runtime", feature = "tokio_async_runtime"))]
compile_error!(
    "Only one runtime type can be enabled as a feature \"thread_pool_runtime\" or \"tokio_async_runtime\""
);

#[cfg(not(any(feature = "thread_pool_runtime", feature = "tokio_async_runtime")))]
compile_error!(
    "At least one runtime type can be enabled as a feature \"thread_pool_runtime\" or \"tokio_async_runtime\""
);

#[cfg(feature = "thread_pool_runtime")]
mod thread_pool_runtime;
#[cfg(feature = "thread_pool_runtime")]
pub fn get_runtime() -> Box<dyn HandlingRuntime> {
    println!("Using tread pool handling runtime");
    Box::new(thread_pool_runtime::ThreadPoolHandlingRuntime::new())
}

#[cfg(feature = "tokio_async_runtime")]
mod tokio_async_runtime;
#[cfg(feature = "tokio_async_runtime")]
pub fn get_runtime() -> Box<dyn HandlingRuntime> {
    println!("Using tread pool handling runtime");
    Box::new(tokio_async_runtime::TokioAsyncHandlingRuntime::new())
}
