use std::io;

use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, style::Stylize};
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

    fn handle_events(&self) -> io::Result<()> {
        todo!()
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

        let mut turn: String;
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

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━━━━━ Tic-Tac-Toe ━━━━━━━━━━━━━━━━━┓",
            "┃                    Turn: X                    ┃",
            "┃                                               ┃",
            "┗━ Move Cursor <WASD> Select <Space> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        // note ratatui also has an assert_buffer_eq! macro that can be used to
        // compare buffers and display the differences in a more readable way
        assert_eq!(buf, expected);
    }

    #[test]
    fn handle_key_event() -> io::Result<()> {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into());
        assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into());
        assert_eq!(app.counter, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert_eq!(app.exit, true);

        Ok(())
    }
}

fn game() {
    println!("Welcome to Tic-Tac-Rust");
    println!("Press x to start...");
    
    let mut input = "\n".to_string();

    while input.chars().nth(0).unwrap() != 'x' {
        input.clear();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    }

    let mut gamestate = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut x_turn = true;

    let mut turns = 0;

    loop {
        //clear screen
        print!("\x1B[2J\x1B[1;1H");
        
        turns += 1;
        print_gamestate(&gamestate);
        if turns > 9 {
            break;
        }
        //check whose turn it is and then print the gamestate
        if x_turn {
            println!("X's turn to move, enter a number from 1 to 9 to play in that square");
        }else {
            println!("O's turn to move, enter a number from 1 to 9 to play in that square");
        }
        

        //take input 
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let c = input.chars().nth(0).unwrap();

        if c.is_ascii_digit() {
            let pos = c as usize - 0x31;
            gamestate[pos] = if x_turn {1} else {2};
        }else {
            println!("Enter a digit bro...");
        }

        x_turn = !x_turn;

    }
}

fn print_gamestate(gamestate : &[i32]){

    let mut buf: String = String::new();

    for i in 0..3 {
        for j in 0..6 {

            if j % 5 == 0 && j / 5 != 0 {
                println!("{}", buf);
                buf.clear();
            }else if j % 2 == 1 {
                buf.push('|');
            }else {
                let state = gamestate[i*3 + j/2];
    
                match state {
                    1 => buf.push('X'),
                    2 => buf.push('O'),
                    _ => buf.push(' '),
                }
            }
    
        }
        if i == 2 {
            break;
        }
        println!("------");
    }

}