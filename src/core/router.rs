use std::rc::Rc;
use sycamore_reactive::{ReadSignal, Signal, create_memo, create_signal, provide_context};

use crate::Render;

#[derive(Debug, Clone, Copy)]
pub struct Router<R: 'static> {
    route: Signal<R>,
}

pub fn provide_router<R: 'static + Clone>(
    mut mapping: impl FnMut(R) -> Rc<dyn Render> + 'static,
    initial: R,
) -> ReadSignal<Rc<dyn Render>> {
    let route = create_signal(initial);
    let router = Router { route };
    provide_context(router);
    create_memo(move || mapping(route.get_clone()))
}

impl<R: 'static + Copy> Router<R> {
    pub fn goto(&self, route: R) {
        self.route.set(route);
    }
}
