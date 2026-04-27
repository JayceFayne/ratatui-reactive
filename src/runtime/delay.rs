use crate::spawn_local;
use async_local_channel::spsc;
use sycamore_reactive::{Signal, create_effect, create_signal, use_current_scope};

#[derive(Debug)]
pub struct DelayedSet<T: 'static> {
    signal: Signal<T>,
}

impl<T: 'static> Clone for DelayedSet<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: 'static> Copy for DelayedSet<T> {}

impl<T: 'static> DelayedSet<T> {
    #[inline]
    pub fn set(&self, value: T) {
        self.signal.set(value);
    }
}

#[derive(Debug)]
pub struct DelayedGet<T: 'static> {
    signal: Signal<T>,
}

impl<T: 'static> Clone for DelayedGet<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: 'static> Copy for DelayedGet<T> {}

impl<T: 'static + Default> DelayedGet<T> {
    #[inline]
    pub fn get(&self) -> T {
        self.signal.track();
        self.signal.take_silent()
    }
}

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn delayed_signal<T: Default>(value: T) -> (DelayedSet<T>, DelayedGet<T>) {
    let input = create_signal(T::default());
    let output = create_signal(value);
    let (tx, rx) = spsc::channel();
    create_effect(move || {
        input.track();
        tx.send(input.take_silent()).unwrap();
    });
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
    (DelayedSet { signal: input }, DelayedGet { signal: output })
}
