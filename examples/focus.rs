use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Widget};
use ratatui_reactive::{
    FocusManager, Focusable, Render, Runtime, on_key_press, provide_focus_manager, run,
};
use std::rc::Rc;
use sycamore_reactive::use_context;
use tokio::task::LocalSet;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum Focus {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl From<Focus> for u8 {
    fn from(value: Focus) -> Self {
        value as u8
    }
}

fn split_row(area: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area)
}

fn create_split(fun: impl FnMut(KeyEvent) + 'static) -> impl Render {
    let this = use_context::<Focusable>();

    on_key_press(fun);

    move |area: Rect, buf: &mut Buffer| {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(if this.is_focused() {
                Style::default().fg(Color::Red)
            } else {
                Style::default()
            });

        block.render(area, buf);
    }
}

fn top_left() -> impl Render {
    let focus_manager = use_context::<FocusManager<Focus>>();
    create_split(move |key| {
        let next = match key.code {
            KeyCode::Down => Focus::BottomLeft,
            KeyCode::Right => Focus::TopRight,
            _ => return,
        };
        focus_manager.focus(next);
    })
}

fn top_right() -> impl Render {
    let focus_manager = use_context::<FocusManager<Focus>>();
    create_split(move |key| {
        let next = match key.code {
            KeyCode::Down => Focus::BottomRight,
            KeyCode::Left => Focus::TopLeft,
            _ => return,
        };
        focus_manager.focus(next);
    })
}

fn bottom_left() -> impl Render {
    let focus_manager = use_context::<FocusManager<Focus>>();
    create_split(move |key| {
        let next = match key.code {
            KeyCode::Up => Focus::TopLeft,
            KeyCode::Right => Focus::BottomRight,
            _ => return,
        };
        focus_manager.focus(next);
    })
}

fn bottom_right() -> impl Render {
    let focus_manager = use_context::<FocusManager<Focus>>();
    create_split(move |key| {
        let next = match key.code {
            KeyCode::Up => Focus::TopRight,
            KeyCode::Left => Focus::BottomLeft,
            _ => return,
        };
        focus_manager.focus(next);
    })
}

fn app() -> impl Render {
    let runtime = use_context::<Runtime>();
    let focus_manager = provide_focus_manager(Focus::TopLeft);

    on_key_press(move |key| {
        if let KeyCode::Char('q') = key.code {
            runtime.quit()
        }
    });

    let top_left = focus_manager.on(Focus::TopLeft, top_left);
    let top_right = focus_manager.on(Focus::TopRight, top_right);
    let bottom_left = focus_manager.on(Focus::BottomLeft, bottom_left);
    let bottom_right = focus_manager.on(Focus::BottomRight, bottom_right);

    move |area: Rect, buf: &mut Buffer| {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let top = split_row(rows[0]);
        let bottom = split_row(rows[1]);

        top_left.render(top[0], buf);
        top_right.render(top[1], buf);
        bottom_left.render(bottom[0], buf);
        bottom_right.render(bottom[1], buf);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    LocalSet::new().run_until(run(app)).await?;
    Ok(())
}
