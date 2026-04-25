use color_eyre::Result;
use crossterm::event::KeyCode;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Paragraph, Widget};
use ratatui_reactive::{Render, Runtime, on_key_press, run};
use sycamore_reactive::use_context;
use tokio::task::LocalSet;

fn app() -> impl Render {
    let runtime = use_context::<Runtime>();

    on_key_press(move |key| {
        if let KeyCode::Char('q') = key.code {
            runtime.quit()
        }
    });

    let greeting = Paragraph::new("Hello World! (press 'q' to quit)");

    move |area: Rect, buf: &mut Buffer| {
        (&greeting).render(area, buf);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    LocalSet::new().run_until(run(app)).await?;
    Ok(())
}
