use ratatui::Frame;

pub trait Render {
    fn render(&self, frame: &mut Frame);
}

impl<F> Render for F
where
    F: Fn(&mut Frame),
{
    #[inline]
    fn render(&self, frame: &mut Frame) {
        (self)(frame)
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
