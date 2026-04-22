use crate::runtime::spawn;
use crate::{Component, Render, Runtime, run_with_terminal};
use async_local_channel::{broadcast, mpsc};
use crossterm::event::EventStream;
use crossterm::event::{Event, KeyEvent, MouseEvent};
use futures_lite::stream::StreamExt;
use std::io::Error;
use sycamore_reactive::{create_signal, provide_context, use_context, use_current_scope};

#[derive(Debug, Clone)]
pub(crate) struct Events(broadcast::InactiveReceiver<Event>);

#[inline]
fn init_events() {
    let (event_tx, event_rx) = broadcast::channel();
    provide_context(Events(event_rx));
    let scope = use_current_scope();
    let trigger = create_signal(());
    spawn(async move {
        let mut event_stream = EventStream::new();
        while let Some(event) = event_stream.next().await {
            if trigger.is_alive() {
                scope.run_in(|| event_tx.send(event.unwrap()).unwrap());
            } else {
                return;
            }
        }
    })
}

#[inline]
pub fn init_mock_events(rx: mpsc::Receiver<Event>) {
    let (event_tx, event_rx) = broadcast::channel();
    provide_context(Events(event_rx));
    let scope = use_current_scope();
    let trigger = create_signal(());
    spawn(async move {
        loop {
            let event = rx.recv().await.unwrap();
            if trigger.is_alive() {
                scope.run_in(|| event_tx.send(event).unwrap());
            } else {
                return;
            }
        }
    })
}

#[inline]
pub async fn run<F: Render + 'static, C: Component<F>>(app: C) -> Result<(), Error> {
    let mut terminal = ratatui::init();
    let app = move || {
        let runtime = use_context::<Runtime>();
        init_events();
        on_resize(move |_, _| runtime.request_draw());
        app.create()
    };
    let res = run_with_terminal(app, &mut terminal).await;
    ratatui::restore();
    res
}
#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn on_event(mut fun: impl FnMut(Event) + 'static) {
    let mut rx = use_context::<Events>().0.activate();
    let scope = use_current_scope();
    let trigger = create_signal(());
    spawn(async move {
        loop {
            let event = rx.recv().await.unwrap();
            if trigger.is_alive() {
                scope.run_in(|| fun(event));
            } else {
                return;
            }
        }
    })
}

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn on_key(mut fun: impl FnMut(KeyEvent) + 'static) {
    on_event(move |event| {
        if let Some(key_event) = event.as_key_event() {
            fun(key_event);
        }
    })
}

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn on_mouse(mut fun: impl FnMut(MouseEvent) + 'static) {
    on_event(move |event| {
        if let Some(mouse_event) = event.as_mouse_event() {
            fun(mouse_event);
        }
    })
}

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn on_resize(mut fun: impl FnMut(u16, u16) + 'static) {
    on_event(move |event| {
        if let Some((x, y)) = event.as_resize_event() {
            fun(x, y);
        }
    })
}

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn on_key_press(mut fun: impl FnMut(KeyEvent) + 'static) {
    on_event(move |event| {
        if let Some(key_event) = event.as_key_press_event() {
            fun(key_event);
        }
    })
}
