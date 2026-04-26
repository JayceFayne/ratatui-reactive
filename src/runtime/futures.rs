use crate::spawn_local;
use async_local_channel::mpmc;
use futures_lite::FutureExt;
use sycamore_reactive::{ReadSignal, Signal, create_effect, create_signal, on_cleanup};

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn create_resource<F>(fut_gen: impl Fn() -> F + 'static) -> ReadSignal<Option<F::Output>>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let (a_tx, a_rx) = mpmc::channel();
    {
        let a_tx = a_tx.clone();
        on_cleanup(move || {
            a_tx.send(()).unwrap();
        });
    }
    let result = create_signal(None);
    create_effect(move || {
        result.set(None);
        a_tx.send(()).unwrap();
        let a_rx = a_rx.clone().activate();
        let fut = fut_gen();
        spawn_local(async move {
            async move {
                a_rx.recv().await.unwrap();
            }
            .or(async move {
                result.set(Some(fut.await));
            })
            .await;
        });
    });
    *result
}

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn create_progress<F>(
    fut_gen: impl Fn(Signal<u16>) -> F + 'static,
) -> (ReadSignal<Option<F::Output>>, ReadSignal<u16>)
where
    F: Future + 'static,
    F::Output: 'static,
{
    let (a_tx, a_rx) = mpmc::channel();
    {
        let a_tx = a_tx.clone();
        on_cleanup(move || {
            a_tx.send(()).unwrap();
        });
    }
    let result = create_signal(None);
    let progress = create_signal(0);
    create_effect(move || {
        result.set(None);
        progress.set(0);
        a_tx.send(()).unwrap();
        let a_rx = a_rx.clone().activate();
        let fut = fut_gen(progress);
        spawn_local(async move {
            async move {
                a_rx.recv().await.unwrap();
            }
            .or(async move {
                result.set(Some(fut.await));
            })
            .await;
        });
    });
    (*result, *progress)
}
