use std::{i128::MIN, io, vec};

use canvas::Canvas;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

mod tui;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    gamestate: [u8; 9],
    cursor_pos: u8,
    x_turn: bool,
    exit: bool,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self ,key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('w') => self.move_cursor(0),
            KeyCode::Char('a') => self.move_cursor(1),
            KeyCode::Char('s') => self.move_cursor(2),
            KeyCode::Char('d') => self.move_cursor(3),
            KeyCode::Char(' ') => self.select(),
            _ => {}
        }
    }

    fn move_cursor(&mut self, direction: u8) {
        match direction {
            0 => self.cursor_pos =  if self.cursor_pos < 3 {self.cursor_pos} else {self.cursor_pos - 3},
            1 => self.cursor_pos = if self.cursor_pos % 3 == 0 {self.cursor_pos} else {self.cursor_pos - 1},
            2 => self.cursor_pos = if self.cursor_pos > 5 {self.cursor_pos} else {self.cursor_pos + 3},
            3 => self.cursor_pos = if self.cursor_pos % 3 == 2 {self.cursor_pos} else {self.cursor_pos + 1},
            _ => {}
        }
    }

    fn select(&mut self){


        self.x_turn = !self.x_turn;
    }

    fn exit(&mut self) {
        self.exit = true;
    }

}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Tic-Tac-Toe ".bold());
        let instructions = Title::from(Line::from(vec![
            " Move Cursor ".into(),
            "<WASD>".blue().bold(),
            " Select ".into(),
            "<Space>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let turn: String;
        if self.x_turn {
            turn = "X".to_string();
        }else {
            turn = "O".to_string();
        }

        let turn_text = Text::from(vec![Line::from(vec![
            "Turn: ".into(),
            turn.yellow(),
        ])]);

        Paragraph::new(turn_text)
            .centered()
            .block(block)
            .render(area, buf);

        let body_text = Text::from(vec![Line::from(vec![
            self.cursor_pos.to_string().into(),
        ])]);

        let inner_title = Title::from("Game time");

        let inner_block = Block::default()
        .title(inner_title.alignment(Alignment::Center))

        .borders(Borders::ALL)
        .border_set(border::THICK);

        Paragraph::new(body_text)
            .centered()
            .block(inner_block)
            .render(area, buf);
    }
}