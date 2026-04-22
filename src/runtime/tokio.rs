use std::time::Duration;
use tokio::task::spawn_local;
use tokio::time::Sleep;

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn spawn<F: Future + 'static>(future: F) {
    spawn_local(future);
}

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn sleep(duration: Duration) -> Sleep {
    tokio::time::sleep(duration)
}
