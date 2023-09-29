use gameboard::*;
use std::io::{stdin, stdout, Write};
use std::{thread, time::Duration};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub mod gameboard;

fn main() {
    let fps = 5;

    let mut stdout = RawTerminal::from(stdout().into_raw_mode().unwrap());

    let board = GameBoard::default();
    write!(stdout, "{}", termion::clear::All);

    loop {
        board.draw(&mut stdout);

        handle_input();

        thread::sleep(Duration::from_millis(1000 / fps));
    }
}

fn handle_input() {
    let stdin = stdin();
    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => panic!("EXITED"),
            _ => {}
        }
    }
}
