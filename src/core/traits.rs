use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

pub trait Render {
    fn render(&self, area: Rect, buf: &mut Buffer);
}

impl<F> Render for F
where
    F: Fn(Rect, &mut Buffer),
{
    #[inline]
    fn render(&self, area: Rect, buf: &mut Buffer) {
        (self)(area, buf)
    }
}

pub trait Component<F: Render> {
    fn create(self) -> F;
}

impl<F, C> Component<F> for C
where
    C: FnOnce() -> F,
    F: Render,
{
    #[inline]
    fn create(self) -> F {
        (self)()
    }
}
