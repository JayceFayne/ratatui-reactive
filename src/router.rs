use crate::Render;
use crate::delay::delayed_signal;
use ratatui::Frame;
use std::fmt::Debug;
use std::rc::Rc;
use sycamore_reactive::{Signal, create_memo, provide_context};

#[derive(Debug, Clone, Copy)]
pub struct Router<R: 'static> {
    route: Signal<R>,
}

#[derive(Clone)]
pub struct Route {
    inner: Rc<dyn Render>,
}

impl Route {
    pub fn new<R: Render + 'static>(route: R) -> Self {
        Self {
            inner: Rc::new(route),
        }
    }
}

pub fn provide_router<R: 'static + Clone + Default + Debug>(
    mut mapping: impl FnMut(R) -> Route + 'static,
) -> impl Render {
    let (route, delayed_route) = delayed_signal(R::default());
    let router = Router { route };
    provide_context(router);
    let component = create_memo(move || mapping(delayed_route.get_clone()));
    move |frame: &mut Frame| {
        component.get_clone().inner.render(frame);
    }
}

impl<R: 'static + Copy> Router<R> {
    pub fn goto(&self, route: R) {
        self.route.set(route);
    }
}
