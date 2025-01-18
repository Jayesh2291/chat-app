use tui::{widgets::{Paragraph, Text}, layout::{Layout, Constraint, Direction}, Terminal};
use tui::backend::CrosstermBackend;

pub fn draw_chat_ui(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) {
    terminal.draw(|f| {
        let size = f.size();
        let block = Paragraph::new(Text::from("Welcome to the chat"))
            .block(tui::widgets::Block::default().borders(tui::widgets::Borders::ALL))
            .alignment(tui::layout::Alignment::Center);
        f.render_widget(block, size);
    }).unwrap();
}
