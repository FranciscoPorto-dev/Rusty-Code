use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Clear, Paragraph, Wrap},
};
use tui_big_text::{BigText, PixelSize};

use crate::model::{App, InputMode};

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let vertical = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(percent_y),
        Constraint::Fill(1),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(vertical[1])[1]
}

pub fn render(app: &App, frame: &mut Frame) {
    let available_width = (frame.area().width / 2) - 1;
    let input_len = app.input.chars().count() as u16;
    let input_height = (input_len / available_width.max(1) + 1) + 2;

    let centered_area = centered_rect(50, input_height + 1, frame.area());
    frame.render_widget(Clear, centered_area);

    let chunks = Layout::default() // Layout for the title and help/input areas
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Min(0),
        ])
        .vertical_margin(4)
        .split(frame.area());
    
    // Main text saying "RUSTY CODE" using big text library for better pixelated look
    let title = BigText::builder()
        .pixel_size(PixelSize::Full)
        .centered()
        .lines(vec![
            Line::styled("RUSTY CODE", Style::default().fg(Color::Rgb(222, 100, 60))),
        ])
        .build();

    frame.render_widget(title, chunks[0]); // Render the title in the first chunk

    let [help_area, input_area] = centered_area.layout(&Layout::vertical([
        Constraint::Length(1), // Help area
        Constraint::Length(input_height), // Input area
    ]));

    let (msg, style) = match app.input_mode {
        InputMode::Normal => ( // Normal mode help message
            vec![
                "Press ".into(),
                "q".bold(),
                " to quit".into(),
                " e".bold(),
                " to edit".into(),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => ( // Editing mode help message
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
    let text = Text::from(Line::from(msg)).patch_style(style); // Create the help message text with the appropriate style
    let help_message = Paragraph::new(text);
    frame.render_widget(help_message, help_area); // Render the help message in the help area

    let input = Paragraph::new(app.input.as_str()) // Create the input text with the appropriate style
        .wrap(Wrap { trim: true })
        .style(match app.input_mode {
            InputMode::Normal => Style::default(), // Normal mode input style
            InputMode::Editing => Style::default().fg(Color::Rgb(222, 100, 60)), // Editing mode input style
        })
        .block(Block::bordered().title("Input")); // Create the input block
    frame.render_widget(input, input_area);

    match app.input_mode {
        InputMode::Normal => {}
        #[expect(clippy::cast_possible_truncation)]
        InputMode::Editing => {
            let cursor_row = app.character_index as u16 / available_width;
            let cursor_col = app.character_index as u16 % available_width;
            frame.set_cursor_position(Position::new(
                input_area.x + cursor_col + 1,
                input_area.y + cursor_row + 1,
            ));
        }
    }
}
