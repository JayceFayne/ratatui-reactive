use crate::{Component, Render};
use async_local_channel::mpsc;
use ratatui::Frame;
use std::fmt::Debug;
use std::mem;
use sycamore_reactive::{
    RootHandle, Signal, batch, create_effect, create_memo, create_root, create_signal,
    provide_context,
};

#[derive(Debug, Clone, Copy)]
pub struct Runtime {
    request_draw: Signal<bool>,
}

impl Runtime {
    #[inline]
    pub fn quit(&self) {
        self.request_draw.set(false)
    }

    #[inline]
    pub fn request_draw(&self) {
        self.request_draw.set(true)
    }
}

pub struct ReactiveApp {
    root: RootHandle,
    request_draw_rx: mpsc::Receiver<bool>,
    current_frame: Signal<Option<*mut Frame<'static>>>,
}

impl ReactiveApp {
    #[inline]
    pub fn new<R: Render + 'static, C: Component<R>>(component: C) -> ReactiveApp {
        let (request_draw_tx, request_draw_rx) = mpsc::channel();

        let root = {
            let request_draw_tx = request_draw_tx.clone();
            create_root(move || {
                let request_draw = create_signal(true);
                provide_context(Runtime { request_draw });
                create_memo(move || request_draw_tx.send(request_draw.get()).unwrap());
            })
        };

        let request_draw_rx = request_draw_rx.activate();

        let current_frame = root.run_in(|| {
            let current_frame: Signal<Option<*mut Frame>> = create_signal(None);
            let app = batch(move || component.create());
            create_effect(move || {
                current_frame.track();
                if let Some(current_frame) = current_frame.replace_silent(None) {
                    // SAFETY: we set this frame once every `draw`
                    app.render(unsafe { &mut *current_frame })
                } else {
                    request_draw_tx.send(true).unwrap();
                }
            });
            current_frame
        });

        ReactiveApp {
            root,
            request_draw_rx,
            current_frame,
        }
    }

    /// # Safety
    ///
    /// This function should only be called with the same type that was used in `new`
    #[inline]
    pub fn draw(&self, frame: &mut Frame) {
        // SAFETY: this will trigger exactly one `render` call
        let frame = unsafe { mem::transmute(frame) };
        self.root
            .run_in(move || self.current_frame.set(Some(frame)))
    }

    #[inline]
    pub async fn draw_requested(&self) -> bool {
        let draw_requested = self.request_draw_rx.recv().await.unwrap();
        if !draw_requested {
            self.root.dispose();
        }
        draw_requested
    }
}

impl Drop for ReactiveApp {
    #[inline]
    fn drop(&mut self) {
        self.root.dispose();
    }
}
