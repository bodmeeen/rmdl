use std::io;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    Frame,
    DefaultTerminal,
};

#[derive(Debug, Default)]
pub struct App {
    pub url: String,
    pub download_progress: u16,
    pub print: String,
    pub exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| crate::ui::render(self, frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events (&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_events(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_events (&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.print_left(),
            KeyCode::Right => self.print_right(),
            _ => {}
        }
    }

    fn exit (&mut self) {
        self.exit = true;
    }

    fn print_left(&mut self) {
        self.print = String::from("left");
    }

    fn print_right(&mut self) {
        self.print = String::from("right");
    }
}