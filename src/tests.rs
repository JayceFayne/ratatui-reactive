use crate::{ReactiveApp, Runtime, init_mock_events, on_key_press};
use async_local_channel::spsc;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use std::time::Duration;
use sycamore_reactive::use_context;
use tokio::select;
use tokio::time::sleep;

#[tokio::test(flavor = "local")]
async fn quit() {
    let (event_tx, event_rx) = spsc::channel();

    let app = ReactiveApp::new(move || {
        init_mock_events(event_rx.activate());
        let runtime = use_context::<Runtime>();

        on_key_press(move |key| {
            if let KeyCode::Char('q') = key.code {
                runtime.quit()
            }
        });

        move |_: Rect, _: &mut Buffer| ()
    });
    assert!(app.draw_requested().await);
    let backend = TestBackend::new(10, 3);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| app.draw(frame)).unwrap();
    terminal
        .backend()
        .assert_buffer(&Buffer::empty(Rect::new(0, 0, 10, 3)));

    select! {
        _ = app.draw_requested() => panic!("shouldn't request redraw"),
        _= sleep(Duration::from_millis(10)) => (),
    }
    event_tx
        .send(Event::Key(KeyEvent::new(
            KeyCode::Char('q'),
            KeyModifiers::empty(),
        )))
        .unwrap();

    select! {
        keep_running = app.draw_requested() => assert!(!keep_running),
        _= sleep(Duration::from_millis(10)) => panic!("expected the app to quit"),
    }
}
