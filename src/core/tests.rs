use crate::ReactiveApp;
use ratatui::Frame;
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use std::time::Duration;
use tokio::select;
use tokio::time::sleep;

#[tokio::test(flavor = "local")]
async fn first_draw() {
    let app = ReactiveApp::new(move || |_: &mut Frame| ());
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
}
