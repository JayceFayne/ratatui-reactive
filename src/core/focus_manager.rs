use crate::{Component, Render};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use std::fmt::Debug;
use std::marker::PhantomData;
use sycamore_reactive::{
    ReadSignal, Signal, create_child_scope, create_signal, on_cleanup, provide_context,
};

#[derive(Debug, Clone, Copy)]
pub struct FocusManager<R> {
    focus: Signal<u8>,
    marker: PhantomData<*mut R>,
}

#[derive(Debug, Clone)]
pub struct Focusable {
    route: u8,
    focus: ReadSignal<u8>,
}

pub fn provide_focus_manager<R: 'static + Clone + Debug + Into<u8>>(initial: R) -> FocusManager<R> {
    let focus = create_signal(initial.into());
    let focus_manager = FocusManager {
        focus,
        marker: PhantomData,
    };
    provide_context(focus_manager.clone());
    focus_manager
}

impl<F: Into<u8>> FocusManager<F> {
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

    pub fn focus(&self, route: F) {
        self.focus.set(route.into());
    }
}

impl Focusable {
    pub fn is_focused(&self) -> bool {
        self.focus.get_clone_untracked() == self.route
    }
}
