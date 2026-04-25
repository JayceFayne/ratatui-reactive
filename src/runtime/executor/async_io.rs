use async_global_executor::Task;
use async_io::Timer;
use std::time::Duration;

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn spawn<F>(future: F) -> Task<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    async_global_executor::spawn(future)
}

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn spawn_local<F: Future + 'static>(future: F) {
    async_global_executor::spawn_local(future).detach()
}

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn sleep(duration: Duration) -> Timer {
    Timer::after(duration)
}
