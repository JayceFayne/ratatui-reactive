use crate::{Component, Render};
use async_local_channel::mpsc;
use ratatui::Frame;
use std::fmt::Debug;
use std::mem;
use sycamore_reactive::{
    RootHandle, Signal, batch, create_effect, create_root, create_signal, provide_context,
};

#[derive(Debug, Clone)]
pub struct Runtime {
    request_draw: mpsc::Sender<bool>,
}

impl Runtime {
    #[inline]
    pub fn quit(&self) {
        self.request_draw.send(false).unwrap();
    }

    #[inline]
    pub fn request_draw(&self) {
        self.request_draw.send(true).unwrap();
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
        let (request_draw, request_draw_rx) = mpsc::channel();

        let root = {
            let request_draw = request_draw.clone();
            create_root(move || {
                provide_context(Runtime { request_draw });
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
                    let frame = unsafe { &mut *current_frame };
                    app.render(frame.area(), frame.buffer_mut())
                } else {
                    request_draw.send(true).unwrap();
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
