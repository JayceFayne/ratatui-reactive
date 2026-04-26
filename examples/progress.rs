use color_eyre::Result;
use crossterm::event::KeyCode;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Gauge, Widget};
use ratatui_reactive::{
    Render, Runtime, Signal, create_memo, create_progress, create_signal, on_key_press, run, sleep,
};
use std::time::Duration;
use sycamore_reactive::use_context;
use tokio::task::LocalSet;

async fn fake_async_work(progress: Signal<u16>) {
    for i in 0..=100 {
        sleep(Duration::from_millis(50)).await;
        progress.set(i);
    }
}

fn app() -> impl Render {
    let runtime = use_context::<Runtime>();

    let reset = {
        let reset = create_signal(());

        on_key_press(move |key| {
            if let KeyCode::Char('r') = key.code {
                reset.set(());
            }
        });
        *reset
    };

    let (_, progress) = create_progress(move |p| {
        reset.track();
        fake_async_work(p)
    });

    on_key_press(move |key| {
        if let KeyCode::Char('q') = key.code {
            runtime.quit()
        }
    });

    let gauge = create_memo(move || {
        Gauge::default()
            .block(Block::bordered().title("Progress"))
            .percent(progress())
    });

    move |area: Rect, buf: &mut Buffer| {
        gauge.with(|g| g.render(area, buf));
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    LocalSet::new().run_until(run(app)).await?;
    Ok(())
}
