use std::{io::{self}, rc::Rc, vec};

use canvas::{Canvas, Context};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
    style::Color,
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
        self.x_turn = true; //start with X's turn
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
        if self.gamestate[self.cursor_pos as usize] == 0 {
            self.gamestate[self.cursor_pos as usize] = if self.x_turn {1} else {2};
            self.x_turn = !self.x_turn;
        }
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

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(10),
                Constraint::Percentage(90),
            ])
            .split(area);

        Paragraph::new(turn_text)
            .centered()
            .block(block)
            .render(layout[0], buf);

        let grid_layout: Rc<[Rect]> = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ])
            .split(layout[1]);

        let mut inner_grid: Vec<Rc<[Rect]>> = Vec::new();

        for i in 0..3 {            
            let temp = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                ])
                .split(grid_layout[i]);

            inner_grid.push(temp);
        }

        for n in 0..9 {
            let i: usize = n / 3;
            let j: usize = n % 3;

            let mut circle = canvas::Circle::default();
            circle.radius = 90.0;

            let cross1 = canvas::Line::new(-150.0, -80.0, 150.0, 80.0, Color::White);
            let cross2 = canvas::Line::new(-150.0, 80.0, 150.0, -80.0, Color::White);

            if n as u8 == self.cursor_pos {
                Canvas::default()
                    .block(Block::default().borders(Borders::all()).blue())
                    .marker(symbols::Marker::Braille)
                    .paint(|ctx: &mut Context| {
                        if self.gamestate[n] == 1 {
                            ctx.draw(&cross1);
                            ctx.draw(&cross2);
                        }else if self.gamestate[n] == 2{
                            ctx.draw(&circle);
                        }
                    })
                    .x_bounds([-180.0, 180.0])
                    .y_bounds([-90.0, 90.0])
                    .render(inner_grid[i][j], buf);
      
            }else {
                Canvas::default()
                    .block(Block::default().borders(Borders::all()))
                    .marker(symbols::Marker::Braille)
                    .paint(|ctx: &mut Context| {
                        if self.gamestate[n] == 1 {
                            ctx.draw(&cross1);
                            ctx.draw(&cross2);
                        }else if self.gamestate[n] == 2{
                            ctx.draw(&circle);
                        }
                    })
                    .x_bounds([-180.0, 180.0])
                    .y_bounds([-90.0, 90.0])
                    .render(inner_grid[i][j], buf);

            }
        }
    }
}