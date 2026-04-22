use color_eyre::Result;
use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui_reactive::{
    Render, Router, Runtime, create_interval, on_key_press, provide_router, run,
};
use std::rc::Rc;
use std::time::Duration;
use sycamore_reactive::{create_memo, create_signal, use_context};
use tokio::task::LocalSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum View {
    Menu,
    Counter,
    Input,
}

fn menu() -> impl Render {
    let selected = {
        let runtime = use_context::<Runtime>();
        let router = use_context::<Router<View>>();
        let selected = create_signal(0_usize);

        on_key_press(move |key| match key.code {
            KeyCode::Up => selected.update(|s| *s = (*s).saturating_sub(1)),
            KeyCode::Down => selected.update(|s| *s = (*s + 1).min(3)),
            KeyCode::Enter => {
                match selected() {
                    0 => router.goto(View::Counter),
                    1 => router.goto(View::Input),
                    2 => runtime.quit(),
                    _ => (),
                };
            }
            KeyCode::Char('q') => runtime.quit(),
            _ => (),
        });

        selected
    };

    let menu_items = ["Counter", "Input", "Quit"];

    let list = create_memo(move || {
        let items: Vec<ListItem> = menu_items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let content = if i == selected() {
                    format!("> {item}")
                } else {
                    format!("  {item}")
                };
                ListItem::new(content)
            })
            .collect();

        List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Menu (↑/↓: navigate, Enter: select, q: quit)"),
        )
    });

    move |frame: &mut Frame| {
        frame.render_widget(list.get_clone(), frame.area());
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

    move |frame: &mut Frame| {
        frame.render_widget(paragraph.get_clone(), frame.area());
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

    move |frame: &mut Frame| {
        frame.render_widget(paragraph.get_clone(), frame.area());
    }
}

fn create_view(view: View) -> Rc<dyn Render> {
    match view {
        View::Menu => Rc::new(menu()),
        View::Counter => Rc::new(counter()),
        View::Input => Rc::new(input()),
    }
}

fn app() -> impl Render {
    let view = provide_router(create_view, View::Menu);

    move |frame: &mut Frame| {
        view.get_clone().render(frame);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    LocalSet::new().run_until(run(app)).await?;
    Ok(())
}
