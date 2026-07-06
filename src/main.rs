use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, KeyCode, KeyModifiers},
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
            edit_history: Vec::new(),
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
                    InputMode::Editing => {
                        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
                        let shortcut = key
                            .modifiers
                            .intersects(KeyModifiers::CONTROL | KeyModifiers::SUPER);
                        match key.code {
                            KeyCode::Char(c) if ctrl && matches!(c, 'a' | 'A') => {
                                self.move_cursor_to_start();
                            }
                            KeyCode::Char(c) if ctrl && matches!(c, 'u' | 'U') => {
                                self.delete_to_line_start();
                            }
                            KeyCode::Char(c) if ctrl && matches!(c, 'z' | 'Z') => {
                                self.undo();
                            }
                            KeyCode::Char(c) if shortcut && matches!(c, 'c' | 'C') => {
                                self.clear_input();
                            }
                            KeyCode::Char(c) if shortcut && matches!(c, 'v' | 'V') => {
                                self.paste();
                            }
                            KeyCode::Enter => self.submit_message(),
                            KeyCode::Char(to_insert) if !shortcut => self.enter_char(to_insert),
                            KeyCode::Backspace => self.delete_char(),
                            KeyCode::Left => self.move_cursor_left(),
                            KeyCode::Right => self.move_cursor_right(),
                            KeyCode::Esc => self.input_mode = InputMode::Normal,
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}
