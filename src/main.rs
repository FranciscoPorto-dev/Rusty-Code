use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{event::{self, KeyCode, KeyEventKind}},
    layout::{Constraint, Direction, Layout, Position},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Clear, Paragraph, Wrap},
};
use tui_big_text::{BigText, PixelSize};

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

    // Basically renders the input block / instructional text
    fn render(&self, frame: &mut Frame) {
        let available_width = (frame.area().width / 2) - 1;
        let input_len = self.input.chars().count() as u16;
        let input_height = (input_len / available_width.max(1) + 1) + 2;

        let centered_area = view::centered_rect(50, input_height + 1, frame.area());
        frame.render_widget(Clear, centered_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8),
                Constraint::Min(0),
            ])
            .vertical_margin(4)
            .split(frame.area());

        let title = BigText::builder()
            .pixel_size(PixelSize::Full)
            .centered()
            .lines(vec![
                Line::styled("RUSTY CODE", Style::default().fg(Color::Rgb(222, 100, 60))),
            ])
            .build();

        frame.render_widget(title, chunks[0]);

        let [help_area, input_area] = centered_area.layout(&Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(input_height),
        ]));

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
            .wrap(Wrap { trim: true })
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Rgb(222, 100, 60)),
            })
            .block(Block::bordered().title("Input"));
        frame.render_widget(input, input_area);

        match self.input_mode {
            InputMode::Normal => {}
            #[expect(clippy::cast_possible_truncation)]
            InputMode::Editing => {
                let cursor_row = self.character_index as u16 / available_width;
                let cursor_col = self.character_index as u16 % available_width;
                frame.set_cursor_position(Position::new(
                    input_area.x + cursor_col + 1,
                    input_area.y + cursor_row + 1,
                ));
            }
        }
    }
}
