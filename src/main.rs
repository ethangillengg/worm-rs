use gameboard::*;
use std::io::{stdin, stdout};
use std::{thread, time::Duration};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub mod gameboard;

fn main() {
    let fps = 5;
    let stdout = RawTerminal::from(stdout().into_raw_mode().unwrap());

    let board = GameBoard::default();

    loop {
        board.draw();

        handle_input();

        thread::sleep(Duration::from_millis(1000 / fps));
    }
}

fn handle_input() {
    let stdin = stdin();
    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => {
                print!("{}", termion::cursor::Goto(1, 1));
                print!("{}", termion::clear::All);
                print!("{}", termion::cursor::Goto(1, 1));
                print!("Thanks for playing!");
                std::process::exit(1);
            }
            _ => {}
        }
    }
}
