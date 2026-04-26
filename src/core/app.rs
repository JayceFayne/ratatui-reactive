use crate::{Component, Render};
use async_local_channel::watch;
use futures_lite::FutureExt;
use ratatui::Frame;
use std::mem;
use sycamore_reactive::{
    RootHandle, Signal, batch, create_effect, create_root, create_signal, provide_context,
};

#[derive(Debug, Clone)]
pub struct Runtime {
    request_draw: watch::Sender<()>,
    quit: watch::Sender<()>,
}

impl Runtime {
    #[inline]
    pub fn quit(&self) {
        self.quit.send(()).unwrap();
    }

    #[inline]
    pub fn request_draw(&self) {
        self.request_draw.send(()).unwrap();
    }
}

pub struct ReactiveApp {
    root: RootHandle,
    request_draw_rx: watch::Receiver<()>,
    quit_rx: watch::Receiver<()>,
    current_frame: Signal<Option<*mut Frame<'static>>>,
}

impl ReactiveApp {
    #[inline]
    pub fn new<R: Render + 'static, C: Component<R>>(component: C) -> ReactiveApp {
        let (request_draw, request_draw_rx) = watch::channel();
        let (quit, quit_rx) = watch::channel();

        let root = {
            let request_draw = request_draw.clone();
            create_root(move || {
                provide_context(Runtime { request_draw, quit });
            })
        };

        let request_draw_rx = request_draw_rx.activate();
        let quit_rx = quit_rx.activate();

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
                    request_draw.send(()).unwrap();
                }
            });
            current_frame
        });

        ReactiveApp {
            root,
            request_draw_rx,
            quit_rx,
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

    async fn on_quit(&self) -> bool {
        self.quit_rx.recv().await.unwrap();
        self.root.dispose();
        false
    }

    async fn on_draw_requested(&self) -> bool {
        self.request_draw_rx.recv().await.unwrap();
        true
    }

    #[inline]
    pub async fn draw_requested(&self) -> bool {
        self.on_quit().or(self.on_draw_requested()).await
    }
}

impl Drop for ReactiveApp {
    #[inline]
    fn drop(&mut self) {
        self.root.dispose();
    }
}
