use color_eyre::Result;
use crossterm::event::KeyCode;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{
    Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget,
};
use ratatui_reactive::{
    Render, Route, Router, Runtime, create_interval, on_key_press, provide_router, run,
};
use std::time::Duration;
use sycamore_reactive::{create_memo, create_signal, use_context};
use tokio::task::LocalSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum View {
    #[default]
    Menu,
    Counter,
    Input,
}

fn menu() -> impl Render {
    let list_state = create_signal(ListState::default().with_selected(Some(0)));
    let runtime = use_context::<Runtime>();
    let router = use_context::<Router<View>>();

    on_key_press(move |key| match key.code {
        KeyCode::Up => list_state.update(|s| s.select_previous()),
        KeyCode::Down => list_state.update(|s| s.select_next()),
        KeyCode::Enter if let Some(selected) = list_state.get().selected() => {
            match selected {
                0 => router.goto(View::Counter),
                1 => router.goto(View::Input),
                2 => runtime.quit(),
                _ => (),
            };
        }
        KeyCode::Char('q') => runtime.quit(),
        _ => (),
    });

    let menu_items = ["Counter", "Input", "Quit"];
    let items: Vec<ListItem> = menu_items.into_iter().map(ListItem::new).collect();
    let list = List::new(items).highlight_symbol("> ").block(
        Block::default()
            .borders(Borders::ALL)
            .title("Menu (↑/↓: navigate, Enter: select, q: quit)"),
    );

    move |area: Rect, buf: &mut Buffer| {
        list_state.track();
        list_state.update(|s| StatefulWidget::render(&list, area, buf, s));
    }
}

fn counter() -> impl Render {
    let count = {
        let count = create_signal(0);
        create_interval(move || count.update(|c| *c += 1), Duration::from_secs(1));
        count
    };

    let router = use_context::<Router<View>>();
    let runtime = use_context::<Runtime>();
    on_key_press(move |key| match key.code {
        KeyCode::Char('b') => router.goto(View::Menu),
        KeyCode::Char('q') => runtime.quit(),
        _ => (),
    });

    let paragraph = create_memo(move || {
        let text = format!(
            "Count: {}\n\nPress 'b' or Esc to go back\nPress 'q' to quit",
            count()
        );
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Counter"))
    });

    move |area: Rect, buf: &mut Buffer| {
        paragraph.get_clone().render(area, buf);
    }
}

fn input() -> impl Render {
    let (text, history) = {
        let router = use_context::<Router<View>>();

        let text = create_signal(String::new());
        let history = create_signal(Vec::new());

        on_key_press(move |key| match key.code {
            KeyCode::Char(c) => text.update(|text| text.push(c)),
            KeyCode::Backspace => text.update(|text| {
                text.pop();
            }),
            KeyCode::Enter if !text.get_clone().is_empty() => {
                history.update(|history| history.push(text.get_clone().clone()));
                text.set(String::new());
            }
            KeyCode::Esc => router.goto(View::Menu),
            _ => (),
        });

        (text, history)
    };

    let paragraph = create_memo(move || {
        use std::fmt::Write;

        let mut content = String::from("Type to enter text, Enter to submit, Esc to go back\n\n");
        let _ = write!(content, "Input: {}_\n\n", text.get_clone());
        content.push_str("History:\n");

        history.with(|history| {
            for (i, item) in history.iter().enumerate() {
                let _ = writeln!(content, "  {}. {item}", i + 1);
            }
        });

        Paragraph::new(content).block(Block::default().borders(Borders::ALL).title("Text Input"))
    });

    move |area: Rect, buf: &mut Buffer| {
        paragraph.get_clone().render(area, buf);
    }
}

fn app() -> impl Render {
    let view = provide_router(|view| match view {
        View::Menu => Route::new(menu()),
        View::Counter => Route::new(counter()),
        View::Input => Route::new(input()),
    });

    move |area: Rect, buf: &mut Buffer| {
        view.render(area, buf);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    LocalSet::new().run_until(run(app)).await?;
    Ok(())
}
