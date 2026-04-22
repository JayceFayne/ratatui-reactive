use crate::runtime::{sleep, spawn};
use std::time::Duration;
use sycamore_reactive::{create_signal, use_current_scope};

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn create_timeout(fun: impl FnOnce() + 'static, duration: Duration) {
    let trigger = create_signal(());
    let scope = use_current_scope();
    spawn(async move {
        sleep(duration).await;
        if trigger.is_alive() {
            scope.run_in(fun);
        }
    });
}

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn create_interval(mut fun: impl FnMut() + 'static, duration: Duration) {
    let trigger = create_signal(());
    let scope = use_current_scope();
    spawn(async move {
        loop {
            sleep(duration).await;
            if trigger.is_alive() {
                scope.run_in(&mut fun);
            } else {
                return;
            }
        }
    });
}
