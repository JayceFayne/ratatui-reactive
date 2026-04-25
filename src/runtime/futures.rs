use crate::spawn;
use async_local_channel::oneshot;
use futures_lite::FutureExt;
use std::fmt::Debug;
use sycamore_reactive::{ReadSignal, create_signal, on_cleanup};
use tokio::task::spawn_local;

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn create_resource<F>(future: F) -> ReadSignal<Option<F::Output>>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static + Debug,
{
    let result = create_signal(None);
    let (tx, rx) = oneshot::channel();
    let rx = rx.activate();
    let handle = spawn(future);
    on_cleanup(move || {
        tx.send(()).unwrap();
    });
    spawn_local(async move {
        async move {
            result.set(Some(handle.await.unwrap()));
        }
        .or(async move { rx.recv().await.unwrap() })
        .await;
    });
    *result
}
