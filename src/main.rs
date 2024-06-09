use std::{io, rc::Rc, vec};

use ai::minimax;
use canvas::{Canvas, Context};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
    style::Color,
};

mod tui;
mod ai;

fn main() -> io::Result<()> {
    let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = tui::init()?;
    let app_result: Result<(), io::Error> = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    gamestate: [u8; 9],
    cursor_pos: u8,
    x_turn: bool,
    exit: bool,
    turn: u8,
    won: u8,
    ai_mode: bool,
    start: bool,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {


        //start screen to select game mode (1 / 2 player)
        terminal.draw(|frame| self.render_start_screen(frame))?;

        while !self.start {
            self.handle_events()?;

            if self.exit {
                return Ok(())
            }
        }

        self.x_turn = true; //start with X's turn
        while self.won == 0 {

            if self.exit {
                return Ok(());
            }

            terminal.draw(|frame: &mut Frame| self.render_frame(frame))?;

            self.handle_events()?;

            if self.ai_mode {
                self.run_ai_player();
            }

            self.check_state();

        }

        self.cursor_pos = 9;
        terminal.draw(|frame| self.render_frame(frame))?;

        while !self.exit {
            self.handle_events()?;
        }

        Ok(())
    }

    fn render_start_screen(&self, frame: &mut Frame) {
        let title = Title::from(" Terminal-Tac-Toe ".bold());
        let instructions = Title::from(Line::from(vec![
            " Singleplayer ".into(),
            "<1>".blue().bold(),
            " Multiplayer ".into(),
            "<2>".blue().bold(),
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

        let screen = Paragraph::new("Welcome To Terminal-Tac-Toe!")
        .block(block)
        .centered();

        frame.render_widget(screen, frame.size());

    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn check_state(&mut self) {
        //if on turns 1-3 no need to check win condition
        if self.turn < 4 {
            return
        }

        //checking the rows and columns
        for i in 0..3 {
            let n = i * 3;
            if self.gamestate[n] == self.gamestate[n+1] && self.gamestate[n] == self.gamestate[n+2] {
                self.won = self.gamestate[n];
                return
            }

            if self.gamestate[i] == self.gamestate[i+3] && self.gamestate[i] == self.gamestate[i+6] {
                self.won = self.gamestate[i];
                return
            }
        }

        //checking the diagonals
        if self.gamestate[0] == self.gamestate[4] && self.gamestate[0] == self.gamestate[8] {
            self.won = self.gamestate[0];
        }
        if self.gamestate[2] == self.gamestate[4] && self.gamestate[2] == self.gamestate[6] {
            self.won = self.gamestate[2];
        }

        //if all 9 turns are finished then end in a draw
        if self.turn > 8 {
            self.won = 3;
            return
        }

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
            KeyCode::Char('2') => self.start_game(true),
            KeyCode::Char('1') => self.start_game(false),
            _ => {}
        }
    }

    fn start_game(&mut self, ai_on: bool){
        if ai_on {
            self.ai_mode = true;
        }
        self.start = true;
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
            let player = if self.x_turn {1} else {2};
            self.play_move(self.cursor_pos as usize, player)
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
    
    //only plays as O for now
    fn run_ai_player(&mut self){
        if self.x_turn {
            return
        }

        let mut best_value = 10;
        let mut best_move = 9;

        for i in 0..9 {
            if self.gamestate[i] != 0 {continue;}

            let mut next_gamestate = self.gamestate.clone();
            next_gamestate[i] = 2;
            let next_value = minimax(next_gamestate, !self.x_turn, self.turn as i8);

            if next_value < best_value {
                best_move = i;
                best_value = next_value;
            }        
            
        }

        if best_move == 9 {
            return
        }

        self.play_move(best_move, 2);

    }

    fn play_move(&mut self, pos: usize, player: u8){
        self.gamestate[pos] = player;
        self.x_turn = !self.x_turn;
        self.turn += 1;
    }

}

impl Widget for &App {

    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Terminal-Tac-Toe ".bold());
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

        let mut turn_text = Text::from(vec![Line::from(vec![
            "Turn: ".into(),
            turn.yellow(),
        ])]);

        if self.won != 0 {

            let win_player = if self.won == 1 {String::from("X")} else if self.won == 2 {String::from("O")} else {String::from("Nobody")};

            turn_text  = Text::from(vec![Line::from(vec![
                win_player.yellow(),
                " wins!".yellow().into(),
            ])]);
        }

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
