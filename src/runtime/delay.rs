use crate::spawn_local;
use async_local_channel::mpsc;
use sycamore_reactive::{Signal, create_signal, use_current_scope};

#[derive(Debug)]
pub struct DelayedSignal<T> {
    tx: mpsc::Sender<T>,
}

impl<T> Clone for DelayedSignal<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}

impl<T> DelayedSignal<T> {
    #[inline]
    pub fn set(&self, value: T) {
        if self.tx.is_empty() {
            self.tx.send(value).unwrap();
        }
    }
}

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn delayed_signal<T>(value: T) -> (DelayedSignal<T>, Signal<T>) {
    let output = create_signal(value);
    let (tx, rx) = mpsc::channel();
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
    (DelayedSignal { tx }, output)
}
