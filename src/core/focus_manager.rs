use crate::{Component, Render};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use std::marker::PhantomData;
use sycamore_reactive::{
    ReadSignal, Signal, create_child_scope, create_signal, on_cleanup, provide_context,
};

#[derive(Debug)]
pub struct FocusManager<R> {
    focus: Signal<u8>,
    marker: PhantomData<*mut R>,
}

impl<R> Clone for FocusManager<R> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<R> Copy for FocusManager<R> {}

#[derive(Debug, Clone, Copy)]
pub struct Focusable {
    route: u8,
    focus: ReadSignal<u8>,
}

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn provide_focus_manager<R: 'static + Into<u8>>(initial: R) -> FocusManager<R> {
    let focus = create_signal(initial.into());
    let focus_manager = FocusManager {
        focus,
        marker: PhantomData,
    };
    provide_context(focus_manager);
    focus_manager
}

impl<F: Into<u8>> FocusManager<F> {
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn on<R: Render + 'static, C: Component<R>>(self, route: F, component: C) -> impl Render {
        let scope = create_child_scope(|| {
            provide_context(Focusable {
                route: route.into(),
                focus: *self.focus,
            })
        });
        on_cleanup(move || scope.dispose());
        let wrapped = scope.run_in(move || component.create());
        move |area: Rect, buf: &mut Buffer| {
            self.focus.track();
            wrapped.render(area, buf);
        }
    }

    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn focus(&self, route: F) {
        self.focus.set(route.into());
    }
}

impl Focusable {
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn is_focused(&self) -> bool {
        self.focus.with_untracked(|r| r == &self.route)
    }
}
