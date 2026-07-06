use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, KeyCode, KeyEventKind},
};

mod controller;
mod model;
mod view;

use crate::model::{App, InputMode};

fn main() -> Result<()> {
    color_eyre::install()?;
    // begin terminal using the App struct found in model.rs
    ratatui::run(|terminal| App::new().run(terminal))
}

impl App {
    const fn new() -> Self {
        // Initialize the App struct with default values
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            character_index: 0,
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| view::render(self, frame))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            // Enter editing mode
                            self.input_mode = InputMode::Editing;
                        }
                        // quit the tui
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        _ => {}
                    },
                    InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => self.submit_message(),
                        KeyCode::Char(to_insert) => self.enter_char(to_insert),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Esc => self.input_mode = InputMode::Normal,
                        _ => {}
                    },
                    InputMode::Editing => {}
                }
            }
        }
    }
}
