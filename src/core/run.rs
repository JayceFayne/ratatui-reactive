use crate::{Component, ReactiveApp, Render};
use ratatui::Terminal;
use ratatui::backend::Backend;

#[inline]
pub async fn run_with_terminal<B: Backend, F: Render + 'static, C: Component<F>>(
    app: C,
    terminal: &mut Terminal<B>,
) -> Result<(), B::Error> {
    let app = ReactiveApp::new(app);
    while app.draw_requested().await {
        terminal.draw(|frame| app.draw(frame))?;
    }
    Ok(())
}
