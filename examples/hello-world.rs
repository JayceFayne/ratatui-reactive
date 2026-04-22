use color_eyre::Result;
use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::widgets::Paragraph;
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

    move |frame: &mut Frame| {
        frame.render_widget(&greeting, frame.area());
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    LocalSet::new().run_until(run(app)).await?;
    Ok(())
}
