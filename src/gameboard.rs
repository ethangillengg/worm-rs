use std::io::{self, stdout, Write};
use termion::cursor::{Down, Goto, Left, Up};

use termion::raw::RawTerminal;

#[derive(Debug)]
pub struct BorderTypes {
    pub horizontal: String,
    pub vertical: String,
    pub top_left: String,
    pub top_right: String,
    pub bottom_left: String,
    pub bottom_right: String,
}

impl Default for BorderTypes {
    fn default() -> Self {
        BorderTypes {
            horizontal: "─".into(),
            vertical: "│".into(),
            top_left: "╭".into(),
            top_right: "╮".into(),
            bottom_left: "╰".into(),
            bottom_right: "╯".into(),
        }
    }
}

#[derive(Debug, Default)]
pub struct GameBoard {
    pub border_types: BorderTypes,
}

impl GameBoard {
    pub fn draw(&self, stdout: &mut RawTerminal<io::Stdout>) {
        let term_size = termion::terminal_size().unwrap();
        let width: usize = (term_size.0 - 2).into();
        let height: usize = (term_size.1 - 3).into();
        let bt = &self.border_types;

        let next_line = format_args!("{}{}", Left(term_size.0), Down(1)).to_string();

        // Top Border
        write!(stdout, "{}", Goto(1, 1)).unwrap();
        writeln!(
            stdout,
            "{}{}{}",
            bt.top_left,
            bt.horizontal.repeat(width),
            bt.top_right
        )
        .unwrap();
        write!(stdout, "{}", Left(term_size.0)).unwrap();

        //Body
        for _ in 0..height {
            write!(stdout, "{}", bt.vertical).unwrap();
            write!(stdout, "{}", " ".repeat(width),).unwrap();
            write!(stdout, "{}", bt.vertical).unwrap();
            write!(stdout, "{}", next_line).unwrap();
        }

        // Bottom Border
        writeln!(
            stdout,
            "{}{}{}",
            bt.bottom_left,
            bt.horizontal.repeat(width),
            bt.bottom_right
        )
        .unwrap();
        write!(stdout, "{}", next_line).unwrap();

        write!(stdout, "{}", Goto(5, 5)).unwrap();
    }
}
