use crate::runtime::delay::DelayedSignal;
use crate::{Render, delayed_signal};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use std::fmt::Debug;
use std::rc::Rc;
use sycamore_reactive::{create_memo, provide_context};

#[derive(Debug, Clone)]
pub struct Router<R> {
    route: DelayedSignal<R>,
}

#[derive(Clone)]
pub struct Route {
    inner: Rc<dyn Render>,
}

impl Route {
    #[inline]
    pub fn new<R: Render + 'static>(route: R) -> Self {
        Self {
            inner: Rc::new(route),
        }
    }
}

#[inline]
#[cfg_attr(debug_assertions, track_caller)]
pub fn provide_router<R: 'static + Clone + Default + Debug>(
    mut mapping: impl FnMut(R) -> Route + 'static,
) -> impl Render {
    let (route, delayed_route) = delayed_signal(R::default());
    let router = Router { route };
    provide_context(router);
    let component = create_memo(move || mapping(delayed_route.get_clone()));
    move |area: Rect, buf: &mut Buffer| {
        component.get_clone().inner.render(area, buf);
    }
}

impl<R> Router<R> {
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn goto(&self, route: R) {
        self.route.set(route);
    }
}
