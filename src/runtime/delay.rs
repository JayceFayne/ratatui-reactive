use crate::spawn_local;
use async_local_channel::spsc;
use std::fmt::Debug;
use sycamore_reactive::{ReadSignal, Signal, create_effect, create_signal, use_current_scope};

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn delayed_signal<T: Debug + Clone>(value: T) -> (Signal<T>, ReadSignal<T>) {
    let input = create_signal(value.clone());
    let output = create_signal(value);
    let (tx, rx) = spsc::channel();
    let rx = rx.activate();
    create_effect(move || {
        tx.send(input.get_clone()).unwrap();
    });
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
    (input, *output)
}
