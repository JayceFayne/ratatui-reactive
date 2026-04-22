use async_global_executor::spawn_local;
use async_io::Timer;
use std::time::Duration;

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn spawn<F: Future + 'static>(future: F) {
    spawn_local(future).detach()
}

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn sleep(duration: Duration) -> Timer {
    Timer::after(duration)
}
