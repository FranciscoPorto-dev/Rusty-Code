use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame, crossterm::event::{self, KeyCode, KeyEventKind}, layout::{Constraint, Layout, Position}, style::{Color, Modifier, Style, Stylize}, text::{Line, Text}, widgets::{Block, Paragraph},
};

mod model;
mod controller;

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
            terminal.draw(|frame| self.render(frame))?;
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

    /*
     * TODO:
     *  1. Make the input go into the center of the screen
     *  2. Reduce the input width and instead make it grow vertically as the user writes
     *  3. Maybe remove the normal mode and only keep the editing mode
     *  4. Find a way to use the messages that we are submitting to the input
     */
    fn render(&self, frame: &mut Frame) {
        let layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ]);
        let [help_area, input_area, messages_area] = frame.area().layout(&layout);
        let (msg, style) = match self.input_mode {
            InputMode::Normal => (
                vec![
                    "Press ".into(),
                    "q".bold(),
                    " to quit".into(),
                    " e".bold(),
                    " to edit".into(),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Editing => (
                vec![
                    "Press ".into(),
                    "Esc".bold(),
                    " to stop editing".into(),
                    " Enter".bold(),
                    " to submit".into(),
                ],
                Style::default(),
            ),
        };
        let text = Text::from(Line::from(msg)).patch_style(style);
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, help_area);

        let input = Paragraph::new(self.input.as_str())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::LightCyan),
            })
            .block(Block::bordered().title("Input"));
        frame.render_widget(input, input_area);

        match self.input_mode {
            InputMode::Normal => {}
            #[expect(clippy::cast_possible_truncation)]
            InputMode::Editing => frame.set_cursor_position(Position::new(input_area.x + self.character_index as u16 + 1, input_area.y + 1)),
        }
    }
}
