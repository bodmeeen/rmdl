use ratatui::{
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Paragraph},
    Frame,
};

// імпортування структури з app.rs
use crate::app::App;

    // ф-я відповідє тільки за малювання, приймає пряме посилання на App та полотно(Frame)
pub fn render(app: &App, frame: &mut Frame) {
    let title = Line::from(" rmdl " .bold());
    let instructions = Line::from(vec![
        " Print ".into(),
        " Q ".into(),
    ]);

    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered());
        // .border_set(border::THICK);

    let text_widget = Text::from(vec![Line::from(vec![
        "Value: ".into(),
        app.print.to_string().red(),
    ])]);

    let paragraph = Paragraph::new(text_widget)
        .centered()
        .block(block);
        // .render(area, buf);

    // параграф малюється на цілий екран
    frame.render_widget(paragraph, frame.area());
}
