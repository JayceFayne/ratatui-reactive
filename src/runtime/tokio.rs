use std::time::Duration;
use tokio::task;
use tokio::time::Sleep;

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn spawn<F>(future: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    task::spawn(future);
}

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn spawn_local<F: Future + 'static>(future: F) {
    task::spawn_local(future);
}

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn sleep(duration: Duration) -> Sleep {
    tokio::time::sleep(duration)
}
