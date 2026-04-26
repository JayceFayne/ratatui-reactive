use crate::spawn_local;
use async_local_channel::watch;
use std::fmt::Debug;
use sycamore_reactive::{ReadSignal, create_signal, use_current_scope};

#[derive(Debug, Clone)]
pub struct DelayedSignal<T> {
    tx: watch::Sender<T>,
}

impl<T> DelayedSignal<T> {
    #[inline]
    pub fn set(&self, value: T) {
        self.tx.send(value).unwrap();
    }
}

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn delayed_signal<T: Clone>(value: T) -> (DelayedSignal<T>, ReadSignal<T>) {
    let output = create_signal(value);
    let (tx, rx) = watch::channel();
    let rx = rx.activate();
    let scope = use_current_scope();
    spawn_local(async move {
        loop {
            let value = rx.recv().await.unwrap();
            if output.is_alive() {
                scope.run_in(move || output.set(value))
            } else {
                return;
            }
        }
    });
    (DelayedSignal { tx }, *output)
}
